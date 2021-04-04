pub struct NameUser {
    pub name: String,
}

impl NameUser {
    pub fn new(s: String) -> Self {
        NameUser { name: s }
    }
}
