pub mod messages {
    pub mod troll_messages {
        use serenity::model::channel::Message;
        pub fn hello_message(msg: &Message) -> String {
            let reply = format!("<@{}>, 0/0/0? Lol. And you call yourself \"challenger\"? 
            Don't make me laugh.There's a reason Europe never wanted you. Because you're bad. 
            Your silver fanboys might like you, but I'd fuck you up on the rift. 
            I'm only plat and I already get much better scores. Drop your smug little smile, kid.", msg.author.id);
            reply
        }
    }
}
