DROP INDEX IF EXISTS idx_users_continuum_sso_subject;

ALTER TABLE users
DROP COLUMN IF EXISTS continuum_provisioned_at;

ALTER TABLE users
DROP COLUMN IF EXISTS continuum_sso_subject;
