#!/usr/bin/env bash

# DATABASE_URL=${DATABASE_URL} cargo sqlx migrate run
DATABASE_URL=${DATABASE_URL} cargo sqlx prepare
DATABASE_URL=${DATABASE_URL} ./nwc-api
