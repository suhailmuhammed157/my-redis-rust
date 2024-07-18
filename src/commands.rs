#[derive(Debug)]
pub enum Command {
    Get,
    Set,
    Invalid,
}

impl Command {
    pub fn get_command(first_array_item: &String) -> Command {
        match first_array_item.as_str() {
            "get" => Command::Get,
            "set" => Command::Set,
            _ => Command::Invalid,
        }
    }
}
