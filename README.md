# Vagas em Araxá

Aplicação Leptos com servidor Actix, SQLite e frontend WASM.

## Rodar sem Docker

Requisitos:

- Rust via `rustup`;
- `cargo-leptos`;
- SQLite acessível pela variável `DATABASE_URL`;
- variáveis `SECRET_KEY`, `SMTP_MAIL` e `SMTP_PASSWORD` no `.env`.

O banco SQLite é um arquivo local e não precisa de serviço ou Docker. A configuração padrão é:

```dotenv
DATABASE_URL="sqlite://vagasemaraxa.db?mode=rwc"
```

O processo cria o arquivo e executa as migrations na inicialização.

Inicialize o submódulo de ícones e instale o alvo WASM:

```sh
git submodule update --init --recursive
rustup target add wasm32-unknown-unknown --toolchain nightly-2025-03-01
cargo install cargo-leptos --version 0.3.11
```

Depois, na raiz do projeto:

```sh
./run.sh
```

A aplicação fica em http://127.0.0.1:3000. Redis não é necessário: as sessões usam cookies assinados.

Para apenas compilar:

```sh
LEPTOS_TAILWIND_VERSION=v3.4.17 cargo leptos build
```
