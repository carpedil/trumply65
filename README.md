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

## Generate Entities

```bash


# 1. create new lib for entity
cargo new --lib entity
# 2. create .env file and set DATABASE_URL
# .env
DATABASE_URL="sqlite:./trumply65.db?mode=rwc"
# 3. run the script below one by one
sea-orm-cli migrate refresh -u "sqlite:./trumply65.db?mode=rwc"
sea-orm-cli generate entity -o entity/src -u "sqlite:./trumply65.db?mode=rwc"

# 4. delete lib.rs and rename mod.rs to lib.rs

# 5. add sea-orm/serde/async-graphql dependencies
cd .\entity\
cargo add sea-orm serde async-graphql

## serde must enable 'derive' feature
- serde = "1.0.204" x rewrite this line
+ serde = { version = "1.0.204", features = ["derive"] }

# 6. add [Serialize, Deserialize, SimpleObject] macro to Model
- #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
+ #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]

# 7. make async_graphql public in lib.rs 
-- snip --
+ pub use async_graphql::*;
```

# Write Services

1. cargo new --lib service
2. 添加依赖

   ```toml
   [dependencies]
   entity = {path ="../entity"}

   oracle = "0.6.2"
   serde = { version = "1.0.204", features = ["derive"] }
   async-graphql = "7.0.6"
   xlsxwriter = "0.6.1"

   [dependencies.sea-orm]
   version = "0.12.15" # sea-orm version
   features = [
       "debug-print",
       # "runtime-tokio-native-tls",
       "runtime-tokio-rustls",
       # "sqlx-postgres",
       # "sqlx-mysql",
       "sqlx-sqlite",
   ]
   ```
3. 按需创建子模块

   ```rust
   // lib.rs
   pub mod query;
   pub mod mutation;

   pub use query::*;
   pub use mutation::*;

   pub use sea_orm;
   ```
4. cargo new --lib common
5. 添加依赖

   ```toml
   [dependencies]
   entity = {path ="../entity"}

   serde = { version = "1.0.204", features = ["derive"] }
   async-graphql = "7.0.6"
   chrono = "0.4.38"
   xlsxwriter = "0.6.1"
   ```
6.
