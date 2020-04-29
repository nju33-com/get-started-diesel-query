#!/usr/bin/env bash

CONTAINER_NAME='ecogreen--slack'
PORT='23455'
CONNECTION_STRING="postgres://postgres:@localhost:$PORT"

set -ueo pipefail

docker-kill() {
	docker rm -f "$CONTAINER_NAME"
}

wait_built() {
	set +e
	local status
	psql "$CONNECTION_STRING" -c '\l' >/dev/null 2>&1
	status="$?"

	if [ $status -ne 0 ]; then
		sleep 1
		wait_built
	fi
	set -e
}

trap 'docker-kill' EXIT
trap 'docker-kill' ERR

if [ ! -f "result-make-dump-ecogreen.sql" ]; then
	make dump-ecogreen
fi

docker run --name "$CONTAINER_NAME" -d --rm -p "$PORT:5432" -e POSTGRES_HOST_AUTH_METHOD=trust postgres:11
wait_built

psql "$CONNECTION_STRING" -c 'create schema ecogreen'
set +e
psql "$CONNECTION_STRING" <'result-make-dump-ecogreen.sql'
set -e

# psql "$CONNECTION_STRING" -c "insert into ecogreen.\"repairLog\" (active, endedat, createdat, updatedat, \"containerId\") values ('t', DATE'2020-04-30', DATE'2020-04-16', DATE'2020-04-16', 10)"

docker attach "$CONTAINER_NAME"
