pub struct UserAgentList {
    include_pc: bool,
    include_phone: bool
}

impl UserAgentList {
    pub fn new() -> Self {
        UserAgentList {
            include_pc: false,
            include_phone: false
        }
    }
    pub fn pc(self) -> Self {
        UserAgentList {
            include_pc: true,
            include_phone: self.include_phone
        }
    }
    pub fn phone(self) -> Self {
        UserAgentList {
            include_pc: self.include_pc,
            include_phone: true
        }
    }
    pub fn no_pc(self) -> Self {
        UserAgentList {
            include_pc: false,
            include_phone: self.include_phone
        }
    }
    pub fn no_phone(self) -> Self {
        UserAgentList {
            include_pc: self.include_pc,
            include_phone: false
        }
    }
    pub fn get_random(&self) -> String {
        "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Mobile Safari/537.36".to_owned()
    }
}