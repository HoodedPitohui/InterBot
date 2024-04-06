use serenity::model::channel::Message;

pub fn pemdas(msg: &Message) -> String {

    //clean out the whitespace
    let mut post: String = msg.content.chars().skip(10).collect();
    post = post.trim().replace(' ', "").to_string();
    post
}
