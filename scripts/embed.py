#!/usr/bin/env python3
"""
Embed learnings using Ollama (nomic-embed-text) and update Supabase.
Run: python scripts/embed.py
"""

import os
import json
import requests
from pathlib import Path

# Load config from .env
env_path = Path(__file__).parent.parent / ".env"
config = {}
if env_path.exists():
    for line in env_path.read_text().splitlines():
        if "=" in line and not line.startswith("#"):
            key, value = line.split("=", 1)
            config[key.strip()] = value.strip()

SUPABASE_URL = config.get("SUPABASE_URL", "")
SUPABASE_KEY = config.get("SUPABASE_ANON_KEY", "")
OLLAMA_URL = "http://localhost:11434"
EMBED_MODEL = "nomic-embed-text"

def get_embedding(text: str) -> list[float]:
    """Generate embedding using Ollama."""
    response = requests.post(
        f"{OLLAMA_URL}/api/embeddings",
        json={"model": EMBED_MODEL, "prompt": text}
    )
    response.raise_for_status()
    return response.json()["embedding"]

def get_learnings_without_embeddings() -> list[dict]:
    """Fetch learnings that don't have embeddings yet."""
    response = requests.get(
        f"{SUPABASE_URL}/rest/v1/learnings",
        params={"embedding": "is.null", "select": "id,slug,name,content"},
        headers={
            "apikey": SUPABASE_KEY,
            "Authorization": f"Bearer {SUPABASE_KEY}"
        }
    )
    response.raise_for_status()
    return response.json()

def update_embedding(learning_id: str, embedding: list[float]) -> bool:
    """Update a learning with its embedding."""
    response = requests.patch(
        f"{SUPABASE_URL}/rest/v1/learnings",
        params={"id": f"eq.{learning_id}"},
        headers={
            "apikey": SUPABASE_KEY,
            "Authorization": f"Bearer {SUPABASE_KEY}",
            "Content-Type": "application/json"
        },
        json={"embedding": embedding}
    )
    return response.status_code in [200, 204]

def main():
    print("🔍 Fetching learnings without embeddings...")
    learnings = get_learnings_without_embeddings()

    if not learnings:
        print("✅ All learnings already have embeddings!")
        return

    print(f"📚 Found {len(learnings)} learnings to embed\n")

    for learning in learnings:
        slug = learning["slug"]
        text = f"{learning['name']}\n{learning['content']}"

        print(f"  → Embedding: {slug}...", end=" ", flush=True)

        try:
            embedding = get_embedding(text)
            success = update_embedding(learning["id"], embedding)

            if success:
                print(f"✓ ({len(embedding)} dims)")
            else:
                print("✗ Failed to update")
        except Exception as e:
            print(f"✗ Error: {e}")

    print("\n✅ Done!")

if __name__ == "__main__":
    main()
