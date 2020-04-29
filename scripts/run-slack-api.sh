#!/usr/bin/env bash

export PATH="node_modules/.bin:$PATH"

SUBDOMAIN='ecogreen--slack'
PORT='23456'
NOW_PID=
NGROK_PID=

set -ueo pipefail

process-kill() {
	kill -KILL "$(jobs -p)"
	kill -KILL "$(lsof -ti "tcp:$PORT")"
}

trap 'process-kill' EXIT
trap 'process-kill' ERR

now dev --listen "$PORT" &
NOW_PID="$!"

ngrok http "$PORT" -subdomain "$SUBDOMAIN" --region jp -log stdout >/dev/null &
NGROK_PID="$!"

jobs

echo ngrok: http://127.0.0.1:4040
echo api: https://ecogreen--slack.jp.ngrok.io/api/slack/slash-commands/ecog--reset-tt-by-track-number.ts

sleep infinity
