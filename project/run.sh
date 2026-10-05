#!/usr/bin/env sh
set -eu

# Trunk 0.21 expects this flag as a boolean; some shells export NO_COLOR=1.
unset NO_COLOR

PROJECT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
TRUNK_BIN=${TRUNK_BIN:-"$HOME/.cargo/bin/trunk"}
PYTHON_BIN=${PYTHON_BIN:-"$PROJECT_DIR/.venv/bin/python"}

if [ ! -x "$TRUNK_BIN" ]; then
    echo "Trunk não encontrado em $TRUNK_BIN. Instale-o com: cargo install trunk --version 0.21.14 --locked" >&2
    exit 1
fi

if [ ! -x "$PYTHON_BIN" ]; then
    echo "Ambiente Python não encontrado em $PYTHON_BIN. Crie-o com:" >&2
    echo "  uv venv .venv && uv pip install -r requirements.txt" >&2
    exit 1
fi

cd "$PROJECT_DIR/frontend/www"
export TRUNK_TOOLS_WASM_BINDGEN=${TRUNK_TOOLS_WASM_BINDGEN:-0.2.129}
"$TRUNK_BIN" build --dist ../../dist

cd ../admin
"$TRUNK_BIN" build --dist ../../dist/admin --public-url /admin/

cd "$PROJECT_DIR"
export APP_ENV=${APP_ENV:-development}
export SECRET_KEY=${SECRET_KEY:-dev-only-change-me}
exec "$PYTHON_BIN" -m flask --app app run --debug --host "${FLASK_HOST:-127.0.0.1}" --port "${PORT:-5000}"
