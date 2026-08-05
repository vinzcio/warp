# privacy-zdr fork

This is a personal fork of the open-source **Warp client** with privacy defaults flipped off.

Fork: https://github.com/vinzcio/warp  
Branch: `privacy-zdr`  
Local path: `/Users/vincent/src/warp-zdr`

## What you get

| Change | Effect |
|--------|--------|
| `privacy.telemetry_enabled` default → **false** | No “help improve Warp” / console-interaction telemetry by default |
| `privacy.crash_reporting_enabled` default → **false** | No crash dumps to Warp by default |
| `agents.cloud_conversation_storage_enabled` default → **false** | Agent conversations not stored in Warp cloud by default |

## What this is **not**

**Forking the client does not give you Warp Enterprise ZDR**, and it does **not** open-source:

- Warp’s **server** / hosted AI agent harness  
- **Oz** orchestration  
- **Warp Drive** backend  
- Hosted auth / billing  

From upstream FAQ: the built-in agent runs **server-side** and is **not** in this repo.  
ACP / bring-your-own agent is on their roadmap, not fully here yet.

So:

- **Real ZDR for models** = you use **your own** LLM API keys (or local models) under a provider policy that is no-train / ZDR, **or** you stay offline.
- **This fork** = client defaults that stop optional Warp telemetry / cloud conversation storage, so the terminal itself is not “learning on your work” via Warp improvement pipelines.

If you still click Warp Agent / hosted models while logged into Warp, prompts still go to Warp’s cloud — that path is proprietary and not fixed by this patch set.

## Recommended agentic setup (actual ZDR)

1. Build/run this client with collection defaults off (or use stock Ghostty).
2. Run agents **in the terminal** with your keys:
   - Claude Code / Codex / Gemini CLI / Grok, etc.
3. Prefer API plans that contractually **do not train** and, if required, **ZDR** (enterprise/API terms — not free consumer chat).
4. Optional: secret redaction on in Settings → Privacy; keep AI “ambient” features off.

## Build (when you have disk)

You currently have limited free space (~14 GB). A full Rust `target/` build often needs **20–40 GB**. Free disk first, then:

```bash
cd /Users/vincent/src/warp-zdr
./script/bootstrap   # installs Rust toolchain + deps
./script/run         # build + launch
```

Or:

```bash
./script/install_rust
cargo run -p warp   # package name may vary; see Cargo.toml / AGENTS.md
```

## User settings override (any build)

If you already have Warp settings, force offline-friendly prefs in your Warp settings TOML (paths vary by platform; macOS often under `~/Library/Application Support/dev.warp.Warp-Stable/`):

```toml
[privacy]
telemetry_enabled = false
crash_reporting_enabled = false

[agents]
cloud_conversation_storage_enabled = false
```

## Remotes

```text
origin    https://github.com/vinzcio/warp.git   (your fork)
upstream  https://github.com/warpdotdev/warp.git
```

Sync:

```bash
git fetch upstream
git merge upstream/master   # or rebase
```

## License

Upstream is **AGPL v3** (client) + **MIT** (UI crates). This fork inherits that. Distributing a modified client publicly triggers AGPL source obligations.
