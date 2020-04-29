connection_string =

result-make-dump-ecogreen.sql:
	pg_dump $(connection_string) > result-make-dump-ecogreen.sql

.PHONY: dump-ecogreen check-env-usage-dump-ecogreen
dump-ecogreen: check-env-usage-dump-ecogreen result-make-dump-ecogreen.sql
check-env-usage-dump-ecogreen:
ifndef connection_string
	$(error 'connection_string required string starts with postgres://')
endif

.PHONY: run-postgres
run-postgres:
	./scripts/run-postgres.sh

.PHONY: run-slack-api
run-slack-api:
	./scripts/run-slack-api.sh
