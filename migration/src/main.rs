use sea_orm_migration::prelude::*;

mod utils;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
