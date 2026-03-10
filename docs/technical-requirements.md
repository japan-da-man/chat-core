# Technical Requirements

## Purpose

この文書は、`chat-core` の技術選定、開発環境、実行環境、運用前提をまとめるための技術要件ドキュメントです。

## Technology Direction

現時点の採用方針:

- Language: Rust
- Architecture style: layered architecture + lightweight CQRS
- Web framework: axum
- Async runtime: tokio
- Database access: sqlx
- Primary database: PostgreSQL
- Serialization: serde
- Logging / tracing: tracing
- Error handling: thiserror / anyhow
- Validation: `validator` または独自バリデーション

ORM ではなく `sqlx` を優先する理由:

- 実 SQL を明示的に扱える
- クエリ設計を学びやすい
- チャット API では読み取り設計とインデックス設計が重要
- ORM の隠蔽が少なく、整合性の把握がしやすい

## Toolchain Policy

バージョンはコード初期化時に固定しますが、方針は次です。

- Rust stable を使う
- Rust edition は新規プロジェクト作成時点の stable に合わせる
- `cargo fmt` と `cargo clippy` を標準品質ゲートにする
- 非同期テストは `tokio::test` を基本にする
- DB migration は SQL ベースで管理する

## Local Development Environment

ローカル開発の前提:

- macOS または Linux
- Windows は WSL2 経由を推奨
- PostgreSQL はローカルインストールまたは Docker/Compose で起動
- Rust は `rustup` で管理

推奨ツール:

- `rustup`
- `cargo`
- `cargo fmt`
- `cargo clippy`
- `cargo test`
- `sqlx-cli`
- `docker` / `docker compose`

## Runtime Environment Assumptions

初期構成では、次のようなシンプルな実行形態を想定します。

- API サーバー 1 プロセス
- PostgreSQL 1 系統
- 非同期副作用は同一アプリ内ワーカー、または別ワーカーで処理

将来的には次の分離を想定します。

- API request handling
- WebSocket connection handling
- outbox consumer / background jobs
- search indexing worker

## Configuration Policy

設定は環境変数を基本にします。想定する代表的な設定項目:

- `APP_HOST`
- `APP_PORT`
- `DATABASE_URL`
- `RUST_LOG`
- `APP_ENV`

添付ファイル対応後に追加想定:

- `OBJECT_STORAGE_ENDPOINT`
- `OBJECT_STORAGE_BUCKET`
- `OBJECT_STORAGE_REGION`

## Database Requirements

- PostgreSQL を正本データベースとして採用する
- timestamp は DB 側で一貫して生成し、保存は UTC を基本とする
- メッセージ取得は cursor ベースで設計する
- `sequence_no` による会話内順序を保持する
- 再送吸収のために冪等キー制約を持つ
- outbox テーブルを使って副作用を非同期化できるようにする

## Observability Requirements

少なくとも次を最初から意識します。

- 構造化ログ
- request 単位の trace
- 失敗した非同期処理の再試行可能性
- メッセージ送信、既読更新、配信失敗の主要メトリクス

## Security And Operational Baseline

- 認証方式は別途確定するが、認可境界は会話参加者ベースで設計する
- TLS 終端はアプリ外の ingress / reverse proxy でもよい
- シークレットは環境変数または secret manager で注入する
- 添付ファイルは将来的にオブジェクトストレージ前提で扱う

## Technical Decisions Pending

今後確定が必要な技術論点:

- Rust edition と crate バージョンの固定値
- API を REST 中心にするか gRPC も持つか
- WebSocket と SSE のどちらを初期採用するか
- outbox consumer を同一プロセスにするか別プロセスにするか
- 添付ファイル保存先を S3 互換にするか
- ローカル開発用の Docker 構成をどこまで用意するか
