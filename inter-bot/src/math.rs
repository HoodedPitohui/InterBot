use serenity::model::channel::Message;

pub fn pemdas(msg: &Message) -> String {

    //clean out the whitespace
    let post: String = msg.content.chars().skip(10).collect();
    post
}
