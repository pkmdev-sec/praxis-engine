# Requirements

## Functional Requirements

### FR1: Learning Storage
- Store learnings with semantic embeddings (1536-dim vectors)
- Support domains, triggers, and scan patterns as metadata
- Enable similarity search with configurable threshold

### FR2: Rust CLI
- `learn search "query"` - semantic search, return top 5
- `learn get <slug>` - retrieve specific learning by slug
- `learn inject <domains>` - output all learnings for domains
- Response time < 200ms for all operations
- Output formats: plain text (default), --json, --raw

### FR3: Dual-Push
- Single action pushes to both Supabase (search) and Craft (human reading)
- Parallel execution for speed
- Automatic embedding generation

### FR4: Meta-Learnings
- Always-active learnings (~300 tokens total)
- Embedded in CLAUDE.md / AGENTS.md
- Teach: task analysis, retrieval, tracking, gap detection

## Non-Functional Requirements

### NFR1: Performance
- CLI response: < 200ms
- Edge Function: < 500ms
- Cold start acceptable for Edge Functions

### NFR2: Compatibility
- Works with: Claude Code, Craft Agent, Codex CLI, any shell-capable agent
- No MCP dependency (avoids 3000+ token bloat)

### NFR3: Security
- Service key never in CLI (use anon key with RLS)
- Env vars for sensitive config

## Learning Format Specification

### Structure
```
## [Name]
WHEN: [trigger condition]
SCAN: [what to look for in code/context]
FIX: [action] → [action] → [action]
NOT: [antipattern] | [antipattern]
```

### Size Tiers
- Micro: 50-100 tokens (single insight)
- Standard: 150-300 tokens (complete learning)
- Complex: 300-500 tokens (multi-step pattern)
