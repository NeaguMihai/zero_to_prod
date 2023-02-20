#! /bin/sh

set -e

export $DB_HOST=postgres

envsubst < ./.env.test > ./.env

cat ./.env

sudo apt-get install libpq-dev

cargo install diesel_cli --no-default-features --features postgres

docker run -d -e POSTGRES_USER=test_app_user -e POSTGRES_PASSWORD=test_db_password -e POSTGRES_DB=test_db postgres

diesel migration run

docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin cargo tarpaulin --ignore-tests -o xml --output-dir coverage --skip-clean