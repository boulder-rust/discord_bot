_default:
    @just --list --unsorted

# Enter the development environment
work:
    @nix develop

# Delete all traces of the database
delete-db:
    @rm -rf ./postgres
    @rm -rf ./postgres_data

# Create the database
init-db:
    @psql postgres -c "create database $PGDATABASE"
