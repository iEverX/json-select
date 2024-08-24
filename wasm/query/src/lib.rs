use std::thread::sleep;
use anyhow;
use serde_json::{Value, Map};

pub fn query_str(full: &str, select: &str) -> anyhow::Result<String> {
    let full_json: Value = serde_json::from_str(full)?;
    let select_json: Value = serde_json::from_str(select)?;
    let value = do_query(&full_json, &select_json);
    Ok(serde_json::to_string_pretty(&value)?)
}

pub fn do_query(full: &Value, select: &Value) -> Value {
    let full_clone = select.is_boolean() && select.as_bool().unwrap()
        || select.is_i64() && select.as_i64().unwrap() > 0;

    if full_clone {
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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn query_str_simple() {
        let full = r#"
        {
            "cities": [
                {
                    "name": "AA",
                    "code": 1
                },
                {
                    "name": "BB",
                    "code": 2
                }
            ],
            "data": "content",
            "value": 1901
        }"#;

        let query = r#"
        {
            "cities": {
                "name": true
            },
            "value": 1
        }"#;

        let result = query_str(full, query);
        if result.is_err() {
           eprintln!("{:?}", result.err()) ;
            return;
        }
        assert!(result.is_ok());

    let expected = r#"{
  "cities": [
    {
      "name": "AA"
    },
    {
      "name": "BB"
    }
  ],
  "value": 1901
}"#;

        assert_eq!(result.unwrap(), expected);

    }
}