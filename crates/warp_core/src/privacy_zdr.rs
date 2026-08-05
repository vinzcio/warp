//! # privacy-zdr hard fork policy
//!
//! This fork is a **private terminal**, not a Warp AI product.
//!
//! Hosted Warp agents, Rudderstack telemetry, crash reporting to Warp, and
//! cloud conversation storage are **removed at the policy layer** — not merely
//! defaulted off. Settings toggles and feature flags cannot re-enable them.
//!
//! Product identity: **Forge** (`dev.vinzcio.Forge`), styled under the
//! **Hemisthia / Harken** principles (quiet depth + attentive warmth).

/// Compile-time constant: this build is the privacy-zdr fork.
pub const IS_PRIVACY_ZDR_FORK: bool = true;

/// Display name of this product (not "Warp").
pub const PRODUCT_NAME: &str = "Forge";

/// Reverse-DNS style application name for AppId / bundle.
pub const APPLICATION_NAME: &str = "Forge";

/// Bundle / app identifier organization segment.
pub const ORGANIZATION: &str = "vinzcio";

/// Bundle qualifier (Java-style).
pub const QUALIFIER: &str = "dev";

/// Log file base name.
pub const LOGFILE_NAME: &str = "forge.log";

/// macOS / deep-link URL scheme.
pub const URL_SCHEME: &str = "forge";

/// Short about-line for About page / copyright.
pub const COPYRIGHT_LINE: &str = "Forge · Hemisthia / Harken";

/// Tagline for About and empty states.
pub const TAGLINE: &str = "A private terminal. Quiet depth. Attentive warmth.";

// ── Hemisthia / Harken design principles ────────────────────────────────────
//
// **Hemisthia** — hemispheric calm: deep void, indigo–obsidian fields, soft
// silver mist text, half-lit geometry. Prefer absence over chrome. Marketing
// surfaces, agent upsells, and cloud commerce do not belong in the product.
//
// **Harken** — to listen: a single warm amber/copper accent (ember), human and
// serious. Controls that earn their place stay; everything else is removed.
//
// Settings nav is intentionally short: Appearance, Features, Keybindings,
// Privacy, About. Onboarding is skipped — straight into the terminal.

/// Hosted Warp AI / agent features are permanently unavailable.
#[inline(always)]
pub fn warp_cloud_ai_removed() -> bool {
    IS_PRIVACY_ZDR_FORK
}

/// Telemetry (Rudderstack / "help improve") is permanently unavailable.
#[inline(always)]
pub fn telemetry_removed() -> bool {
    IS_PRIVACY_ZDR_FORK
}

/// Crash reporting to Warp/Sentry backends is permanently unavailable.
#[inline(always)]
pub fn crash_reporting_removed() -> bool {
    IS_PRIVACY_ZDR_FORK
}

/// Cloud conversation storage is permanently unavailable.
#[inline(always)]
pub fn cloud_conversation_storage_removed() -> bool {
    IS_PRIVACY_ZDR_FORK
}

/// Skip Warp onboarding / login theatre and open a terminal immediately.
#[inline(always)]
pub fn skip_onboarding() -> bool {
    IS_PRIVACY_ZDR_FORK
}

/// Use the slim Hemisthia settings sidebar (no Agents/Cloud/Billing/Teams…).
#[inline(always)]
pub fn slim_settings_nav() -> bool {
    IS_PRIVACY_ZDR_FORK
}
