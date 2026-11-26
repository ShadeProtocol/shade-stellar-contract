# Docs Tooling

## Previewing the site

The docs are built with [mdBook](https://rust-lang.github.io/mdBook/) and [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid).

```bash
cargo install mdbook mdbook-mermaid
mdbook-mermaid install docs   # once, writes mermaid.min.js and mermaid-init.js
mdbook serve docs --open      # or: make docs-serve
```

`make docs` builds the book into `target/book/` and the rustdoc API reference (`cargo doc --workspace --no-deps`) into `target/book/api/`.

When you add a page, also add it to [`docs/SUMMARY.md`](../SUMMARY.md) or it will not appear in the site navigation.

## Quality checks

The [Docs Quality workflow](../../.github/workflows/docs-quality.yml) checks event
publisher coverage and error codes when reference pages, contract event/error
sources, or the checker change. It also runs the checker's regression tests.

Run the reference check with Python 3 (no packages or Rust build required):

```bash
python3 scripts/check_reference_docs.py
python3 -m unittest discover -s scripts -p 'test_check_reference_docs.py' -v
```

You can also use `bash scripts/check-reference-docs.sh` from the repository root.
The Python entry point works from any directory.

For each `contracts/*/src/events.rs`, add every `publish_*_event` helper as the
first cell of a publisher-index row in [events.md](../reference/events.md), under
a level-two heading containing the backticked source path. For each error enum,
use the matching source-path heading in [errors.md](../reference/errors.md), then
a level-three heading with the backticked enum name. Error rows start with the
backticked variant name and its decimal code. This keeps identical names in
different contracts separate. Comments and fenced examples do not count as
reference entries.

The checker supports unit error variants with integer literals (including hex,
binary, octal, underscores and `u32` suffixes) or implicit successor values.
Unsupported discriminant expressions fail with a diagnostic; extend the parser
and its tests before adopting a new source format. The check verifies coverage
and codes, not the accuracy of payload descriptions or emission semantics.

Link checking remains a local [lychee](https://github.com/lycheeverse/lychee) check
using `lychee.toml`; its previously disabled CI job is not enabled by this change.

Run it from the repository root:

```bash
cargo install lychee

lychee --config lychee.toml docs README.md CONTRIBUTING.md
```

Run both reference coverage and link checking with `make docs-check`. Python 3
and lychee must be available; missing references fail before the link check.

To allowlist a URL that fails the link checker (for example a site that blocks bots), add it to `exclude` in `lychee.toml`.
