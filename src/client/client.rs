#![warn(clippy::str_to_string)]

use crate::{client::commands, server::{classes::{fasel::FaselSearcher, netflix::{Netflix, NetflixSearcher}}, functions::quit_browser}, Streamer};

use poise::serenity_prelude as serenity;
use ::serenity::all::{ButtonStyle, ComponentInteractionDataKind, CreateActionRow, CreateButton, CreateEmbed, CreateSelectMenu, CreateSelectMenuOption, EditMessage, GuildId};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;


pub struct Data {
    pub votes: Mutex<HashMap<String, u32>>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    
    
    
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }

        serenity::FullEvent::InteractionCreate { interaction } => {

            if let Some(component_interaction) = interaction.clone().message_component() {
                
                if component_interaction.data.custom_id.starts_with("movieSelect:") {
                    if let ComponentInteractionDataKind::StringSelect { ref values } = component_interaction.data.kind {
                        if let Some(selected_movie) = values.get(0) {
                            println!("User selected: {}", selected_movie);
                            if let Err(err) = component_interaction.defer(&_ctx.http).await {
                                eprintln!("Failed to defer interaction: {:?}", err);
                                return Ok(()); 
                            }
                            if let Some(channel) = component_interaction.channel_id.to_channel(&_ctx.http).await.ok().and_then(|c| c.guild()) {
                                if let Ok(mut message) = channel.message(&_ctx.http, component_interaction.message.id).await {
                                    let loading_embed = CreateEmbed::new()
                                        .title("Loading...")
                                        .description("Please wait while we fetch the movie.");
                
                                    if let Err(err) = message.edit(&_ctx.http, EditMessage::new().embed(loading_embed).components(Vec::new()).remove_all_attachments()).await {
                                        eprintln!("Failed to send loading message: {:?}", err);
                                        return Ok(()); 
                                    }
                                }
                            }
                            
                            let streaming_service = component_interaction.data.custom_id.strip_prefix("movieSelect:").unwrap_or("Unknown");
                            println!("{:?}", streaming_service);

                            if streaming_service == "Fasel" {
                                let original_data = FaselSearcher::_search(selected_movie).await;
                                let movie_data = match original_data {
                                    Ok(data) => data,
                                    Err(_) => return Ok(()),
                                };
                                if let Err(err) = Streamer::start(&movie_data[0].0.to_string(), streaming_service, "1421137405158756543").await {
                                    eprintln!("Streamer Failed: {:?}", err)
                                }
                            } else if streaming_service == "Netflix" {
                                let original_data = NetflixSearcher::search(selected_movie).await;
                                let movie_data = match original_data {
                                    Ok(data) => data,
                                    Err(_) => return Ok(()),
                                };
                                if let Some(show_result) = movie_data.get(0) {
                                    if show_result.3.is_show {
                                        let season_embed = CreateEmbed::new()
                                            .title("Select a Season")
                                            .description("Choose an season from the dropdown menu.");
                            
                                        let mut options: Vec<CreateSelectMenuOption> = Vec::new();
                            
                                        if let Some(seasons) = show_result.3.show_data.get("seasons")
                                        .and_then(|s| s.get("edges"))
                                        .and_then(|e| e.as_array())
                                    {
                                        for season in seasons {
                                            if let Some(season_node) = season.get("node") {
                                                let season_title = season_node.get("title")
                                                    .and_then(|t| t.as_str())
                                                    .unwrap_or("Unknown Season");
                                    
                                                let season_id = season_node.get("videoId")
                                                .and_then(|id| id.as_u64())
                                                .map(|id| id.to_string())
                                                .unwrap_or_else(|| "Unknown".to_string());
                                                options.push(
                                                    CreateSelectMenuOption::new(
                                                        season_title.to_string(), 
                                                        season_id
                                                    )
                                                );
                                            }
                                        }
                                    }

                                        
                                        if let Some(channel) = component_interaction.channel_id.to_channel(&_ctx.http).await.ok().and_then(|c| c.guild()) {
                                            if let Ok(mut message) = channel.message(&_ctx.http, component_interaction.message.id).await {
                            
                                                let select_menu = CreateSelectMenu::new(
                                                    "seasonSelect",
                                                    serenity::all::CreateSelectMenuKind::String { options },
                                                )
                                                .placeholder("Select a season");
                                    
                                                let action_row = CreateActionRow::SelectMenu(select_menu).to_owned();

                                                if let Err(err) = message.edit(&_ctx.http, EditMessage::new().embed(season_embed).remove_all_attachments().components(vec![action_row])).await {
                                                    eprintln!("Failed to edit interaction message: {:?}", err);
                                                }
                                            } else {
                                                eprintln!("Failed to fetch interaction message.");
                                            }
                                        }
                                    } else {
                                        println!("{:?}", movie_data[0].0.to_string());
                                        if let Err(err) = Streamer::start(&movie_data[0].0.to_string(), streaming_service, "1421137405158756543").await {
                                            eprintln!("Streamer Failed: {:?}", err)
                                        }
                                        if let Some(channel) = component_interaction.channel_id.to_channel(&_ctx.http).await.ok().and_then(|c| c.guild()) {
                                            if let Ok(mut message) = channel.message(&_ctx.http, component_interaction.message.id).await {
                                                let embed = CreateEmbed::new()
                                                    .title("Control the movie")
                                                    .description("Controller");
            
                                                let action_row = CreateActionRow::Buttons(vec![CreateButton::new("back").label("⏪"), CreateButton::new("pause").label("⏸️")]);
                    
                                                if let Err(err) = message.edit(&_ctx.http, EditMessage::new().embed(embed).remove_all_attachments().components(vec![action_row])).await {
                                                    eprintln!("Failed to edit interaction message: {:?}", err);
                                                }
                                            } else {
                                                eprintln!("Failed to fetch interaction message.");
                                            }
                                        }
                                    }
                                }
                                                       
                            }
                            


                        }
                    }
                } else if component_interaction.data.custom_id == "seasonSelect" {
                    if let ComponentInteractionDataKind::StringSelect { ref values } = component_interaction.data.kind {
                        if let Some(selected_season_id) = values.get(0) {
                            println!("User selected season ID: {}", selected_season_id);
                            if let Err(err) = component_interaction.defer(&_ctx.http).await {
                                eprintln!("Failed to defer interaction: {:?}", err);
                                return Ok(()); 
                            }
                            let episodes_result = NetflixSearcher::get_episodes_for_shows(selected_season_id).await;
                
                            match episodes_result {
                                Ok(episodes) => {
                                    let episode_embed = CreateEmbed::default()
                                        .title("Select an Episode")
                                        .description("Choose an episode from the dropdown menu.");
                
                                    let mut options: Vec<CreateSelectMenuOption> = Vec::new();
                
                                    for (number, title, video_id) in episodes {
                                        let episode_label = format!("Episode {} - {}", number, title);
                                        let episode_value = video_id.to_string();
                
                                        options.push(
                                            CreateSelectMenuOption::new(episode_label, episode_value)
                                        );
                                    }
                
                                    let select_menu = CreateSelectMenu::new(
                                        "episodeSelect",
                                        serenity::all::CreateSelectMenuKind::String { options },
                                    )
                                    .placeholder("Select an episode");
                
                                    let action_row = CreateActionRow::SelectMenu(select_menu).to_owned();
                
                                    if let Some(channel) = component_interaction.channel_id.to_channel(&_ctx.http).await.ok().and_then(|c| c.guild()) {
                                        if let Ok(mut message) = channel.message(&_ctx.http, component_interaction.message.id).await {
                                            if let Err(err) = message.edit(&_ctx.http, EditMessage::new()
                                                .content("")
                                                .embed(episode_embed)
                                                .components(vec![action_row])
                                            ).await {
                                                eprintln!("Failed to update message with episode selection: {:?}", err);
                                            }
                                        }
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Failed to fetch episodes for season {}: {:?}", selected_season_id, err);
                                }
                            }
                        }
                    }
                } else if component_interaction.data.custom_id == "episodeSelect" {
                    if let ComponentInteractionDataKind::StringSelect { ref values } = component_interaction.data.kind {
                        if let Some(selected_episode_id) = values.get(0) {
                            if let Err(err) = component_interaction.defer(&_ctx.http).await {
                                eprintln!("Failed to defer interaction: {:?}", err);
                                return Ok(()); 
                            }
                            println!("{}", selected_episode_id);
                        
                            if let Some(channel) = component_interaction.channel_id.to_channel(&_ctx.http).await.ok().and_then(|c| c.guild()) {
                                if let Ok(mut message) = channel.message(&_ctx.http, component_interaction.message.id).await {
                                    let embed = CreateEmbed::new()
                                        .title("Control the movie")
                                        .description("Controller");
                                    let action_row = CreateActionRow::Buttons(vec![CreateButton::new("skipback").label("⏪"), CreateButton::new("pause").label("⏸️").style(ButtonStyle::Success), CreateButton::new("skipfront").label("⏩"), CreateButton::new("stop").label("STOP").style(ButtonStyle::Danger)]);
                                    
                                    if let Err(err) = message.edit(&_ctx.http, EditMessage::new().embed(CreateEmbed::new().title("Please Wait for the Controller").description("The Controller will start once the movie/show has started, Please sit tight till that happens.")).remove_all_attachments().components(vec![])).await {
                                        eprintln!("Failed to edit interaction message: {:?}", err);
                                    }
                                    if let Err(err) = Streamer::start(&format!("https://netflix.com/watch/{}", selected_episode_id), "Netflix", "1421137405158756543").await {
                                        eprintln!("Streamer Failed: {:?}", err)
                                    }
                                    if let Err(err) = message.edit(&_ctx.http, EditMessage::new().embed(embed).remove_all_attachments().components(vec![action_row])).await {
                                        eprintln!("Failed to edit interaction message: {:?}", err);
                                    }
                                } else {
                                    eprintln!("Failed to fetch interaction message.");
                                }
                            }
                        }
                    }
                } else if component_interaction.data.custom_id == "pause" {
                    if let ComponentInteractionDataKind::Button = component_interaction.data.kind {
                        if let Err(err) = component_interaction.defer(&_ctx.http).await {
                            eprintln!("Failed to defer interaction: {:?}", err);
                            return Ok(()); 
                        }
                        Netflix::pause().await?;
                    }
                } else if component_interaction.data.custom_id == "stop" {
                    if let ComponentInteractionDataKind::Button = component_interaction.data.kind {
                        if let Err(err) = component_interaction.defer(&_ctx.http).await {
                            eprintln!("Failed to defer interaction: {:?}", err);
                            return Ok(()); 
                        }
                        quit_browser().await?;
                    }
                } else if component_interaction.data.custom_id == "skipfront" {
                    if let ComponentInteractionDataKind::Button = component_interaction.data.kind {
                        if let Err(err) = component_interaction.defer(&_ctx.http).await {
                            eprintln!("Failed to defer interaction: {:?}", err);
                            return Ok(()); 
                        }
                        Netflix::skipfront().await?;
                    }
                } else if component_interaction.data.custom_id == "skipback" {
                    if let ComponentInteractionDataKind::Button = component_interaction.data.kind {
                        if let Err(err) = component_interaction.defer(&_ctx.http).await {
                            eprintln!("Failed to defer interaction: {:?}", err);
                            return Ok(()); 
                        }
                        Netflix::skipback().await?
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}
pub async fn main() {

    
    
    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::vote(), commands::getvotes(), commands::watch(), commands::stop(), commands::pause(), commands::skip_to()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot,"),
                poise::Prefix::Literal("hey bot"),
            ],
            ..Default::default()
        },
        
        on_error: |error| Box::pin(on_error(error)),
        
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        
        
        skip_checks_for_owners: false,
        event_handler: |_ctx: &::serenity::prelude::Context, event: &serenity::FullEvent, _framework, _data| {
            Box::pin(event_handler(_ctx, event, _framework, _data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_in_guild(ctx, &framework.options().commands, GuildId::new(1421128559803498568)).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build();

    let token = "MTQyMTM0OTc4NDAwNjAzMzU0Mg.Ggs_DP.lmx0ky0TNA05PbID2KevDN_PgRk72-FKARWky4";
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}