use serde::Serialize;

/// Module for handling encrypted commitment leaves API
pub mod encrypted_leaves;

/// Module for handle commitment leaves API
pub mod leaves;

/// Module for handling relayer metric API
pub mod metric;

/// Module for handling relayer info API
pub mod info;

// Unsupported feature response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UnsupportedFeature {
    message: String,
}
