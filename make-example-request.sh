#!/usr/bin/env bash
set -efuo pipefail
no_proxy="*" curl -vX POST --header "X-Gitlab-Event: Push Hook" http://127.0.0.1:8080/api/v1/push -d @example-request.json --header "Content-Type: application/json"
