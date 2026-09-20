# Learning Format Specification

## Overview

Learnings are compressed, execution-optimized knowledge units designed for AI agent consumption. They replace verbose skills (~1500 tokens) with focused instructions (~200 tokens).

## Structure

Every learning follows the **WHEN/SCAN/FIX/NOT** structure:

```
## [2-4 Word Name]
WHEN: [single trigger condition - when to apply this learning]
SCAN: [what to look for in code/context before acting]
FIX: [action] → [action] → [action]
NOT: [antipattern] | [antipattern] | [antipattern]
```

### Field Definitions

| Field | Purpose | Format |
|-------|---------|--------|
| WHEN | Activation trigger | Single phrase describing when to apply |
| SCAN | Context check | What patterns/issues to identify first |
| FIX | Action sequence | Arrow-separated steps (→) |
| NOT | Antipatterns | Pipe-separated mistakes to avoid (\|) |

## Size Tiers

### Micro (50-100 tokens)
Single insight, immediate action.

```
## Prompt Caching
WHEN: Claude API calls with repeated context
SCAN: system prompts > 1024 tokens, unchanged between calls
FIX: add cache_control breakpoint → verify cache hit in headers
NOT: cache dynamic content | skip 1min TTL wait
```

### Standard (150-300 tokens)
Complete pattern with context.

```
## Anti-Hallucination
WHEN: Generating technical content (code, configs, commands)
SCAN: Invented APIs, fictional methods, "I assume" phrases
FIX: cite source → verify against docs → explicit uncertainty when unsure
NOT: assume existence | confident without source | skip verification
```

### Complex (300-500 tokens)
Multi-step patterns with alternatives.

```
## React Performance
WHEN: React component shows render lag or bundle size issues
SCAN: re-renders without prop changes, large initial bundles, blocked main thread
FIX: profile with DevTools → identify bottleneck → apply targeted fix:
  - Unnecessary re-renders: useMemo/useCallback/React.memo
  - Bundle size: dynamic imports, code splitting
  - Blocking: move to Web Worker or defer
NOT: premature optimization | memo everything | optimize before profiling
```

## Metadata

Each learning includes metadata for retrieval:

```json
{
  "slug": "anti-hallucination",
  "name": "Anti-Hallucination",
  "domains": ["ai", "llm", "agents"],
  "triggers": ["generating code", "technical output", "api calls"],
  "scan_patterns": ["invented", "fictional", "assume"],
  "task_types": ["coding", "documentation", "api-integration"]
}
```

### Metadata Fields

| Field | Type | Purpose |
|-------|------|---------|
| `slug` | string | Unique identifier (kebab-case) |
| `name` | string | Human-readable name |
| `domains` | string[] | Technology/topic domains |
| `triggers` | string[] | Keywords that activate retrieval |
| `scan_patterns` | string[] | Code patterns to match |
| `task_types` | string[] | Task categories where applicable |

## Domain Taxonomy

Standardized domains for consistent retrieval:

### Languages
`javascript`, `typescript`, `python`, `rust`, `go`, `java`, `ruby`, `swift`

### Frameworks
`react`, `nextjs`, `vue`, `angular`, `express`, `fastapi`, `django`

### Concepts
`ai`, `llm`, `agents`, `testing`, `security`, `performance`, `architecture`
`debugging`, `devops`, `database`, `api`, `frontend`, `backend`

## Writing Guidelines

1. **Be specific** - Vague advice wastes tokens
2. **Action-oriented** - Tell what to DO, not what IS
3. **Scannable** - Agent should execute in seconds
4. **Composable** - Learnings can combine for complex tasks
5. **Verifiable** - Include check conditions when possible

## Example: Converting a Skill to Learning

### Before (Skill - 1500+ tokens)
```markdown
# React Performance Optimization

## Overview
React applications can suffer from various performance issues...
[500 tokens of context]

## Common Issues
### 1. Unnecessary Re-renders
When a parent component re-renders, all children...
[300 tokens]

### 2. Large Bundle Sizes
Modern JavaScript applications often ship...
[400 tokens]

## Solutions
...
```

### After (Learning - 200 tokens)
```
## React Performance
WHEN: React component shows render lag or bundle size issues
SCAN: DevTools Profiler shows wasted renders, Lighthouse reports large JS
FIX: identify bottleneck → apply targeted fix:
  - Wasted renders: memo, useMemo, key prop audit
  - Bundle: lazy(), dynamic import(), route-based splitting
  - Initial load: defer non-critical, Suspense boundaries
NOT: wrap everything in memo | optimize before measuring | ignore profiler
```

## Storage

Learnings are stored in Supabase with pgvector embeddings for semantic search:

```sql
create table learnings (
  id uuid primary key,
  slug text unique not null,
  name text not null,
  content text not null,          -- The WHEN/SCAN/FIX/NOT content
  domains text[] not null,
  triggers text[] not null,
  embedding vector(1536),          -- OpenAI text-embedding-3-small
  ...
);
```
