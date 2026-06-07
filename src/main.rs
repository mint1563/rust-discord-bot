extern crate rust_llm;

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
        // ボットの起動が完了したとき
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("{} としてログインしました！", data_about_bot.user.name);
        }
        // メッセージが投稿されたとき
        serenity::FullEvent::Message { new_message } => {
            // メッセージ送信者がボット自身なら無視
            if new_message.author.bot {
                return Ok(());
            }

            // 「!ping」というメッセージに反応
            if new_message.content == "!ping" {
                if let Err(why) = new_message.channel_id.say(&ctx.http, "Pong!").await {
                    println!("メッセージ送信エラー: {:?}", why);
                }
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

/// rust-llmを使ってAIに質問します
#[poise::command(slash_command)]
async fn ask(
    ctx: Context<'_>,
    #[description = "AIに送信するプロンプト"] prompt: String,
) -> Result<(), Error> {
    // LLMの処理は時間がかかることが多いため、Discordの3秒ルール（タイムアウト）を
    // 回避するために「考え中...」状態を先に送信します (defer)
    ctx.defer().await?;

    // -----------------------------------------------------------------------
    // TODO: ここでご自身の rust-llm の関数を呼び出します。
    // リポジトリの実際の構成に合わせて書き換えてください。
    //
    // 【実装例】
    // let response = rust_llm::generate(&prompt).await?;
    // -----------------------------------------------------------------------

    // 以下はコンパイルを通すための仮のレスポンスです。実際の実装に置き換えてください。
    let response = format!(
        "「{}」ですね。これはrust-llmからの応答のモックです。",
        prompt
    );

    // 処理が完了したら、結果をユーザーに返信します
    ctx.say(response).await?;

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
            commands: vec![ping(), text(), ask()],
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
