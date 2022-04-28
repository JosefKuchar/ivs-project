//! Layer between frontend and backend
//!
//! Handles calls from frontend and returns operation result.

use crate::mathlib::*;
use dec::Context;
use serde::Deserialize;
use tauri::command;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
/// Request JSON structure
pub struct RequestBody {
  /// First operand
  a: String,
  /// Second operand (only used in some operations)
  b: Option<String>,
  /// Operation name (listed in mathlib)
  operation: String,
}

/// Number parsing wrapper
///
/// # Arguments
/// * `string` - Number in string to be parsed
fn handle_parse(string: String) -> Result<Dec, String> {
  match string.parse() {
    Ok(num) => Ok(num),
    Err(_) => Err(format!("Couldn't parse number: {}", string).into()),
  }
}

/// Command handler
///
/// This gets called when frontend calls invoke via api.
/// Returns operation result as string.
///
/// # Arguments
/// * `payload` - JSON payload from frontend
#[command]
pub fn math_operation(payload: RequestBody) -> Result<String, String> {
  // Parse first operand
  let a: Dec = handle_parse(payload.a)?;

  // Execute operation
  let result = match payload.b {
    // Functions with two arguments
    Some(b_string) => {
      // Parse second operand
      let b: Dec = handle_parse(b_string)?;
      match payload.operation.as_str() {
        "add" => Ok(add(a, b)),
        "subtract" => Ok(subtract(a, b)),
        "multiply" => Ok(multiply(a, b)),
        "divide" => divide(a, b),
        "pow" => pow(a, b),
        "root" => root(a, b),
        op => Err(format!("Invalid operation: {}", op).into()),
      }
    }
    // Functions with one argument
    None => match payload.operation.as_str() {
      "factorial" => factorial(a),
      "abs" => Ok(abs(a)),
      op => Err(format!("Invalid operation: {}", op).into()),
    },
  };

  // Return final result
  match result {
    Ok(num) => Ok(num.to_string()),
    Err(msg) => Err(msg),
  }
}
