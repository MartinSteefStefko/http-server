use percent_encoding::percent_decode;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QueryString {
    data: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Single(String),
    Multiple(Vec<String>),
}

impl QueryString {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// /user?types=Tap+Water&favorite=true&x=8&y=&z===9&z=10

impl From<&str> for QueryString {
    fn from(s: &str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut parts = sub_str.splitn(2, '=');
            let key = parts.next().unwrap_or_default();
            let value = parts.next().unwrap_or_default();

            let decoded_key = percent_decode(key.as_bytes())
                .decode_utf8_lossy()
                .replace('+', " ");
            let decoded_value = percent_decode(value.as_bytes())
                .decode_utf8_lossy()
                .replace('+', " ");

            match data.entry(decoded_key) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    match entry.get_mut() {
                        Value::Single(prev) => {
                            if decoded_value.is_empty() && !prev.is_empty() {
                                // Only update if new value has meaningful data
                                *entry.get_mut() =
                                    Value::Multiple(vec![prev.clone(), decoded_value.clone()]);
                            } else {
                                *entry.get_mut() =
                                    Value::Multiple(vec![prev.clone(), decoded_value]);
                            }
                        }
                        Value::Multiple(vec) => {
                            if !decoded_value.is_empty() {
                                vec.push(decoded_value);
                            }
                        }
                    }
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(Value::Single(decoded_value));
                }
            }
        }

        QueryString { data }
    }
}
