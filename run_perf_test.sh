#!/bin/bash
set -e

echo "## $(date --iso-8601=seconds)"

echo '### Config'
echo '```'
cat config.yaml
echo '```'

./target/release/rust-postgres-queue-load-test > run.log

echo '### Result'
echo '```'
duckdb < analysis.duckdb.sql
echo '```'


