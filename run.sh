#!/usr/bin/env sh
set -eu

if [ ! -f "leptos-icons/Cargo.toml" ]; then
    echo "O submódulo leptos-icons não está inicializado." >&2
    echo "Execute: git submodule update --init --recursive" >&2
    exit 1
fi

export LEPTOS_TAILWIND_VERSION=${LEPTOS_TAILWIND_VERSION:-v3.4.17}
exec cargo leptos watch "$@"
