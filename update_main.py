import sys

content = r'''extern crate rust_llm;

use poise::serenity_prelude as serenity;
use std::env;
use std::sync::Mutex;

use candle_core::Device;
use candle_nn::{VarBuilder, VarMap};
use rust_llm::model::{Config, Transformer};
use rust_llm::{chat, CharTokenizer};

// ボット内で共有したいデータ
struct Data {
    transformer: Transformer,
    device: Device,
    tokenizer: Mutex<CharTokenizer>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!(\"{} としてログインしました！\", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }

            if new_message.content == \"!ping\" {
                if let Err(why) = new_message.channel_id.say(&ctx.http, \"Pong!\").await {
                    println!(\"メッセージ送信エラー: {:?}\", why);
                }
            }
        }
        _ => {}
    }
    Ok(())
}

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(\"Pong!\").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn text(
    ctx: Context<'_>,
    #[description = \"出力したい文章\"] text: String,
) -> Result<(), Error> {
    ctx.say(text).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn ask(
    ctx: Context<'_>,
    #[description = \"AIに送信するプロンプト\"] prompt: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let response = {
        let data = ctx.data();
        let mut tokenizer = data.tokenizer.lock().unwrap();
        match chat(&data.transformer, &data.device, &mut tokenizer, &prompt) {
            Ok(res) => {
                if res.is_empty() {
                    \"（AIは何も返答しませんでした）\".to_string()
                } else {
                    res
                }
            }
            Err(e) => format!(\"エラーが発生しました: {}\", e),
        }
    };

    ctx.say(response).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token = env::var(\"DISCORD_TOKEN\").expect(\"Expected a token in the environment\");

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), text(), ask()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, _framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &_framework.options().commands).await?;
                
                let device = Device::Cpu;
                let varmap = VarMap::new();
                let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, &device);
                let config = Config::new();
                let transformer = Transformer::new(&config, vb).unwrap();
                let tokenizer = CharTokenizer::new();

                Ok(Data {
                    transformer,
                    device,
                    tokenizer: Mutex::new(tokenizer),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect(\"クライアントの作成に失敗しました\");

    client.start().await.unwrap();
}
'''

with open('src/main.rs', 'w', encoding='utf-8') as f:
    f.write(content)
