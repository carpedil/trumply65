# 项目启动

```bash
# 65岁，正式闯的年纪
cargo new --bin trumply65
```

## Setup Workspace

```toml
[workspace]
members =['.']
```

## Setup Migration

```toml

## 1. create new lib for migration
cargo new --lib migration
# 2. run sea-orm-cli migrate init
sea-orm-cli migrate init
# 3. enable sea-orm-migration
[dependencies.sea-orm-migration]
version = "0.12.0"
features = [
  # Enable at least one `ASYNC_RUNTIME` and `DATABASE_DRIVER` feature if you want to run migration via CLI.
  # View the list of supported features at https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime.
  # e.g.
  "runtime-tokio-rustls",  # `ASYNC_RUNTIME` feature
  "sqlx-sqlite",         # `DATABASE_DRIVER` feature
]

# 4. Define your Migration Object [m2024XXX.rs,lib.rs]
```
