use serde_json::json;

use crate::utils;

/**
 * JSON Test
 */
pub fn json_test() {
    println!("{:#}\n", json!({
        "type": "message",
        "content": "Hi there!",
        "createdAt": utils::epoch::ms(),
    }));
}
