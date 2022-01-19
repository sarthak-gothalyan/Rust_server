// Imports
use std::collections::HashMap;

// Value enum
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

// Query string struct
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        // Create new hash map
        let mut data = HashMap::new();

        // Loop over tokens
        for sub in s.split('&') {
            let mut key = sub;
            let mut value = "";

            // Valid key value exist
            if let Some(i) = sub.find('=') {
                key = &sub[..i];
                value = &sub[i+1..];
            }

            // Add key value
            data.entry(key)
                // Make multiple values for this key
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => *existing = Value::Multiple(vec![prev_val, value]),
                    Value::Multiple(vec) => vec.push(value)
                })
                // make single value for this key
                .or_insert(Value::Single(value));
        }

        // Return query string struct
        QueryString {data}
    }
}