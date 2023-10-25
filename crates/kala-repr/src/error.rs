pub struct Error {
    pub message: String,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        self.message.clone()
    }
}