#!/bin/bash

docker-compose up -d 

until docker-compose exec postgres pg-is-ready; do
  sleep 1
done

sqlx migrate run

cargo build

cargo run