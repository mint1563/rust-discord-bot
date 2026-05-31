use poise::serenity_prelude as serenity;
use std::env;

// ボット内で共有したいデータ（状態管理など）があればここに定義します
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Message { new_message } => {
            if new_message.content == "!ping" && !new_message.author.bot {
                new_message.channel_id.say(&ctx.http, "Pong!").await?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// ユーザーに「Pong!」と返すシンプルなスラッシュコマンド
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// 入力した文をそのまま出力するスラッシュコマンド
#[poise::command(slash_command)]
async fn text(
    ctx: Context<'_>,
    #[description = "出力したい文章"] text: String,
) -> Result<(), Error> {
    ctx.say(text).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // .env ファイルから環境変数を読み込み
    dotenvy::dotenv().ok();

    // Discordのトークンを取得
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // ボットのインテント（権限）を設定
    // ※メッセージの内容を読み取るため、`MESSAGE_CONTENT` インテントが必要です
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Poise のフレームワーク設定
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // ここに登録したいコマンドを追加していきます
            commands: vec![ping(), text()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, _framework| {
            Box::pin(async move {
                // グローバルにスラッシュコマンドを登録
                poise::builtins::register_globally(ctx, &_framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // クライアントを起動
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("クライアントの作成に失敗しました");

    client.start().await.unwrap();
}
