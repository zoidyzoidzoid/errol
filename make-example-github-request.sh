#!/usr/bin/env bash
set -efuo pipefail
export no_proxy="*"
curl -v --header "X-Github-Event: push" \
  http://localhost:8080/api/v1/hooks/github -d @example-github-push-hook.json \
  --header "Content-Type: application/json"
