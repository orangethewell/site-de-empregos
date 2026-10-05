# Vagas e Maraxá

Aplicação Flask com frontends Yew compilados para WebAssembly. O caminho local
não depende de Docker, Redis ou subdomínios: o site é servido em
`http://127.0.0.1:5000` e o painel em `http://127.0.0.1:5000/admin/`.

## Requisitos

- Python 3.11+
- Rust e `rustup`
- Trunk (`cargo install trunk`)

## Primeira execução

Na pasta `project/`:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
uv venv .venv
uv pip install -r requirements.txt
./run.sh
```

Os dois frontends fixam o `wasm-bindgen` do Trunk em `0.2.129`, igual à
versão usada pelo workspace Rust.

Sem `uv`, use `python3 -m venv .venv` e
`.venv/bin/python -m pip install -r requirements.txt`.

O banco SQLite é criado em `app/app.db`. O usuário administrativo inicial é
`admin@localhost`, com senha `admin`; troque essa senha antes de qualquer uso
real.

## Execução manual

O script `run.sh` compila os dois frontends e inicia o Flask. Para iniciar
apenas a API, depois de instalar as dependências:

```sh
SECRET_KEY=dev-only-change-me .venv/bin/python -m flask --app app run --debug
```
