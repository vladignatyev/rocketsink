#!/bin/bash

cargo build -r

ROCKET_LOG_LEVEL="critical" ROCKET_ADDRESS='127.0.0.1' ROCKET_WORKERS=64 ROCKET_PORT=8001 ROCKET_DATABASES='{redis={url="unix:///var/run/redis/redis.sock", min_connections=1024, max_connections=4096, connect_timeout=5, idle_timeout=120}}' target/release/eventsink &


wrk='wrk.method = "POST"
     wrk.headers["X-Test"] = "benchmark are fun! yeah"'
echo $wrk > /tmp/wrk.post.lua

wrk -s /tmp/wrk.post.lua -c800 -d30s -t16 http://localhost:8001/sink/teststream/ &