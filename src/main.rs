use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{
        interactions::{
            application_command::{ApplicationCommandInteraction, ApplicationCommandOptionType},
            Interaction,
        },
        prelude::Ready,
    },
    prelude::*,
};
use std::env;

fn get_option<T>(command: ApplicationCommandInteraction, i: usize) -> Option<Str> {
    let option = command.data.options.get(i);
    if let None = option {
        return None;
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.options.get(0)?.value.as_ref()?.as_str()? {
                Some(s) => s,
                _ => "Please specify message (this may be bug)".to_string(),
            };

            let ret = command
                .create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|data| data.content(content))
                })
                .await;

            if let Err(why) = ret {
                println!("Error sending response: {:?}", why);
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        for guild in ready.guilds {
            let name = guild.id().to_partial_guild(&ctx.http).await.unwrap().name;
            match guild
                .id()
                .create_application_command(&ctx.http, |command| {
                    command
                        .name("say")
                        .description("say specified message")
                        .create_option(|option| {
                            option
                                .kind(ApplicationCommandOptionType::String)
                                .name("message")
                                .required(true)
                                .description("message to say")
                        })
                })
                .await
            {
                Ok(command) => println!("Successfully set guild command in {}", name),
                Err(why) => println!("Error setting guild command in {}: {:?}", name, why),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    println!("User ID {}", application_id);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
