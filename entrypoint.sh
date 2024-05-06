#!/usr/bin/env bash

DATABASE_URL=${DATABASE_URL} sqlx migrate run
DATABASE_URL=${DATABASE_URL} ./nwc-api
