# Praxis Engine Setup Guide

## Prerequisites
- Supabase project
- OpenAI API key (for embeddings)
- Rust installed (for CLI)

## Step 1: Database Setup

### Option A: Via Supabase Dashboard
1. Open your project's SQL Editor in the Supabase dashboard.
2. Copy contents of `supabase/schema.sql`
3. Paste and run in SQL Editor

### Option B: Via CLI
```bash
supabase login
cd praxis-engine
supabase link --project-ref YOUR_PROJECT_REF
supabase db push
```

## Step 2: Edge Functions Secrets

Set these secrets in Supabase Dashboard → Settings → Edge Functions → Secrets:

| Key | Value |
|-----|-------|
| `OPENAI_API_KEY` | Your OpenAI API key |

Or via CLI:
```bash
supabase secrets set OPENAI_API_KEY=sk-your-key
```

## Step 3: Deploy Edge Functions

### Option A: Via Dashboard
Go to Edge Functions in dashboard and deploy manually.

### Option B: Via CLI
```bash
cd praxis-engine
supabase functions deploy embed-learning
supabase functions deploy search-learnings
supabase functions deploy get-learning
supabase functions deploy get-by-domains
```

## Step 4: Test

```bash
# Test embed (requires service key)
curl -X POST 'https://your-project.supabase.co/functions/v1/embed-learning' \
  -H 'Authorization: Bearer YOUR_SERVICE_KEY' \
  -H 'Content-Type: application/json' \
  -d '{
    "slug": "test-learning",
    "name": "Test Learning",
    "content": "WHEN: testing\nDO: verify → validate → confirm\nNOT: skip tests",
    "domains": ["testing"]
  }'

# Test search (uses anon key)
curl -X POST 'https://your-project.supabase.co/functions/v1/search-learnings' \
  -H 'Authorization: Bearer YOUR_ANON_KEY' \
  -H 'Content-Type: application/json' \
  -d '{"query": "testing best practices"}'
```

## Step 5: Build Rust CLI

```bash
cd praxis-engine/cli
cargo build --release
cargo install --path .
```

## Verification

After setup, you should be able to:
1. Insert learnings via Edge Function
2. Search learnings semantically
3. Use `learn` CLI from any terminal
