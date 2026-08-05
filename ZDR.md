# privacy-zdr — private Warp terminal (sidebar, no cloud agents)

**Goal:** Warp’s **client UX** (blocks, tabs, **sidebar**, modern terminal) as *your* app — **without** Warp learning on your work and **without** relying on their hosted agents.

This is **not** “use Ghostty instead.” The product target is Warp’s terminal UI, privatized.

| | |
|--|--|
| Fork | https://github.com/vinzcio/warp |
| Branch | `privacy-zdr` |
| Local | `/Users/vincent/src/warp-zdr` |

## What stays (why Warp)

- Terminal UI: blocks, tabs, panes, command editor feel  
- **Sidebar / agent-management-style chrome** (layout features in the client)  
- Modern terminal features you wanted that plain Ghostty doesn’t ship the same way  

## What we turn off by default (privacy)

| Setting | Default here |
|---------|----------------|
| Telemetry / “Help improve Warp” | **off** |
| Crash reporting | **off** |
| Cloud conversation storage | **off** |
| **All Warp AI** (`is_any_ai_enabled`) | **off** |
| Active AI / suggestions / code suggestions / NL ghost text | **off** |
| Voice AI | **off** |
| “Use Agent” footer on commands | **off** |
| Local→cloud handoff | **force disabled** |

You keep a nice terminal. You do **not** send console/work to Warp for model improvement or hosted agents by default.

## What forking cannot invent

- Warp’s **server**, **Oz**, and **hosted agent harness** are **not** open source.  
- We are **not** rebuilding their cloud agent. You said you don’t need it.  
- “ZDR” here means: **our client doesn’t opt you into their collection/AI cloud** — not a legal ZDR contract with Warp Enterprise.

Optional later: run *your* CLI agents inside this terminal (Claude Code, Codex, etc.) with *your* keys. That’s separate from Warp’s cloud.

## Build

Disk was tight (~14 GB free). A full Rust build often needs **20–40 GB**. Free space, then:

```bash
cd /Users/vincent/src/warp-zdr
git checkout privacy-zdr
./script/bootstrap
./script/run
```

## Remotes

```text
origin    https://github.com/vinzcio/warp.git
upstream  https://github.com/warpdotdev/warp.git
```

## License

AGPL v3 (client) + MIT (UI crates). Same as upstream.
