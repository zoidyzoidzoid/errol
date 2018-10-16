#!/usr/bin/env bash
set -efuo pipefail
no_proxy="*" curl -v http://localhost:8080/api/v1/hooks/gitlab -d @example-request.json --header "Content-Type: application/json"
no_proxy="*" curl -v --header "X-Gitlab-Event: Push Hook" http://localhost:8080/api/v1/hooks/gitlab --header "Content-Type: application/json"
no_proxy="*" curl -v --header "X-Gitlab-Event: Push Hook" http://localhost:8080/api/v1/hooks/gitlab -d @example-request.json
