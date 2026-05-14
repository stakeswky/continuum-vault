# Continuum SSO Subject Migration

This migration adds nullable Continuum-only SSO metadata to Vaultwarden's
`users` table without editing any upstream migration.

The C1 plan named `migrations/postgresql/continuum/0001_c1_metadata_columns.sql`,
but Vaultwarden embeds Postgres migrations through Diesel's standard
`migrations/postgresql/<migration-name>/up.sql` and `down.sql` layout. Keeping
this migration in the upstream layout ensures the compiled Vaultwarden binary
actually discovers and runs it.

The migration is additive and reversible:

- `continuum_sso_subject TEXT NULL`
- `continuum_provisioned_at TIMESTAMP NULL`
- partial unique index on non-null `continuum_sso_subject`

## Policy

Continuum migrations must not edit upstream Vaultwarden migration directories.
Each extension should be added as a new timestamped migration directory so
monthly upstream rebases can separate Continuum changes from upstream schema
history.

All Continuum-specific columns must be nullable or otherwise backwards
compatible with an upstream-only Vaultwarden binary. If a future migration
requires stricter constraints, add them only after the bridge has backfilled
existing data and record the compatibility window in this README.
