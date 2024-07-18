pub fn buffer_to_array(buffer: Vec<u8>) -> Vec<String> {
    let mut message_in_words = String::new();
    for i in buffer {
        let char = i as char;
        message_in_words.push(char);
    }
    let commands: Vec<String> = message_in_words.split(" ").map(|s| s.to_string()).collect();
    commands
}
