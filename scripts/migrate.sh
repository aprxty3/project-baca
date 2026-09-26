#!/usr/bin/env bash
set -euo pipefail

# Project Baca — Database Migration Runner
# Executes paired SQL migrations (.up.sql and .down.sql) against PostgreSQL 17

DB_USER="${DB_USER:-baca_user}"
DB_NAME="${DB_NAME:-project_baca_db}"
MIGRATIONS_DIR="migrations"

psql_cmd() {
    docker compose exec -T postgres psql -U "$DB_USER" -d "$DB_NAME" -v ON_ERROR_STOP=1 "$@"
}

init_table() {
    psql_cmd -q -c "
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version VARCHAR(100) PRIMARY KEY,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
    "
}

migrate_up() {
    init_table
    echo "Checking migration files in ${MIGRATIONS_DIR}..."

    shopt -s nullglob
    local files=("${MIGRATIONS_DIR}"/*.up.sql)
    shopt -u nullglob

    if [[ ${#files[@]} -eq 0 ]]; then
        echo "No .up.sql migration files found."
        return 0
    fi

    local applied_count=0
    for file in "${files[@]}"; do
        local basename
        basename=$(basename "$file")
        local version="${basename%.up.sql}"

        local is_applied
        is_applied=$(psql_cmd -t -A -c "SELECT count(*) FROM schema_migrations WHERE version = '${version}';")

        if [[ "$is_applied" -eq 0 ]]; then
            echo "Applying: ${version}..."
            psql_cmd < "$file"
            psql_cmd -q -c "INSERT INTO schema_migrations (version) VALUES ('${version}');"
            echo "Applied: ${version}."
            applied_count=$((applied_count + 1))
        else
            echo "Skipped: ${version} (already applied)."
        fi
    done

    echo "Completed. Total new migrations applied: ${applied_count}."
}

migrate_down() {
    init_table
    local last_version
    last_version=$(psql_cmd -t -A -c "SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1;")

    if [[ -z "$last_version" ]]; then
        echo "No recorded migrations to roll back."
        return 0
    fi

    local down_file="${MIGRATIONS_DIR}/${last_version}.down.sql"
    if [[ ! -f "$down_file" ]]; then
        echo "Error: Rollback file '${down_file}' not found."
        exit 1
    fi

    echo "Rolling back: ${last_version}..."
    psql_cmd < "$down_file"
    psql_cmd -q -c "DELETE FROM schema_migrations WHERE version = '${last_version}';"
    echo "Rolled back: ${last_version}."
}

migrate_status() {
    init_table
    echo "Schema Migration Status:"
    psql_cmd -c "SELECT version, applied_at FROM schema_migrations ORDER BY version ASC;"
}

case "${1:-up}" in
    up)
        migrate_up
        ;;
    down)
        migrate_down
        ;;
    status)
        migrate_status
        ;;
    *)
        echo "Usage: $0 {up|down|status}"
        exit 1
        ;;
esac
