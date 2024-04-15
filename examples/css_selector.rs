use anyhow::Result;
use scrappy::query;
use sqlparser::{dialect::GenericDialect, parser::Parser};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://www.google.com";

    // 使用 sql 从 URL 里获取数据
    let sql = format!(
        "SELECT css_selector('div.gb_n') \
        FROM {}",
        url
    );
    let df1 = query(sql).await?;
    println!("{:?}", df1);

    Ok(())
}
