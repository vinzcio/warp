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

Keep Warp’s **terminal UX** (blocks, tabs, panes, sidebar chrome).

**Remove** Warp’s cloud learning / hosted-agent product surface — not as soft defaults, but as **hard policy** that settings cannot turn back on.

## What is removed (hard)

Policy module: `crates/warp_core/src/privacy_zdr.rs`  
(`IS_PRIVACY_ZDR_FORK = true` always)

| Surface | How |
|---------|-----|
| Hosted Warp AI / agents | `is_any_ai_enabled()` always false; agent HTTP APIs error out; onboarding cannot enable |
| Rudderstack telemetry | Macros are no-ops; collector never starts; send path returns immediately |
| Crash reporting to Warp | Snapshot/setters forced off; OSS channel has no crash config |
| Cloud conversation storage | Forced off; cannot re-enable |

These are **not** user toggles. UI settings that used to flip them are overridden.

## What remains

- Local terminal, shells, blocks, tabs, splits  
- Client sidebar / layout features that don’t require Warp cloud AI  
- Running **your own** CLI tools inside the terminal (Claude Code, etc.) — those are separate processes, not Warp cloud agents  

## What this is not

- Not Warp Enterprise ZDR contracts  
- Not an open-source rewrite of Oz / Warp server (those were never in this repo)  
- Not deleting 500+ AI UI source files (that would brick the client graph); they are **dead** behind the hard kill  

## Build

Needs free disk (~20–40 GB for Rust `target/`). Then:

```bash
cd /Users/vincent/src/warp-zdr
git checkout privacy-zdr
./script/bootstrap
./script/run          # or: cargo run -p warp --bin warp-oss
```

## License

AGPL v3 (client) + MIT (UI crates), inherited from upstream Warp.
