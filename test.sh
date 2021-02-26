#!/bin/sh

ETEBASE_SERVER="${ETEBASE_SERVER:-http}"
ETEBASE_DEBUG="${ETEBASE_DEBUG:-true}"
ETEBASE_AUTO_UPDATE="${ETEBASE_AUTO_UPDATE:-true}"
ETEBASE_SUPER_EMAIL="${ETEBASE_SUPER_EMAIL:-test@localhost}"
ETEBASE_SUPER_USER="${ETEBASE_SUPER_USER:-test_user}"
ETEBASE_SUPER_PASS="${ETEBASE_SUPER_PASS:-SomePassword}"
ETEBASE_NAME="${ETEBASE_NAME:-etebase-rs-tests}"
ETEBASE_IMAGE="${ETEBASE_IMAGE:-victorrds/etebase:latest}"
ETEBASE_DJANGO_DEBUG="${ETEBASE_DJANGO_DEBUG:-true}"

echo_error() {
    echo "$@" 1>&2
}

die() {
    echo_error "$@"
    exit 1
}

exe_exists() {
    which "$1" &> /dev/null
}

docker=""

if exe_exists docker; then
    docker=docker
elif exe_exists podman; then
    docker=podman
else
    die "docker or podman is required to set up etebase for tests"
fi

start_container() {
    $docker run \
        -p 8033:3735 \
        -e SERVER="$ETEBASE_SERVER" \
        -e DEBUG="$ETEBASE_DEBUG" \
        -e AUTO_UPDATE="$ETEBASE_AUTO_UPDATE" \
        -e SUPER_USER="$ETEBASE_SUPER_USER" \
        -e SUPER_EMAIL="$ETEBASE_SUPER_EMAIL" \
        -e SUPER_PASS="$ETEBASE_SUPER_PASS" \
        -e DJANGO_DEBUG="$ETEBASE_DJANGO_DEBUG" \
        --name "$ETEBASE_NAME" --replace \
        --detach \
        "$ETEBASE_IMAGE"
}

stop_container() {
    $docker stop "$ETEBASE_NAME"
}

logs_container() {
    $docker logs -f "$ETEBASE_NAME"
}

enter_container() {
    $docker exec -it "$ETEBASE_NAME" bash
}

run_tests() {
    cargo test -- --test-threads=1
}

case "$1" in
    enter) enter_container ;;
    logs)  logs_container  ;;
    start) start_container ;;
    stop)  stop_container  ;;
    test)  run_tests ;;
    run)
        start_container
        run_tests
        stop_container
        ;;
esac
