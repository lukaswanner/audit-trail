#!/usr/bin/env bash
# sudo ... might not work since it switches enviroments 
# enable this if you want debuging
# set -x

set -eo pipefail

if docker info > /dev/null 2>&1; then
    echo "Docker is running"
else
    echo "Docker is not running"
    exit 1
fi

if ! command -v psql > /dev/null 2>&1; then
    echo "psql is not installed"
    exit 1
fi

if ! command -v sqlx > /dev/null 2>&1; then
    echo "sqlx is not installed"
    echo "Use:"
    echo "cargo install sqlx-cli"
    echo "to install it"
    exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=audit_trail}
DB_PORT=${POSTGRES_PORT:=5432}
DB_HOST=${POSTGRES_HOST:=localhost}

if [[ -z "${SKIP_DOCKER}" ]]
then

	RUNNING_CONTAINER=$(docker ps --filter "name=audit_trail_db" --format "{{.ID}}")
	if [[ -n $RUNNING_CONTAINER ]]; then
		echo "Stopping and removing existing audit_trail_db container"
		docker stop $RUNNING_CONTAINER > /dev/null 2>&1
		docker rm $RUNNING_CONTAINER > /dev/null 2>&1
	fi
	docker run \
		-e POSTGRES_USER=${DB_USER} \
		-e POSTGRES_PASSWORD=${DB_PASSWORD} \
		-e POSTGRES_DB=${DB_NAME} \
		-p "${DB_PORT}":5432 \
		-d \
		--name "audit_trail_$(date '+%s')" \
		postgres -N 1000
fi

until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
	echo "Waiting for postgres to start"
	sleep 1;
done

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

echo "Database audit_trail_db created, ready to go!"


