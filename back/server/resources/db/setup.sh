#!/usr/bin/env bash
cd $(dirname "$0")

sqlite3 ../app.db < db.sql