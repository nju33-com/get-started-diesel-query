# ecogreen slack

## bin

terminal 1

```bash
. let.sh
make run-postgres
```

terminal 2

```bash
. let.sh
cargo run -p ecogreen-slack -- "$LOCAL_PG_CONN" 9000
```

## slack

```bash
make run-slack-api
# https://ecogreen--slack.jp.ngrok.io/api/slack/slash-command.ts
```
