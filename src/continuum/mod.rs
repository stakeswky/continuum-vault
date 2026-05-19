//! Continuum patches. ALL Continuum-specific code must live under this module.
//! Touching upstream files outside `src/continuum/` breaks rebase CI.

use ctor::ctor;

pub mod custom_item_validators;
pub mod startup_guard;
pub mod webhooks;

#[ctor]
fn _continuum_init() {
    startup_guard::enforce();
    register_webhooks();
}

pub fn register_webhooks() {
    let _validator = custom_item_validators::validate_continuum_fields;

    let Ok(bridge_url) = std::env::var("CONTINUUM_BRIDGE_URL") else {
        return;
    };
    let Ok(secret) = std::env::var("CONTINUUM_WEBHOOK_SECRET") else {
        return;
    };

    let target_url = format!(
        "{}/internal/vault-events",
        bridge_url.trim_end_matches('/')
    );

    let spawn_result = std::thread::Builder::new()
        .name("continuum-webhook-poller".to_string())
        .spawn(move || {
            match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime.block_on(webhooks::poll_and_emit(
                    webhooks::WebhookEmitter::new(target_url, secret),
                )),
                Err(error) => eprintln!("continuum webhook runtime failed: {error}"),
            }
        });

    if let Err(error) = spawn_result {
        eprintln!("continuum webhook poller failed to start: {error}");
    }
}
