-- Praxis Engine Schema
-- Run this in Supabase SQL Editor

-- Enable pgvector extension
create extension if not exists vector;

-- Learnings table
create table if not exists learnings (
  id uuid primary key default gen_random_uuid(),
  slug text unique not null,
  name text not null,
  content text not null,

  -- Metadata for retrieval
  domains text[] not null default '{}',
  triggers text[] not null default '{}',
  scan_patterns text[] not null default '{}',
  task_types text[] not null default '{}',

  -- Embedding for semantic search (OpenAI text-embedding-3-small = 1536 dims)
  embedding vector(1536),

  -- Tracking
  created_at timestamp with time zone default now(),
  updated_at timestamp with time zone default now(),
  source_url text,
  version integer default 1,

  -- Usage stats (for future effectiveness tracking)
  retrieval_count integer default 0,
  application_count integer default 0
);

-- Index for vector similarity search
create index if not exists learnings_embedding_idx
  on learnings
  using ivfflat (embedding vector_cosine_ops)
  with (lists = 100);

-- Index for domain filtering
create index if not exists learnings_domains_idx
  on learnings
  using gin (domains);

-- Index for slug lookups
create index if not exists learnings_slug_idx
  on learnings (slug);

-- Function: Search learnings by semantic similarity
create or replace function search_learnings(
  query_embedding vector(1536),
  match_threshold float default 0.7,
  match_count int default 5,
  filter_domains text[] default null
)
returns table (
  id uuid,
  slug text,
  name text,
  content text,
  domains text[],
  similarity float
)
language plpgsql
as $$
begin
  return query
  select
    l.id,
    l.slug,
    l.name,
    l.content,
    l.domains,
    1 - (l.embedding <=> query_embedding) as similarity
  from learnings l
  where
    l.embedding is not null
    and 1 - (l.embedding <=> query_embedding) > match_threshold
    and (filter_domains is null or l.domains && filter_domains)
  order by l.embedding <=> query_embedding
  limit match_count;
end;
$$;

-- Function: Get learnings by domains
create or replace function get_learnings_by_domains(
  target_domains text[]
)
returns table (
  slug text,
  name text,
  content text,
  domains text[]
)
language plpgsql
as $$
begin
  return query
  select
    l.slug,
    l.name,
    l.content,
    l.domains
  from learnings l
  where l.domains && target_domains
  order by l.name;
end;
$$;

-- RLS Policies (anon can read, service_role can write)
alter table learnings enable row level security;

-- Allow anyone to read
create policy "Allow public read" on learnings
  for select using (true);

-- Only service_role can insert/update/delete
create policy "Service role can insert" on learnings
  for insert with check (auth.role() = 'service_role');

create policy "Service role can update" on learnings
  for update using (auth.role() = 'service_role');

create policy "Service role can delete" on learnings
  for delete using (auth.role() = 'service_role');

-- Trigger: Update updated_at timestamp
create or replace function update_updated_at()
returns trigger as $$
begin
  new.updated_at = now();
  return new;
end;
$$ language plpgsql;

create trigger learnings_updated_at
  before update on learnings
  for each row
  execute function update_updated_at();
