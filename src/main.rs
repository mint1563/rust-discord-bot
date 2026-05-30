use poise::serenity_prelude as serenity;
use std::env;

// ボット内で共有したいデータ（状態管理など）があればここに定義します
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// ユーザーに「Pong!」と返すシンプルなスラッシュコマンド
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // .env ファイルから環境変数を読み込み
    dotenvy::dotenv().ok();

    // Discordのトークンを取得
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // ボットのインテント（権限）を設定
    let intents = serenity::GatewayIntents::non_privileged();

    // Poise のフレームワーク設定
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // ここに登録したいコマンドを追加していきます
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, _framework| {
            Box::pin(async move {
                // グローバル（すべてのサーバー）にスラッシュコマンドを登録
                // ※反映に数分かかる場合があります。開発時は guild コマンドとしての登録がおすすめです。
                poise::builtins::register_globally(ctx, &_framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // クライアントを起動
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
