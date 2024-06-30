use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Overrides {
    /// File containing the main() function. Defaults to src/main.c
    main: Option<String>,
}