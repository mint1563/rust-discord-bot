use poise::serenity_prelude as serenity;
use std::env;

// Botの全体で共有したいデータ（APIクライアントなど）があればここに持たせます
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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
    // .envファイルから環境変数を読み込みます
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("環境変数 DISCORD_TOKEN が設定されていません");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ask()], // 作成したスラッシュコマンドを登録
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // スラッシュコマンドをDiscordにグローバル登録します
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap();
    client.start().await.unwrap();
}
