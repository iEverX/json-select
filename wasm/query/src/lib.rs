use anyhow;
use serde_json::{Value, Map};

pub fn query_str(full: &str, select: &str) -> anyhow::Result<String> {
    let full_json: Value = serde_json::from_str(full)?;
    let select_json: Value = serde_json::from_str(select)?;
    let value = do_query(&full_json, &select_json);
    Ok(serde_json::to_string_pretty(&value)?)
}

pub fn do_query(full: &Value, select: &Value) -> Value {
    if select.is_boolean() && select.as_bool().unwrap() {
        full.clone()
    } else if full.is_array() {
        let mut arr = Vec::<Value>::new();
        for value in full.as_array().unwrap() {
            let r = do_query(value, select);
            arr.push(r);
        }
        Value::Array(arr)
    } else {
        let mut m = Map::<String, Value>::new();

        let kvs = select.as_object().unwrap();
        for (k, v) in kvs {
            if let Some(value) = full.get(k) {
                m.insert(k.clone(), do_query(value, v));
            }
        }
        Value::Object(m)
    }
}