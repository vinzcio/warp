//! # privacy-zdr hard fork policy
//!
//! This fork is a **private terminal**, not a Warp AI product.
//!
//! Hosted Warp agents, Rudderstack telemetry, crash reporting to Warp, and
//! cloud conversation storage are **removed at the policy layer** — not merely
//! defaulted off. Settings toggles and feature flags cannot re-enable them.
//!
//! Product identity for this fork: **Forge** (`dev.vinzcio.Forge`).

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
