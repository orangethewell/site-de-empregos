#! /bin/sh

cd frontend/www
$HOME/.cargo/bin/trunk build --dist ../../dist 

cd ../admin
$HOME/.cargo/bin/trunk build --dist ../../dist/admin --public-url /admin/ 

cd ../../

export COMPOSE_PROFILES=dev 
docker compose up --build