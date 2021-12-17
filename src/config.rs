use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type ConfigMap = HashMap<String, Backend>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Backend {
    Postgres {
        host:     String,
        port:     u16,
        user:     String,
        password: String,
        database: String,
    },
    Redis {
        host:     String,
        port:     u16,
        password: String,
        database: u32,
    },
    Mysql {
        host:     String,
        port:     u16,
        user:     String,
        password: String,
        database: String,
    },
}
