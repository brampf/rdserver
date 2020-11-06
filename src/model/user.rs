
pub struct User {
    id: u32,
    name: String,
}

impl User {

    pub fn new(id: u32, name: String) -> Self {
        Self {
            name: name,
            id,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("User: {} ({})",self.name,self.id);
    }

}