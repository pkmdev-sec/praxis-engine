# Praxis Engine

## Vision
Revolutionary knowledge system that replaces verbose AI skills (~1500 tokens) with compressed, execution-optimized learnings (~200 tokens).

## Core Problem
Traditional skills suffer from:
- **Context bloat**: 1500+ tokens each, most unused
- **Low utilization**: ~20% of loaded skill content gets applied
- **Attention decay**: Information degrades at 70-85% context position
- **No semantic matching**: Load entire skill vs. relevant parts

## Solution: Praxis Engine
- **90% context reduction**: 200 tokens vs 1500
- **95%+ utilization**: Only load what's needed
- **Semantic retrieval**: Vector search for relevance
- **Universal access**: Rust CLI works with any agent

## Architecture
```
Meta-Learnings (300 tok)  →  Supabase + pgvector  →  Rust CLI
     Always Active              Semantic Index         <200ms
```

## Key Innovation
**Meta-learnings** teach agents HOW to discover learnings, solving cold-start.
Only ~300 tokens always loaded, everything else retrieved on-demand.

## Success Metrics
| Metric | Before | After |
|--------|--------|-------|
| Context per skill | 1500 tok | 200 tok |
| Utilization rate | 20% | 95%+ |
| Retrieval time | N/A | <200ms |
| Always-loaded | 5000+ tok | 300 tok |
