use unicase::UniCase;

pub type Key = UniCase<String>;

pub trait ToKey {
    fn to_key(self) -> Key;
}

impl ToKey for String {
    fn to_key(self) -> Key {
        UniCase::new(self)
    }
}

impl ToKey for &str {
    fn to_key(self) -> Key {
        UniCase::new(self.to_string())
    }
}

impl ToKey for UniCase<String> {
    fn to_key(self) -> Key {
        self
    }
}

impl ToKey for &UniCase<String> {
    fn to_key(self) -> Key {
        self.clone()
    }
}
