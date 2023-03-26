#!/bin/bash

# WARNING!! use this script while running without docker.


export TRUNK_SERVE_PORT=8000
export ACTIX_PORT=3000

children=()

_term() {
    
    echo "Caught SIGTERM"
    for child in "${children[@]}"; do
        kill -TERM "$child" 2>/dev/null
    done
    docker kill $(docker compose ps -q postgres)
    docker kill $(docker compose ps -q redis)
}

_int() {
    echo "Caught SIGINT"
    for child in "${children[@]}"; do
        kill -TERM "$child" 2>/dev/null
    done
    docker kill $(docker compose ps -q postgres)
    docker kill $(docker compose ps -q redis)
}

trap _term SIGTERM
trap _int SIGINT

docker compose -f docker-compose.yaml up -d postgres
sleep 5
docker compose -f docker-compose.yaml up dbmate

docker compose -f docker-compose.yaml up -d redis
sleep 5

pushd backend;
cargo watch -x "run" &
ACTIX_PROC=$!
children+=($ACTIX_PROC)
popd;

pushd frontend;
npm start &
YEW_PROCESS=$!
children+=($YEW_PROCESS)
popd;

wait $ACTIX_PROC
echo "Done running actix and yew, bye"