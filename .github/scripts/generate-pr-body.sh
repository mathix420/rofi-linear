#!/bin/bash
# Generate PR title and body with optional Claude AI enhancement
# Usage: generate-pr-body.sh <changes_file> "<fallback_title>" <output_mode>
# output_mode: "title" | "body" | "both" (default: both)

CHANGES_FILE="$1"
FALLBACK_TITLE="$2"
OUTPUT_MODE="${3:-both}"

CHANGES=$(cat "$CHANGES_FILE")

# If no API key, use simple format
if [ -z "$ANTHROPIC_API_KEY" ]; then
    if [ "$OUTPUT_MODE" = "title" ]; then
        echo "$FALLBACK_TITLE"
    elif [ "$OUTPUT_MODE" = "body" ]; then
        cat << EOF
## Summary

This PR contains the following changes:

$CHANGES

---
*Auto-generated PR description*
EOF
    else
        echo "TITLE:$FALLBACK_TITLE"
        echo "BODY_START"
        cat << EOF
## Summary

This PR contains the following changes:

$CHANGES

---
*Auto-generated PR description*
EOF
    fi
    exit 0
fi

# Use Claude API to generate title and body
PROMPT="You are helping write a GitHub pull request title and description. Based on the following git changes, generate both.

Fallback title (use as context for the type of PR): $FALLBACK_TITLE

Changes:
$CHANGES

Generate:
1. A PR title following conventional commits format (e.g., \"feat: add user authentication\", \"fix: resolve login timeout\", \"chore: update dependencies\"). Keep it under 72 characters, lowercase after the type prefix.
2. A PR description with:
   - A brief summary (2-3 sentences max)
   - Key changes as bullet points
   - Any notable implementation details

Output format (MUST follow exactly):
TITLE:<the title here>
BODY_START
<the body here>

Keep it concise. Use markdown formatting for the body."

# Escape the prompt for JSON
ESCAPED_PROMPT=$(printf '%s' "$PROMPT" | jq -Rs '.')

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
CONTENT=$(echo "$RESPONSE" | jq -r '.content[0].text // empty')

if [ -n "$CONTENT" ]; then
    # Parse title and body from response
    TITLE=$(echo "$CONTENT" | grep "^TITLE:" | sed 's/^TITLE://')
    BODY=$(echo "$CONTENT" | sed -n '/^BODY_START$/,$ p' | tail -n +2)

    # Fallback if parsing fails
    [ -z "$TITLE" ] && TITLE="$FALLBACK_TITLE"
    [ -z "$BODY" ] && BODY="$CONTENT"

    if [ "$OUTPUT_MODE" = "title" ]; then
        echo "$TITLE"
    elif [ "$OUTPUT_MODE" = "body" ]; then
        echo "$BODY"
        echo ""
        echo "---"
        echo "*AI-generated PR description*"
    else
        echo "TITLE:$TITLE"
        echo "BODY_START"
        echo "$BODY"
        echo ""
        echo "---"
        echo "*AI-generated PR description*"
    fi
else
    # Fallback if API call fails
    if [ "$OUTPUT_MODE" = "title" ]; then
        echo "$FALLBACK_TITLE"
    elif [ "$OUTPUT_MODE" = "body" ]; then
        cat << EOF
## Summary

This PR contains the following changes:

$CHANGES

---
*Auto-generated PR description (API fallback)*
EOF
    else
        echo "TITLE:$FALLBACK_TITLE"
        echo "BODY_START"
        cat << EOF
## Summary

This PR contains the following changes:

$CHANGES

---
*Auto-generated PR description (API fallback)*
EOF
    fi
fi
