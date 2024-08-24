use std::thread::sleep;
use anyhow::{anyhow, Error};
use serde_json::{Value, Map, json, Number};
use serde_json::Value::Object;
use serde::{Deserialize, Deserializer, Serialize, Serializer};


pub fn query_str(full: &str, select: &str) -> anyhow::Result<String> {
    let config = QueryConfig { pretty: true };
    query_str_with_config(full, select, config)
}

struct QueryConfig {
    pretty: bool,
}

fn query_str_with_config(full: &str, select: &str, query_config: QueryConfig) -> anyhow::Result<String> {
    let full_json: Value = serde_json::from_str(full)?;
    let select_json  = serde_json::from_str(select)?;
    let value = do_query(&full_json, &select_json)?;

    if query_config.pretty {
        Ok(serde_json::to_string_pretty(&value)?)
    } else {
        Ok(serde_json::to_string(&value)?)
    }
}

fn do_query(full: &Value, select: &Value) -> anyhow::Result<Option<Value>> {
    let result = match select {
        Value::Bool(x) => {
            if *x {
                Ok(Some(full.clone()))
            } else {
                Ok(None)
            }
        }
        Value::Number(x) => {
            if x.is_i64() && x.as_i64().unwrap() > 0 {
                Ok(Some(full.clone()))
            } else {
                Ok(None)
            }
        }
        Object(x) => {
            if full.is_array() {
                let mut arr = Vec::<Value>::new();
                for value in full.as_array().unwrap() {
                    if let Some(r) = do_query(value, select)? {
                        arr.push(r);
                    }
                }
                Ok(Some(Value::Array(arr)))
            } else if full.is_object() {
                let mut m = Map::<String, Value>::new();

                let kvs = select.as_object().unwrap();
                for (k, v) in kvs {
                    if let Some(value) = full.get(k) {
                        if let Some(r) = do_query(value, v)? {
                            m.insert(k.clone(), r);
                        }
                    }
                }
                Ok(Some(Object(m)))
            } else {
                Err(anyhow!("object select with non-object json field"))
            }
        }
        Value::Null => {
            Err(anyhow!("null select"))
        }
        Value::String(x) => {
            Err(anyhow!("string select {}", x))
        }
        Value::Array(_) => {
            Err(anyhow!("array select"))
        }
    };
    result
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
            eprintln!("{:?}", result.err());
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

    #[test]
    fn query_str_with_zero() {
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
            "value": {
                "a": 1,
                "b": 2
            }
        }"#;

        let query = r#"
        {
            "cities": {
                "name": 0
            },
            "value": 1
        }"#;

        let result = query_str(full, query);
        if result.is_err() {
            eprintln!("====================== {:?}", result.err());
            return;
        }
        assert!(result.is_ok());

        let expected = r#"{
  "cities": [
    {},
    {}
  ],
  "value": {
    "a": 1,
    "b": 2
  }
}"#;

        assert_eq!(result.unwrap(), expected);
    }
}