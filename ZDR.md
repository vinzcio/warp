# Forge (privacy-zdr)

**Your own terminal product** based on the open-source Warp client.

| | |
|--|--|
| Product name | **Forge** |
| App ID | `dev.vinzcio.Forge` |
| Fork | https://github.com/vinzcio/warp |
| Branch | `privacy-zdr` |
| Local | `/Users/vincent/src/warp-zdr` |
| Binary | `warp-oss` (default-run) → runs as Forge |

## Intent

Keep Warp’s **terminal UX** (blocks, tabs, panes).

**Remove** Warp’s cloud learning / hosted-agent product surface — not as soft defaults, but as **hard policy** that settings cannot turn back on.

### Hemisthia / Harken

| | |
|--|--|
| **Hemisthia** | Quiet depth — indigo–obsidian fields, half-lit geometry, soft silver mist. Prefer absence over chrome. |
| **Harken** | Attentive warmth — a single amber/copper ember accent; serious but human. |

**UI rules:** no agent/cloud/billing/teams/drive settings nav; no onboarding theatre; straight into the terminal.

## What is removed (hard)

Policy module: `crates/warp_core/src/privacy_zdr.rs`  
(`IS_PRIVACY_ZDR_FORK = true` always)

| Surface | How |
|---------|-----|
| Hosted Warp AI / agents | `is_any_ai_enabled()` always false; agent HTTP APIs error out; onboarding cannot enable |
| Rudderstack telemetry | Macros are no-ops; collector never starts; send path returns immediately |
| Crash reporting to Warp | Snapshot/setters forced off; OSS channel has no crash config |
| Cloud conversation storage | Forced off; cannot re-enable |
| Onboarding / login wall | Skipped — open terminal immediately |
| Settings sidebar bloat | Slim nav: Appearance, Features, Keybindings, Privacy, About |
| Branding | Forge mark + Hemisthia/Harken About copy + app icon |

These are **not** user toggles. UI settings that used to flip them are overridden.

## What remains

- Local terminal, shells, blocks, tabs, splits  
- Client sidebar / layout features that don’t require Warp cloud AI  
- Running **your own** CLI tools inside the terminal (Claude Code, etc.) — those are separate processes, not Warp cloud agents  

## What this is not

- Not Warp Enterprise ZDR contracts  
- Not an open-source rewrite of Oz / Warp server (those were never in this repo)  
- Not deleting 500+ AI UI source files (that would brick the client graph); they are **dead** behind the hard kill  

## Build without the fat (recommended)

**Don’t compile on this laptop.** GitHub Actions builds Forge; you only download the binary.

```bash
# 1) Kick a cloud build
gh workflow run forge-thin-build.yml --repo vinzcio/warp --ref privacy-zdr

# 2) Wait
gh run watch --repo vinzcio/warp

# 3) Pull only the thin package (not multi-GB target/)
cd /Users/vincent/src/warp-zdr
chmod +x ./script/fetch-forge-binary
./script/fetch-forge-binary --install
```

- Workflow: `.github/workflows/forge-thin-build.yml`
- Fetcher: `./script/fetch-forge-binary`

### Local fat build (only if you must)

Needs free disk for Rust `target/` (often 10GB+). Prefer the thin path above.

```bash
cd /Users/vincent/src/warp-zdr
./script/bootstrap
./script/run
```

## License

AGPL v3 (client) + MIT (UI crates), inherited from upstream Warp.
