use serenity::model::channel::Message;

pub fn pemdas(msg: &Message) -> String {

    //clean out the whitespace
    let clean_string: String = msg.content.chars().filter(|c| !c.is_whitespace()).collect();
    clean_string

}
