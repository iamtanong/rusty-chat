#!/bin/bash

# Clean up
rm -r data

# Start the database
docker-compose up -d 

# Wait for the database to be ready
until docker-compose exec postgres pg-is-ready; do
  sleep 1
done

# Run the migrations
sqlx migrate run


cargo build

cargo run