#!/usr/bin/env bash
set -efuo pipefail
no_proxy="*" curl -v --header "X-Gitlab-Event: Push Hook" http://localhost:8000/api/v1/push/gitlab -d @example-request.json --header "Content-Type: application/json"
