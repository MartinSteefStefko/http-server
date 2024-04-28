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
            let decoded_values = percent_decode(value.as_bytes())
                .decode_utf8_lossy()
                .replace('+', " ")
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            // Handling both single and multiple values
            if decoded_values.len() == 1 {
                match data.entry(decoded_key) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let entry_value = entry.get_mut();
                        match entry_value {
                            Value::Single(prev) => {
                                *entry_value =
                                    Value::Multiple(vec![prev.clone(), decoded_values[0].clone()]);
                            }
                            Value::Multiple(vec) => vec.push(decoded_values[0].clone()),
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(Value::Single(decoded_values[0].clone()));
                    }
                }
            } else if decoded_values.len() > 1 {
                match data.entry(decoded_key) {
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let entry_value = entry.get_mut();
                        match entry_value {
                            Value::Single(prev) => {
                                let mut new_vec = vec![prev.clone()];
                                new_vec.extend(decoded_values);
                                *entry_value = Value::Multiple(new_vec);
                            }
                            Value::Multiple(vec) => vec.extend(decoded_values),
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(Value::Multiple(decoded_values));
                    }
                }
            }
        }

        QueryString { data }
    }
}
