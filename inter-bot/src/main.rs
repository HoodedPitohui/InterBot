use anyhow::Context as _;
use serenity::{async_trait, model};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use serenity::utils::MessageBuilder;
use inter_bot::messages::troll_messages;
use inter_bot::math;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "int!help" {
            let res_str = MessageBuilder::new() 
                .push("The commands I currently have are: \n")
                .build();
            if let Err(why) = msg.channel_id.say(&ctx.http, &res_str).await {
                println!("Error sending message: {why:?}");
            }
        }
        //All the troll messages
        if msg.content.to_lowercase() == "int!hello" {
            if let Err(why) = msg.channel_id.say(&ctx.http, &troll_messages::hello_message(&msg)).await {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content.to_lowercase() == "int!gleb" {
            if let Err(e) = msg.channel_id.say(&ctx.http, &troll_messages::gleb_message()).await {
                error!("Error sending message: {:?}", e);
            }
        }
        
        //utility functions
        if msg.content.to_lowercase() == "int!pemdas" {
            if let Err(e) = msg.channel_id.say(&ctx.http, &math::pemdas(&msg)).await {
                println!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
