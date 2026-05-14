# Continuum Vault Fork Notes

This repository is the Continuum-maintained Vaultwarden fork.

## Upstream Baseline

- Upstream: `https://github.com/dani-garcia/vaultwarden`
- Baseline tag: `1.32.5`
- Baseline commit: `cdfdc6ff`

## License Verification

Vaultwarden is licensed as AGPLv3, not MIT.

Verification command:

```bash
$ head -20 Cargo.toml | rg -n "license"
11:license = "AGPL-3.0-only"
```

All Continuum-specific Rust code must live under `src/continuum/`.
The only allowed upstream source edit for C1 is a single `mod continuum;`
declaration in `src/main.rs`.
