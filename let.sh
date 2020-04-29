#!/usr/bin/env bash

export LOCAL_PG_CONN='postgres://postgres:@localhost:23455'
alias ecodb="psql $LOCAL_PG_CONN"
