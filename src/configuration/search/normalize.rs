pub trait Normalize<T> {
    fn normalize(&self) -> T;
}

impl Normalize<String> for String {
    fn normalize(&self) -> String {
        self.split_whitespace().map(|word| {
                match word {
                    "the" | "of" | "to" => word.to_string(),
                    _ => {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().to_string() + chars.as_str()
                        }
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}