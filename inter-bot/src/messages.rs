pub mod troll_messages {
    use serenity::model::channel::Message;
    use std::rand::{task_rng, Rng};
    pub fn hello_message(msg: &Message) -> String {
        let reply = format!("<@{}>, 0/0/0? Lol. And you call yourself \"challenger\"? 
        Don't make me laugh.There's a reason Europe never wanted you. Because you're bad. 
        Your silver fanboys might like you, but I'd fuck you up on the rift. 
        I'm only plat and I already get much better scores. Drop your smug little smile, kid.", msg.author.id);
        reply
    }
    pub fn gleb_message() -> String {

        //for sebastian
        let reply = String::from("<:pepeLaugh:798083157667610653>");
        println!("{}", reply);
        reply
    }
    pub fn king_troll_message(msg: &Message) -> String {
        //return a massively troll message
        let mut post: String = msg.content.chars().skip(4).collect();
        let mut temp = post.split(" ");

        //get the [preferably name] of the person that is sent to the function
        post = temp.next().unwrap_or("No substring found").to_string();
        let addition = String::from(" is a ");

        //insult list
        let comments = vec!["clown!".to_string(), "idiot!".to_string(), "inter!".to_string(),
                                        "troll!".to_string(), "muppet!".to_string(), "yasuo main!".to_string(),
                                        "genius!".to_string()];
        let num: f64 = task_rng().gen_range(0, comments.len()); 
        let chosen = &comments[num];

        let ret_string = format!("{} {} {}", post, addition, chosen);
        ret_string
    }
}