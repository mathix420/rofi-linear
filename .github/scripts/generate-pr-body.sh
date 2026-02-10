#!/bin/bash
# Generate PR body with optional Claude AI enhancement
# Usage: generate-pr-body.sh "<changes>" "<title>"

CHANGES="$1"
TITLE="$2"

# If no API key, use simple format
if [ -z "$ANTHROPIC_API_KEY" ]; then
    cat << EOF
## Summary

This PR contains the following changes:

$CHANGES

---
*Auto-generated PR description*
EOF
    exit 0
fi

# Use Claude API to generate a better description
PROMPT="You are helping write a GitHub pull request description. Based on the following git changes, write a concise and helpful PR description.

Title: $TITLE

Changes:
$CHANGES

Write a PR description with:
1. A brief summary (2-3 sentences max)
2. Key changes as bullet points
3. Any notable implementation details

Keep it concise. Use markdown formatting. Do not include a title - just the body content."

# Escape the prompt for JSON
ESCAPED_PROMPT=$(echo "$PROMPT" | jq -Rs '.')

RESPONSE=$(curl -s https://api.anthropic.com/v1/messages \
    -H "Content-Type: application/json" \
    -H "x-api-key: $ANTHROPIC_API_KEY" \
    -H "anthropic-version: 2023-06-01" \
    -d "{
        \"model\": \"claude-sonnet-4-20250514\",
        \"max_tokens\": 1024,
        \"messages\": [{
            \"role\": \"user\",
            \"content\": $ESCAPED_PROMPT
        }]
    }")

# Extract the text content from the response
BODY=$(echo "$RESPONSE" | jq -r '.content[0].text // empty')

if [ -n "$BODY" ]; then
    echo "$BODY"
    echo ""
    echo "---"
    echo "*AI-generated PR description*"
else
    # Fallback if API call fails
    cat << EOF
## Summary

This PR contains the following changes:

$CHANGES

---
*Auto-generated PR description (API fallback)*
EOF
fi
