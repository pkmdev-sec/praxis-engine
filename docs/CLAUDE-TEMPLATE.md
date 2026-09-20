# CLAUDE.md - Praxis Meta-Learnings

Add this to any project's CLAUDE.md to enable learning-based execution.

---

## Praxis Engine

This project uses execution-optimized learnings. Run `learn` CLI for relevant knowledge.

### Task Analysis
WHEN: Starting any task
SCAN: technologies mentioned, error types, code patterns in request
FIX: extract keywords → `learn search "<task summary>"` → apply top learnings
NOT: skip search | apply irrelevant learnings | ignore domain context

### Learning Retrieval
WHEN: Domain-specific work (react, python, security, etc.)
SCAN: file extensions, imports, framework patterns in codebase
FIX: `learn inject <domains>` → integrate into approach → cite applied
NOT: guess patterns | skip retrieval | apply without context check

### Execution Tracking
WHEN: Applying any learning
SCAN: learning's SCAN conditions match current code
FIX: verify match → execute FIX sequence → validate NOT conditions avoided
NOT: partial application | skip verification | ignore NOT warnings

### Gap Detection
WHEN: Uncertain about approach, seeing unfamiliar pattern
SCAN: no relevant search results, low similarity scores
FIX: note gap → proceed with caution → flag for learning creation
NOT: hallucinate solution | pretend confidence | skip gap logging

---

**Total: ~280 tokens**
