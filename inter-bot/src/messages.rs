pub mod troll_messages {
    use serenity::model::channel::Message;
    use serenity::model::id::GuildId;
    use serenity::http::Http;
    use serenity::model::id::UserId;



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

    pub async fn pepe_spam(guild_id: &GuildId, http: &Http) -> String {
        let emotes = match guild_id.emojis(&http).await {
            Ok(emojis) => emojis,
            Err(_e) => return String::from("no emotes found"),
        };
    
        
        //find the pepe emotes -> need name and id
        let mut pepe_names = Vec::new();
        let mut pepe_ids = Vec::new();

        //search all of the emotes -> time intensive and might need to be optimized later
        for emo in &emotes {
            if emo.name.to_lowercase().contains("pepe") 
                || emo.name.to_lowercase().contains("peep") 
                || emo.name.to_lowercase().contains("eege") {
                pepe_names.push(emo.name.clone());
                pepe_ids.push(emo.id.to_string());
            }
        }
        let mut reply = String::new();

        //fencepost so that there is no trailing line
        for e in 0..pepe_names.len() - 1 {
            let add_string = format!("<:{}:{}> \n", pepe_names[e], pepe_ids[e]);
            reply.push_str(&add_string);
        }
        let add_string = format!("<:{}:{}>", pepe_names[pepe_names.len() - 1], pepe_ids[pepe_names.len() - 1]);
        reply.push_str(&add_string);

        //return reply
        reply
    }
    
    pub async fn ping_spam(msg: &Message, guild_id: &GuildId, http: &Http) -> String {
        //this is supposed to let users to specify a username, and then a number
        //the bot will then ping the user that many times
        let post: String = msg.content.chars().skip(12).collect();
        let parts = post.trim().split(" ");
        let part_vec = parts.collect::<Vec<&str>>();

        //check if there are more than two arguments entered
        //otherwise default to 10 pings
        let mut pings = 10;
        if part_vec.len() > 2 {
            return String::from("You have entered in too many arguments!");
        }

        //0 arguments after = not viable ping
        else if part_vec.len() == 0 {
            return String::from("You have not given me a user to ping!");
        }

        //this means that a number of pings has been specified = 2nd position
        else if part_vec.len() == 2 {
            //make sure it's an integer
            let num = part_vec[1].to_string().parse::<i64>();
            match num {
                Ok(_val) => pings = num.unwrap(),
                Err(_why) => return String::from("You have not entered in an integer number of pings!"),
            }
        }

        let mut reply = String::new();

        if pings > 50 {
            pings = 50;
            let add_string = String::from("I can send a maximum of 50 pings at once!");
            reply.push_str(&add_string);
        }


        //check if the first part does have a viable ping
        let mut user_id: Vec<_> = part_vec[0].to_string().chars().skip(2).collect();

        //last letter should be >
        let last_char = part_vec[0].to_string().chars().last().unwrap(); //this should always work        
        user_id = user_id[0..user_id.len() - last_char.len_utf8()].to_vec();
        
        let user_f64 = match user_id.into_iter().collect::<String>().parse::<u64>() {
            Ok(user_f64) => user_f64,
            Err(_e) => return String::from("A viable user was not found")
        };

        //create ping id
        let ping_id = UserId::new(user_f64);

        //check if user is in server
        let _ping_member = match guild_id.member(&http, ping_id).await {
            Ok(ping_member) => ping_member,
            Err(_e) => return String::from("This person is not in the server!"),
        };  

        //viable user found
        for _p in 0..pings {
            let add_str = format!("<@!{}> ", user_f64);
            reply.push_str(&add_str);
        }
        reply
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