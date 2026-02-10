#!/bin/bash
# Collect changes between current branch and base branch
# Usage: collect-changes.sh <base_branch>

BASE_BRANCH="${1:-master}"

echo "## Commits"
echo ""

# Get commits that are in HEAD but not in base branch
if git rev-parse --verify "origin/$BASE_BRANCH" >/dev/null 2>&1; then
    COMMITS=$(git log --oneline "origin/$BASE_BRANCH..HEAD" 2>/dev/null)
else
    # If base branch doesn't exist remotely, use local
    COMMITS=$(git log --oneline "$BASE_BRANCH..HEAD" 2>/dev/null || git log --oneline -10)
fi

if [ -n "$COMMITS" ]; then
    echo "$COMMITS" | while read -r line; do
        echo "- $line"
    done
else
    echo "- Initial commits"
fi

echo ""
echo "## Files Changed"
echo ""

# Get diff stats
if git rev-parse --verify "origin/$BASE_BRANCH" >/dev/null 2>&1; then
    STATS=$(git diff --stat "origin/$BASE_BRANCH..HEAD" 2>/dev/null)
else
    STATS=$(git diff --stat "$BASE_BRANCH..HEAD" 2>/dev/null || echo "Unable to compute diff")
fi

echo '```'
echo "$STATS"
echo '```'
