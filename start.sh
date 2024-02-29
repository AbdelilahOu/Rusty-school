#!/bin/sh

set -e

echo "run db migrations"
./migration up -u "$DATABASE_URL"

echo "start the app"
exec "$@"