use std::io::BufRead;

use sqlparser::ast::{SelectItem, SetExpr, Statement, TableFactor};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

fn main() {
    let dialect = PostgreSqlDialect {};
    let sql = "select * from tbl;";
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    match &ast[0] {
        Statement::Query(query) => {
            assert!(query.with.is_none(), "todo");
            assert!(query.order_by.is_empty(), "todo");
            assert!(query.limit.is_none(), "todo");
            assert!(query.offset.is_none(), "todo");
            assert!(query.fetch.is_none(), "not supported");
            match &query.body {
                SetExpr::Select(select) => {
                    assert!(!select.distinct, "todo");
                    assert!(select.top.is_none(), "not supported");
                    assert!(select.lateral_views.is_empty(), "not supported");
                    assert!(select.selection.is_none(), "todo");
                    assert!(select.group_by.is_empty(), "todo");
                    assert!(select.cluster_by.is_empty(), "not supported");
                    assert!(select.distribute_by.is_empty(), "not supported");
                    assert!(select.sort_by.is_empty(), "not supported");
                    assert!(select.having.is_none(), "todo");

                    assert_eq!(select.projection.len(), 1, "todo");
                    assert_eq!(select.projection[0], SelectItem::Wildcard, "todo");
                    assert_eq!(select.from.len(), 1, "todo");
                    let table_with_joins = &select.from[0];
                    assert!(table_with_joins.joins.is_empty(), "todo");
                    match &table_with_joins.relation {
                        TableFactor::Table {
                            name,
                            alias,
                            args,
                            with_hints,
                        } => {
                            assert!(alias.is_none(), "todo");
                            assert!(args.is_empty(), "not supported");
                            assert!(with_hints.is_empty(), "not supported");
                            assert_eq!(name.0.len(), 1, "not supported");
                            let table_name = &name.0[0].value;
                            let file =
                                std::fs::File::open(format!("db/{}.ndjson", table_name)).unwrap();
                            let buf_rdr = std::io::BufReader::new(file);
                            for line in buf_rdr.lines() {
                                let line = line.unwrap();
                                let tuple: Vec<serde_json::Value> =
                                    serde_json::from_str(&line).unwrap();
                                println!("{:?}", tuple);
                            }
                        }
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
        _ => todo!(),
    }
}
