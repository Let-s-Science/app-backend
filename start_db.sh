#!/bin/bash

docker network create letsscience-net

docker run -itd \
    --restart always \
    --net letsscience-net \
    -e POSTGRES_USER=letsscience \
    -e POSTGRES_PASSWORD=strong_password \
    -e POSTGRES_DB=letsscience \
    -p 5432:5432 \
    --name postgres \
    postgres:15;
