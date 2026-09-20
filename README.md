# Praxis Engine

Revolutionary knowledge system that replaces verbose AI skills (~1500 tokens) with compressed, execution-optimized learnings (~200 tokens).

## Why Praxis?

Traditional skills suffer from:
- **Context bloat**: 1500+ tokens each, most content unused
- **Low utilization**: ~20% of loaded skill content actually gets applied
- **Attention decay**: Information at 70-85% context position gets ignored
- **No semantic matching**: Must load entire skill vs. relevant parts only

Praxis Engine solves these with:
- **90% context reduction**: ~200 tokens vs 1500
- **95%+ utilization**: Only load what's relevant
- **Semantic retrieval**: Vector search for relevance
- **Universal access**: CLI works with any agent

## Architecture

```
Meta-Learnings (300 tok)  →  Supabase + pgvector  →  Rust CLI
     Always Active              Semantic Index         <200ms
```

## Quick Start

### 1. Install CLI

```bash
cd cli
cargo install --path .
```

### 2. Configure

Create `~/praxis-engine/.env`:

```env
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_ANON_KEY=your-anon-key
```

### 3. Use

```bash
# Search learnings by semantic query
learn search "react performance optimization"

# Get specific learning by slug
learn get anti-hallucination

# Inject all learnings for domains
learn inject react,typescript

# Output as JSON
learn search "testing" --json

# Raw output for piping
learn get prompt-caching --raw
```

## Learning Format

Every learning follows the **WHEN/SCAN/FIX/NOT** structure:

```
## Anti-Hallucination
WHEN: Generating technical content (code, configs, commands)
SCAN: Invented APIs, fictional methods, "I assume" phrases
FIX: cite source → verify against docs → explicit uncertainty when unsure
NOT: assume existence | confident without source | skip verification
```

### Size Tiers

| Tier | Tokens | Use Case |
|------|--------|----------|
| Micro | 50-100 | Single insight |
| Standard | 150-300 | Complete pattern |
| Complex | 300-500 | Multi-step workflow |

## Meta-Learnings

Add to `CLAUDE.md` or `AGENTS.md` in any project (~280 tokens):

```markdown
## Praxis Engine

This project uses execution-optimized learnings. Run `learn` CLI for relevant knowledge.

### Task Analysis
WHEN: Starting any task
FIX: extract keywords → `learn search "<task summary>"` → apply top learnings

### Learning Retrieval
WHEN: Domain-specific work
FIX: `learn inject <domains>` → integrate into approach → cite applied

### Execution Tracking
WHEN: Applying any learning
FIX: verify SCAN matches → execute FIX sequence → validate NOT avoided

### Gap Detection
WHEN: Uncertain, no search results
FIX: note gap → proceed with caution → flag for learning creation
```

## Adding Learnings

### Via Craft Agent

```
/learning-from https://example.com/article --domains "react,performance"
```

Automatically pushes to both Supabase (CLI access) and Craft (human reading).

### Via API

```bash
curl -X POST 'https://your-project.supabase.co/functions/v1/embed-learning' \
  -H 'Authorization: Bearer YOUR_SERVICE_KEY' \
  -H 'Content-Type: application/json' \
  -d '{
    "slug": "my-learning",
    "name": "My Learning",
    "content": "WHEN: ...\nSCAN: ...\nFIX: ...\nNOT: ...",
    "domains": ["react", "performance"]
  }'
```

## Project Structure

```
praxis-engine/
├── .planning/          # GSD planning documents
├── cli/                # Rust CLI (learn command)
├── supabase/
│   ├── schema.sql      # Database schema
│   └── functions/      # Edge Functions
├── docs/
│   ├── LEARNING-FORMAT.md
│   ├── CLAUDE-TEMPLATE.md
│   └── AGENTS-TEMPLATE.md
└── README.md
```

## Edge Functions

| Function | Purpose |
|----------|---------|
| `embed-learning` | Generate embedding and store learning |
| `search-learnings` | Semantic similarity search |
| `get-learning` | Retrieve by slug |
| `get-by-domains` | Retrieve all for domains |

## Comparison

| Metric | Skills | Praxis Learnings |
|--------|--------|------------------|
| Size | 1500+ tok | 150-300 tok |
| Utilization | ~20% | 95%+ |
| Loading | All or nothing | On-demand |
| Search | Filename only | Semantic |
| Cross-agent | No | Yes (CLI) |

## License

MIT
