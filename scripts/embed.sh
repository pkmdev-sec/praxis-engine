#!/bin/bash
# Embed learnings using Ollama and update Supabase
# Usage: ./scripts/embed.sh

set -e

# Load config
source "$(dirname "$0")/../.env"

echo "🔍 Fetching learnings without embeddings..."

# Get learnings without embeddings
LEARNINGS=$(curl -s "${SUPABASE_URL}/rest/v1/learnings?embedding=is.null&select=id,slug,name,content" \
  -H "apikey: ${SUPABASE_ANON_KEY}" \
  -H "Authorization: Bearer ${SUPABASE_ANON_KEY}")

# Check if empty
if [ "$LEARNINGS" = "[]" ]; then
  echo "✅ All learnings already have embeddings!"
  exit 0
fi

# Count learnings
COUNT=$(echo "$LEARNINGS" | jq length)
echo "📚 Found $COUNT learnings to embed"
echo ""

# Process each learning
echo "$LEARNINGS" | jq -c '.[]' | while read -r learning; do
  ID=$(echo "$learning" | jq -r '.id')
  SLUG=$(echo "$learning" | jq -r '.slug')
  NAME=$(echo "$learning" | jq -r '.name')
  CONTENT=$(echo "$learning" | jq -r '.content')

  TEXT="${NAME}
${CONTENT}"

  echo -n "  → Embedding: $SLUG... "

  # Get embedding from Ollama
  EMBEDDING=$(curl -s http://localhost:11434/api/embeddings \
    -d "$(jq -n --arg text "$TEXT" '{model: "nomic-embed-text", prompt: $text}')" \
    | jq -c '.embedding')

  if [ -z "$EMBEDDING" ] || [ "$EMBEDDING" = "null" ]; then
    echo "✗ Failed to get embedding"
    continue
  fi

  # Update Supabase
  RESULT=$(curl -s -o /dev/null -w "%{http_code}" \
    -X PATCH "${SUPABASE_URL}/rest/v1/learnings?id=eq.${ID}" \
    -H "apikey: ${SUPABASE_ANON_KEY}" \
    -H "Authorization: Bearer ${SUPABASE_ANON_KEY}" \
    -H "Content-Type: application/json" \
    -d "{\"embedding\": $EMBEDDING}")

  if [ "$RESULT" = "204" ] || [ "$RESULT" = "200" ]; then
    DIMS=$(echo "$EMBEDDING" | jq 'length')
    echo "✓ ($DIMS dims)"
  else
    echo "✗ Failed to update (HTTP $RESULT)"
  fi
done

echo ""
echo "✅ Done!"
