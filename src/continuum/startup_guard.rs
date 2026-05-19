//! Spec §6.1 (Rust side) + §6.4 (3). Refuses to start the vault binary when
//! ROCKET_ENV != "development" and any tracked secret still equals its known
//! dev default. Called from the `#[ctor]` in `src/continuum/mod.rs` so it
//! runs before Rocket's main() has a chance to bind a port.
//!
//! Failure mode: writes a diagnostic to stderr and calls std::process::exit(78)
//! (sysexits EX_CONFIG). Bubble-up via Result is not viable from a #[ctor]
//! constructor, which is the earliest point we can intercept.

const DEV_DEFAULTS: &[(&str, &str)] = &[
    ("SSO_CLIENT_SECRET", "dev-sso-secret"),
    ("ADMIN_TOKEN", "dev-admin-token"),
];

const EX_CONFIG: i32 = 78;

/// Result of evaluating the env. `Ok(())` means start. `Err(...)` lists the
/// env vars that are missing / empty / equal to a dev default.
///
/// Pure function: takes a `getenv` closure rather than calling
/// `std::env::var` directly so unit tests can inject env values without
/// mutating process env (the vault repo forbids `unsafe_code`, and Rust
/// edition 2024 made `std::env::set_var` an unsafe call).
pub fn evaluate<F>(getenv: F) -> Result<(), Vec<String>>
where
    F: Fn(&str) -> Option<String>,
{
    let rocket_env = getenv("ROCKET_ENV").unwrap_or_else(|| "development".to_string());
    if rocket_env == "development" {
        return Ok(());
    }

    let mut offenders: Vec<String> = Vec::new();
    for (env_name, dev_default) in DEV_DEFAULTS {
        match getenv(env_name) {
            Some(value) if value.is_empty() || value == *dev_default => {
                offenders.push((*env_name).to_string());
            }
            None => {
                offenders.push(format!("{} (unset)", env_name));
            }
            _ => {}
        }
    }

    // §6.4 (3) defence-in-depth: also require DISABLE_ADMIN_TOKEN=true in non-dev.
    let admin_disabled = getenv("DISABLE_ADMIN_TOKEN")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if !admin_disabled {
        offenders.push("DISABLE_ADMIN_TOKEN (must be \"true\" in non-development)".to_string());
    }

    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders)
    }
}

pub fn enforce() {
    if let Err(offenders) = evaluate(|name| std::env::var(name).ok()) {
        let rocket_env = std::env::var("ROCKET_ENV").unwrap_or_else(|_| "development".to_string());
        eprintln!(
            "continuum startup_guard: refusing to start under ROCKET_ENV={:?}; \
             these env vars are missing, empty, or still equal their dev default: {:?}. \
             Run scripts/gen_secrets.sh in the continuum repo to mint replacements.",
            rocket_env, offenders
        );
        std::process::exit(EX_CONFIG);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn env_from(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        move |name: &str| map.get(name).cloned()
    }

    #[test]
    fn development_skips_all_checks() {
        let getenv = env_from(&[("ROCKET_ENV", "development")]);
        assert!(evaluate(getenv).is_ok());
    }

    #[test]
    fn missing_rocket_env_treated_as_development() {
        let getenv = env_from(&[]);
        assert!(evaluate(getenv).is_ok());
    }

    #[test]
    fn production_rejects_dev_default_sso_secret() {
        let getenv = env_from(&[
            ("ROCKET_ENV", "production"),
            ("SSO_CLIENT_SECRET", "dev-sso-secret"),
            ("ADMIN_TOKEN", "real-admin-token-please-change"),
            ("DISABLE_ADMIN_TOKEN", "true"),
        ]);
        let err = evaluate(getenv).expect_err("must reject dev default");
        assert!(err.iter().any(|o| o.contains("SSO_CLIENT_SECRET")));
    }

    #[test]
    fn production_rejects_empty_admin_token() {
        let getenv = env_from(&[
            ("ROCKET_ENV", "production"),
            ("SSO_CLIENT_SECRET", "real-sso-secret-12345"),
            ("ADMIN_TOKEN", ""),
            ("DISABLE_ADMIN_TOKEN", "true"),
        ]);
        let err = evaluate(getenv).expect_err("must reject empty value");
        assert!(err.iter().any(|o| o.contains("ADMIN_TOKEN")));
    }

    #[test]
    fn production_rejects_unset_secrets() {
        let getenv = env_from(&[("ROCKET_ENV", "production")]);
        let err = evaluate(getenv).expect_err("must reject unset");
        assert!(err.iter().any(|o| o.contains("SSO_CLIENT_SECRET")));
        assert!(err.iter().any(|o| o.contains("ADMIN_TOKEN")));
        assert!(err.iter().any(|o| o.contains("DISABLE_ADMIN_TOKEN")));
    }

    #[test]
    fn production_requires_disable_admin_token_true() {
        let getenv = env_from(&[
            ("ROCKET_ENV", "production"),
            ("SSO_CLIENT_SECRET", "real-sso"),
            ("ADMIN_TOKEN", "real-admin"),
            // DISABLE_ADMIN_TOKEN missing
        ]);
        let err = evaluate(getenv).expect_err("must require DISABLE_ADMIN_TOKEN");
        assert!(err.iter().any(|o| o.contains("DISABLE_ADMIN_TOKEN")));
    }

    #[test]
    fn production_accepts_real_secrets_and_disabled_admin() {
        let getenv = env_from(&[
            ("ROCKET_ENV", "production"),
            ("SSO_CLIENT_SECRET", "real-sso-secret-12345"),
            ("ADMIN_TOKEN", "real-admin-token-67890"),
            ("DISABLE_ADMIN_TOKEN", "true"),
        ]);
        assert!(evaluate(getenv).is_ok());
    }

    #[test]
    fn disable_admin_token_is_case_insensitive() {
        let getenv = env_from(&[
            ("ROCKET_ENV", "staging"),
            ("SSO_CLIENT_SECRET", "real-sso"),
            ("ADMIN_TOKEN", "real-admin"),
            ("DISABLE_ADMIN_TOKEN", "TRUE"),
        ]);
        assert!(evaluate(getenv).is_ok());
    }
}
