<!--
Contract reference template — use for pages documenting one contract crate's
public interface (e.g. "Shade Contract Reference," "Escrow Contract Reference").
Delete this comment block before publishing the page.
See docs/contributing/documentation-style-guide.md for tone/format rules.
-->

# <Contract name> reference

One or two sentences: what this contract does and where its crate lives (e.g. `contracts/shade/`).

## Contract interface

The trait defining this contract's public interface: [`<path>`](../../../contracts/shade/src/interface.rs).

## Functions

Group functions by feature area, matching the grouping in the source. For each function, copy the signature verbatim from the source — do not retype or simplify it.

### <Feature area, e.g. "Merchants">

#### `function_name`

```rust
fn function_name(env: Env, merchant: Address, amount: i128) -> u64;
```

- **Auth:** who must authorize this call (e.g. "requires `merchant.require_auth()`").
- **Parameters:** brief description of any non-obvious parameter.
- **Returns:** what the return value means.
- **Panics:** conditions that cause a panic.
- **Events:** events emitted, if any, with a link to [`contracts/shade/src/events.rs`](../../../contracts/shade/src/events.rs).

## Types

| Type | Defined in | Purpose |
|---|---|---|
| `ExampleType` | [`contracts/shade/src/types.rs`](../../../contracts/shade/src/types.rs) | One-line purpose |

## Storage keys

| Key | Enum | Purpose |
|---|---|---|
| `DataKey::Example` | [`DataKey`](../../../contracts/shade/src/types.rs) | One-line purpose |

## Errors

| Error | Defined in | Cause |
|---|---|---|
| `Error::Example` | [`contracts/shade/src/errors.rs`](../../../contracts/shade/src/errors.rs) | When this is raised |

## Related pages

- [Concept page(s) explaining the "why" behind this contract's design](#)
