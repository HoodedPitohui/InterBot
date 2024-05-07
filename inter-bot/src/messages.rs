pub mod troll_messages {
    use serenity::model::channel::Message;
    use serenity::model::guild::Guild;

    use rand::Rng;
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
    pub fn pepe_spam(guild: &Guild) -> String {

        //find the emojis that have pepe in it on the server
        let emotes = &guild.emojis;
        for (_key, value) in emotes {
            let emoji_val = value;
            return emoji_val.name.clone();
        }
        return String::from("hello");
    }

    pub fn king_troll_message(msg: &Message) -> String {
        //return a massively troll message
        let mut post: String = msg.content.chars().skip(4).collect();
        let mut temp = post.trim().split(" ");

        //get the [preferably name] of the person that is sent to the function
        post = temp.next().unwrap_or("No substring found").to_string();
        let addition = String::from("is a");

        //add an intermediate line
        let intermediate = vec!["total".to_string(), "unadulterated".to_string(), "half".to_string(), "omega".to_string(),
                                "mega".to_string(), "feeding".to_string()];

        //insult list
        let comments = vec!["clown!".to_string(), "idiot!".to_string(), "inter!".to_string(),
                                        "troll!".to_string(), "muppet!".to_string(), "yasuo main!".to_string(),
                                        "genius!".to_string(), "pillock!".to_string(), "bugger".to_string(),
                                        "arsehole".to_string(), "lint licker!".to_string(), "pompous worm!".to_string(),
                                        "fat lard!".to_string()];
        
        //ping a random insult from the insult list
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..intermediate.len());
        let chosen_intermediate = &intermediate[num];
        let num = rng.gen_range(0..comments.len()); 
        let chosen_insult = &comments[num];

        //combine the messages
        let ret_string = format!("{} {} {} {}", post, addition, chosen_intermediate, chosen_insult);
        ret_string
    }
}