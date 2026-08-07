# lava-prelude

Single-import facade over the [lava](https://github.com/pleme-io) typed surface.

```rust
use lava_prelude::*;
```

That one line pulls every typed primitive across the suite, collapsing a
nine-crate dependency wall into one import. Pattern: `std::prelude`.

## What it re-exports

| Source crate | Exports |
|---|---|
| `lava-core` | `Architecture`, `Resource`, `ResourceRef`, `Value`, `Multiplicity`, `ProviderRef`, `Stack`, `BackendRef`, `Synthesizer`, `TerraformJson`, `MagmaPlan`, `RenderError` |
| `lava-arch` | `Builder`, `Library`, `ArchitectureSpec`, `ArchError` |
| `lava-contracts` | `ArchitectureResult`, `NetworkResult`, `IamResult`, `ClusterResult`, `SecretsResult`, `SecretRef`, `SecretBackendKind`, `DnsResult`, `DnsRecord`, `StorageResult`, `LoadBalancerResult`, `ObservabilityResult` |
| `lava-stack` | `Stack as StackInstance`, `StackTag`, `StackConfig`, `StackError` |
| `lava-types` | `Type`, `TypeError`, `MatchKind` |

## Usage

```toml
[dependencies]
lava-prelude = "0.1"
```

## When not to use it

A library that needs one or two lava types should depend on that crate
directly — the prelude exists for consumers who want the whole surface and
would otherwise hand-maintain a wall of dependencies.

## License

MIT
