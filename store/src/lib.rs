pub struct Store {}

impl Store {
    pub fn create_user(&self) {
        println!("user created")
    }

    pub fn create_website(&self) -> String {
        String::from("1")
    }
}
