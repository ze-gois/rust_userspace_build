# userspace_build

[![crates.io](https://img.shields.io/crates/v/userspace_build.svg)](https://crates.io/crates/userspace_build)
[![docs.rs](https://docs.rs/userspace_build/badge.svg)](https://docs.rs/userspace_build)

Build-side companion to `userspace` in the [userspace.party](https://userspace.party) ecosystem.

## Role

`userspace_build` exists for code that must participate in building the freestanding userspace stack while remaining versioned with the rest of the family. Higher-level crates consume it as a build dependency, commonly with the `with_std` feature enabled.

At present it shares much of the low-level target, file, memory, trait and startup machinery used by `userspace`. That overlap is intentional but still evolving: the long-term boundary is to keep build-time responsibilities here and runtime responsibilities in `userspace`.

## Use as a build dependency

```toml
[build-dependencies]
userspace_build = { version = "0.2", features = ["with_std"] }
```

Available feature modes include `no_std` and `with_std`.

## Ecosystem

- Ecosystem: https://userspace.party
- Crate homepage: https://userspace.party/userspace_build
- API documentation: https://docs.rs/userspace_build
- crates.io: https://crates.io/crates/userspace_build
- Source: https://github.com/ze-gois/rust_userspace_build
- Workspace hub: https://github.com/ze-gois/rust_userspace_hub

The crate is independently clonable but participates in the coordinated hub release and version line.

## Status

Experimental. The split between runtime and build-time code is under active refinement.

## License

See [LICENSE](LICENSE).
