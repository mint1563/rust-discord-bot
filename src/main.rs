use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("メッセージ送信エラー: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} としてログインしました！", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // 💡 .env ファイルから環境変数を読み込む（この1行を追加）
    dotenvy::dotenv().expect(".env ファイルの読み込みに失敗しました");

    // これで自動的に .env 内の DISCORD_TOKEN が取得できるようになります
    let token = env::var("DISCORD_TOKEN").expect("環境変数 'DISCORD_TOKEN' が設定されていません");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("クライアントの作成に失敗しました");

    if let Err(why) = client.start().await {
        println!("クライアントの起動エラー: {:?}", why);
    }
}
