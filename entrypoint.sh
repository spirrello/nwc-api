#!/usr/bin/env bash

DATABASE_URL=${DATABASE_URL} cargo sqlx prepare
DATABASE_URL=${DATABASE_URL} ./nwc-api
