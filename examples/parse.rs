#![warn(clippy::all)]

use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::*;

fn main() {
    let sql = "SELECT a, b, 123, myfunc(b) \
               FROM table_1 \
               WHERE a > b AND b < 100 \
               ORDER BY a DESC, b";

    let dialect = PostgreSqlDialect {};

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("AST: {ast:?}");
}
