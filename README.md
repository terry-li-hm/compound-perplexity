# compound-perplexity

CLI for the Perplexity API. Search, ask, research, and reason from your terminal.

## Install

```bash
cargo install compound-perplexity
```

The binary is called `compound-pplx`.

## Commands

| Command | What it does | Model | Est. cost |
|---------|-------------|-------|-----------|
| `compound-pplx search <query>` | Quick web search | sonar | ~$0.006 |
| `compound-pplx ask <query>` | Pro search with analysis | sonar-pro | ~$0.01 |
| `compound-pplx research <query>` | Deep research | sonar-deep-research | ~$0.40 |
| `compound-pplx reason <query>` | Reasoning with thinking | sonar-reasoning-pro | ~$0.01 |
| `compound-pplx log` | Show recent queries | — | — |
| `compound-pplx log --stats` | Usage and cost breakdown | — | — |

## Features

- **Four modes** matching Perplexity's model tiers — from quick lookups to deep research
- **Cited sources** extracted and displayed inline
- **Cost tracking** — every query logged with timestamp, model, estimated cost, and duration
- **Color-coded output** — each mode has a distinct colour for easy scanning
- **`--raw`** flag for JSON output, **`--no-log`** to skip history

Requires `PERPLEXITY_API_KEY` in your environment.

## License

MIT
