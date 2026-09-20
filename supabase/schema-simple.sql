-- Praxis Engine Schema (Simple - No OpenAI Required)
-- Uses PostgreSQL full-text search instead of vector embeddings

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

  -- Full-text search vector (auto-generated)
  search_vector tsvector generated always as (
    setweight(to_tsvector('english', coalesce(name, '')), 'A') ||
    setweight(to_tsvector('english', coalesce(content, '')), 'B') ||
    setweight(to_tsvector('english', coalesce(array_to_string(domains, ' '), '')), 'C') ||
    setweight(to_tsvector('english', coalesce(array_to_string(triggers, ' '), '')), 'C')
  ) stored,

  -- Tracking
  created_at timestamp with time zone default now(),
  updated_at timestamp with time zone default now(),
  source_url text,
  version integer default 1,

  -- Usage stats
  retrieval_count integer default 0,
  application_count integer default 0
);

-- Index for full-text search
create index if not exists learnings_search_idx
  on learnings using gin (search_vector);

-- Index for domain filtering
create index if not exists learnings_domains_idx
  on learnings using gin (domains);

-- Index for slug lookups
create index if not exists learnings_slug_idx
  on learnings (slug);

-- Function: Search learnings by text query
create or replace function search_learnings_text(
  query_text text,
  match_count int default 5,
  filter_domains text[] default null
)
returns table (
  id uuid,
  slug text,
  name text,
  content text,
  domains text[],
  rank real
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
    ts_rank(l.search_vector, websearch_to_tsquery('english', query_text)) as rank
  from learnings l
  where
    l.search_vector @@ websearch_to_tsquery('english', query_text)
    and (filter_domains is null or l.domains && filter_domains)
  order by rank desc
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

-- RLS Policies
alter table learnings enable row level security;

-- Allow anyone to read
create policy "Allow public read" on learnings
  for select using (true);

-- Allow anyone to insert (for testing - tighten later)
create policy "Allow public insert" on learnings
  for insert with check (true);

-- Allow anyone to update (for testing - tighten later)
create policy "Allow public update" on learnings
  for update using (true);

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
