-- Continuum-only SSO metadata for OIDC provisioning.
-- Kept as an additive Diesel migration so upstream Vaultwarden binaries can
-- ignore the nullable columns safely.

ALTER TABLE users
ADD COLUMN IF NOT EXISTS continuum_sso_subject TEXT NULL;

ALTER TABLE users
ADD COLUMN IF NOT EXISTS continuum_provisioned_at TIMESTAMP NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_continuum_sso_subject
ON users(continuum_sso_subject)
WHERE continuum_sso_subject IS NOT NULL;
