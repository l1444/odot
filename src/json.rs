pub mod json {
    use serde::{Serialize};

    // il faut que toutes les structs supportent..
    pub fn parse<T>(obj: &T) -> String
    where T: ?Sized + Serialize {
        return serde_json::to_string(&obj).unwrap()
    }

    pub fn unparse<'a, T>(str: &'a str) -> T
    where T: serde::de::Deserialize<'a> {
        return serde_json::from_str(&*str).unwrap()
    }
}