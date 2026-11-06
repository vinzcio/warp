// On Windows, we don't want to display a console window when the application is running in release
// builds. See https://doc.rust-lang.org/reference/runtime.html#the-windows_subsystem-attribute.
#![cfg_attr(feature = "release_bundle", windows_subsystem = "windows")]

use anyhow::Result;
use warp_core::channel::{Channel, ChannelConfig, ChannelState, OzConfig, WarpServerConfig};
use warp_core::{AppId, privacy_zdr};

// Forge: privacy-zdr private terminal (Warp client UX, cloud AI/telemetry removed).
fn main() -> Result<()> {
    let mut state = ChannelState::new(
        Channel::Oss,
        ChannelConfig {
            app_id: AppId::new(
                privacy_zdr::QUALIFIER,
                privacy_zdr::ORGANIZATION,
                privacy_zdr::APPLICATION_NAME,
            ),
            logfile_name: privacy_zdr::LOGFILE_NAME.into(),
            // Keep server configs for non-AI local features that still speak HTTP;
            // AI/telemetry/crash paths are hard-killed in privacy_zdr.
            server_config: WarpServerConfig::production(),
            oz_config: OzConfig::production(),
            telemetry_config: None,
            crash_reporting_config: None,
            autoupdate_config: None,
            mcp_static_config: None,
        },
    );
    if cfg!(debug_assertions) {
        state = state.with_additional_features(warp_core::features::DEBUG_FLAGS);
    }
    ChannelState::set(state);

    log::info!(
        "{} privacy-zdr: cloud AI removed={}, telemetry removed={}",
        privacy_zdr::PRODUCT_NAME,
        privacy_zdr::warp_cloud_ai_removed(),
        privacy_zdr::telemetry_removed(),
    );

    warp::run()
}

// If we're not using an external plist, embed the following as the Info.plist.
#[cfg(all(not(feature = "extern_plist"), target_os = "macos"))]
embed_plist::embed_info_plist_bytes!(r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleDisplayName</key>
    <string>Forge</string>
    <key>CFBundleExecutable</key>
    <string>warp-oss</string>
    <key>CFBundleIdentifier</key>
    <string>dev.vinzcio.Forge</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Forge</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.developer-tools</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>UIDesignRequiresCompatibility</key>
    <true/>
    <key>CFBundleURLTypes</key>
    <array><dict><key>CFBundleURLName</key><string>Forge</string><key>CFBundleURLSchemes</key><array><string>forge</string></array></dict></array>
    <key>NSHumanReadableCopyright</key>
    <string>© Forge privacy-zdr fork (AGPL client). Based on Warp OSS.</string>
    </dict>
    </plist>
"#.as_bytes());
