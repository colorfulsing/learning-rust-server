use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(text: &'buf str) -> Self {
        let mut data = HashMap::new();

        for key_pair in text.split('&') {
            let mut key = key_pair;
            let  mut value = "";
            if let Some(i) = key_pair.find('=') {
                key = &key_pair[..i];
                value = &key_pair[i + 1..];
            }

            data.entry(key).and_modify(|old| {
                match old {
                    Value::Single(first_value) => {
                        *old = Value::Multiple(vec![first_value, value])
                    },
                    Value::Multiple(vec) => vec.push(value)
                }
            }).or_insert(Value::Single(value));
        }

        QueryString{data}
    }
}