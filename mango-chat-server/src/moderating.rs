fn check_abuse(chat:&str) -> bool {
    false
}

fn check_spam(old_message:&mut Vec<String>,chat:&str,sender:&str) -> bool {
    let sender_message = format!("{}{}",sender,chat);
    if old_message.contains(&sender_message) {
        let index = old_message.iter().position(|x| *x==sender_message).unwrap();
        old_message.remove(index);
        old_message.push(sender_message);
        return false
    }
    if check_abuse(chat) {
        return false
    }
    old_message.push(sender_message);
    if old_message.len() > 10 {
        old_message.remove(0);
    }
    true
}
