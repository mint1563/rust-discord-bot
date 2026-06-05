# Rust Discord Bot

Rust と [poise](https://github.com/serenity-rs/poise) / [serenity](https://github.com/serenity-rs/serenity) クレートを使用して作成されたシンプルな Discord ボットです。

## 機能

現在、以下のコマンドと機能が実装されています。

### スラッシュコマンド
- `/ping` : ボットが「Pong!」と返信します。
- `/text <text>` : 入力したメッセージをそのままオウム返しします。

### テキストコマンド
- `!ping` : チャットで `!ping` と送信すると「Pong!」と返信します。

## 前提条件

このボットを実行するには、以下の準備が必要です。

1. **Rustツールチェーン** がインストールされていること。（[Rustup](https://rustup.rs/) を使用してください）
2. [Discord Developer Portal](https://discord.com/developers/applications) でボットを作成し、**Bot Token** を取得していること。
3. Developer Portalの「Bot」タブ内で、以下の **Privileged Gateway Intents** を有効にしていること：
   - **Message Content Intent** (テキストコマンドの `!ping` を処理するために必要です)

## セットアップと実行方法

1. リポジトリをクローンまたはダウンロードします。
2. プロジェクトのルートディレクトリ（`Cargo.toml` があるディレクトリ）に `.env` ファイルを作成し、取得した Discord トークンを記述します。

   ```env
   DISCORD_TOKEN=あなたの_DISCORD_BOT_TOKEN_をここに貼り付けます
   ```

3. プロジェクトをビルド・実行します。

   ```bash
   cargo run
   ```

4. コンソールに `[ボット名] としてログインしました！` と表示されれば、起動完了です。

## 使用技術

- [Rust](https://www.rust-lang.org/) (Edition 2021)
- [serenity](https://crates.io/crates/serenity) v0.12 - Discord API ラッパー
- [poise](https://crates.io/crates/poise) v0.6 - serenity ベースのコマンドフレームワーク
- [tokio](https://crates.io/crates/tokio) v1 - 非同期ランタイム
- [dotenvy](https://crates.io/crates/dotenvy) v0.15 - 環境変数（`.env`）の読み込み
