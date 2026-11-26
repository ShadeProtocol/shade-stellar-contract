"""Regression tests mutate isolated copies of the reference/source files."""
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch
from contextlib import redirect_stderr
import io

from check_reference_docs import ROOT, check, error_variants, source_text, main


class ReferenceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for pattern in ('contracts/*/src/events.rs', 'contracts/*/src/errors.rs',
                        'docs/reference/events.md', 'docs/reference/errors.md'):
            for source in ROOT.glob(pattern):
                target = self.root / source.relative_to(ROOT)
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source, target)

    def edit(self, path, old, new):
        target = self.root / path
        content = target.read_text(encoding='utf-8')
        self.assertIn(old, content)
        target.write_text(content.replace(old, new), encoding='utf-8')

    def errors(self):
        return '\n'.join(check(self.root)[0])

    def test_current_references_pass(self):
        self.assertEqual(check(self.root)[0], [])

    def test_added_event_fails(self):
        path = self.root / 'contracts/shade/src/events.rs'
        with path.open('a') as file:
            file.write('\npub fn publish_new_event() {}\n')
        self.assertIn('missing event helper publish_new_event', self.errors())

    def test_added_error_fails(self):
        self.edit('contracts/shade/src/errors.rs', 'NotAuthorized = 1,',
                  'NotAuthorized = 1, NewError = 99,')
        self.assertIn('ContractError::NewError: missing error entry', self.errors())

    def test_wrong_code_fails(self):
        self.edit('docs/reference/errors.md', '| `NotAuthorized` | 1 |',
                  '| `NotAuthorized` | 99 |')
        self.assertIn('expected code 1, documented 99', self.errors())

    def test_same_name_in_other_contract_does_not_mask_missing_error(self):
        self.edit('docs/reference/errors.md', '| `NotAuthorized` | 3 |',
                  '| `Removed` | 3 |')
        self.assertIn('contracts/account/src/errors.rs::ContractError::NotAuthorized: missing',
                      self.errors())

    def test_same_event_in_other_contract_does_not_mask_missing_helper(self):
        self.edit('docs/reference/events.md',
                  '## `account` publishers (`contracts/account/src/events.rs`)',
                  '## Removed account section')
        self.assertIn('contracts/account/src/events.rs: missing event helper publish_token_added_event',
                      self.errors())

    def test_commented_documentation_does_not_count(self):
        self.edit('docs/reference/errors.md', '| `NotAuthorized` | 1 |',
                  '<!-- | `NotAuthorized` | 1 | -->\n| `Removed` | 1 |')
        self.assertIn('ContractError::NotAuthorized: missing', self.errors())

    def test_commented_source_does_not_add_requirements(self):
        path = self.root / 'contracts/shade/src/events.rs'
        with path.open('a') as file:
            file.write('\n// fn publish_fake_event() {}\n/* fn publish_fake_event() {} */\n')
        self.assertEqual(check(self.root)[0], [])

    def test_new_contract_is_discovered(self):
        path = self.root / 'contracts/new/src'
        path.mkdir(parents=True)
        (path / 'events.rs').write_text('pub fn publish_new_event() {}')
        (path / 'errors.rs').write_text('pub enum NewError { Bad = 1 }')
        errors = self.errors()
        self.assertIn('contracts/new/src/events.rs', errors)
        self.assertIn('NewError::Bad', errors)

    def test_unsupported_discriminant_fails_closed(self):
        self.edit('contracts/shade/src/errors.rs', 'NotAuthorized = 1,',
                  'NotAuthorized = 1 << 2,')
        self.assertIn('unsupported error variant', self.errors())

    def test_duplicate_error_rows_fail(self):
        self.edit('docs/reference/errors.md', '| `NotAuthorized` | 1 |',
                  '| `NotAuthorized` | 1 |\n| `NotAuthorized` | 1 |')
        self.assertIn('expected code 1, documented 1, 1', self.errors())

    def test_integer_literals_and_implicit_values(self):
        self.assertEqual(error_variants('First = 0x10, Second, Third = 1_000u32,'),
                         [('First', 16), ('Second', 17), ('Third', 1000)])

    def test_nested_comments(self):
        self.assertNotIn('fake', source_text('/* outer /* fake */ comment */ real'))
        self.assertIn('real', source_text('/* outer /* fake */ comment */ real'))
        self.assertIn('real', source_text('/* // comment */ real'))

    def test_fenced_example_does_not_count(self):
        self.edit('docs/reference/errors.md', '| `NotAuthorized` | 1 |',
                  '```markdown\n| `NotAuthorized` | 1 |\n```\n| `Removed` | 1 |')
        self.assertIn('ContractError::NotAuthorized: missing', self.errors())

    def test_new_enum_cannot_reuse_another_enums_rows(self):
        path = self.root / 'contracts/shade/src/errors.rs'
        with path.open('a') as file:
            file.write('\npub enum OtherError { NotAuthorized = 1 }\n')
        self.assertIn('OtherError::NotAuthorized: missing', self.errors())

    def test_unsupported_enum_body_fails_closed(self):
        path = self.root / 'contracts/shade/src/errors.rs'
        with path.open('a') as file:
            file.write('\npub enum OtherError { Bad = { 1 } }\n')
        self.assertIn('unsupported error enum body', self.errors())

    def test_cli_returns_failure_with_actionable_diagnostic(self):
        self.edit('docs/reference/errors.md', '| `NotAuthorized` | 1 |',
                  '| `NotAuthorized` | 99 |')
        output = io.StringIO()
        with patch('check_reference_docs.ROOT', self.root), redirect_stderr(output):
            self.assertEqual(main(), 1)
        self.assertIn('expected code 1, documented 99', output.getvalue())

    def test_missing_page_returns_failure(self):
        (self.root / 'docs/reference/errors.md').unlink()
        output = io.StringIO()
        with patch('check_reference_docs.ROOT', self.root), redirect_stderr(output):
            self.assertEqual(main(), 1)
        self.assertIn('Reference check failed:', output.getvalue())


if __name__ == '__main__':
    unittest.main()
