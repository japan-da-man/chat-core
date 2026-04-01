# Technical Requirements

## Purpose

この文書は、`chat-core` の技術選定、開発環境、実行環境、運用前提をまとめるための技術要件ドキュメントです。

## Technology Direction

現時点の採用方針:

- Language: Rust
- Architecture style: layered architecture + lightweight CQRS
- Web framework: axum
- HTTP API style: REST
- Async runtime: tokio
- Middleware foundation: tower / tower-http
- Database access: sqlx
- Primary database: PostgreSQL
- Search for MVP: PostgreSQL + PGroonga
- Serialization: serde
- Logging / tracing: tracing
- Error handling: thiserror / anyhow
- Validation: `validator` または独自バリデーション

ORM ではなく `sqlx` を優先する理由:

- 実 SQL を明示的に扱える
- クエリ設計を学びやすい
- チャット API では読み取り設計とインデックス設計が重要
- ORM の隠蔽が少なく、整合性の把握がしやすい

`axum` を採用する理由:

- `tokio` / `tower` / `hyper` 系の部品と素直に組める
- request ID、tracing、auth、CORS、compression などの middleware をレイヤーで積みやすい
- WebSocket を同一アプリ内で扱いやすい
- 今回の `REST + WebSocket + outbox worker` 構成と相性がよい

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
- MVP のローカル検索は PostgreSQL 拡張として `PGroonga` を導入する
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
- self-hosted PostgreSQL 1 系統
- WebSocket connection handling は API サーバー内で行う
- 非同期副作用は同一アプリ内ワーカーで処理する
- 検索更新は outbox を介して `search_documents` を更新する

将来的には次の分離を想定します。

- API request handling
- WebSocket connection handling
- outbox consumer / background jobs
- search indexing worker
- 正本 DB を Aurora PostgreSQL、検索を OpenSearch へ切り出す

## Configuration Policy

設定は環境変数を基本にします。想定する代表的な設定項目:

- `APP_HOST`
- `APP_PORT`
- `DATABASE_URL`
- `INTERNAL_API_BEARER_TOKEN`
- `RUST_LOG`
- `APP_ENV`
- `SEARCH_BACKEND`

添付ファイル対応後に追加想定:

- `OBJECT_STORAGE_ENDPOINT`
- `OBJECT_STORAGE_BUCKET`
- `OBJECT_STORAGE_REGION`

## Database Requirements

- PostgreSQL を正本データベースとして採用する
- MVP の日本語検索は PostgreSQL 拡張 `PGroonga` を使う
- timestamp は DB 側で一貫して生成し、保存は UTC を基本とする
- メッセージ取得は cursor ベースで設計する
- `sequence_no` による会話内順序を保持する
- `reply_seq` によるスレッド内順序を保持する
- 再送吸収のために冪等キー制約を持つ
- `search_documents` のような検索専用テーブルを持つ
- `audit_logs` を持ち、主要 write 操作の証跡を残す
- outbox テーブルを使って副作用を非同期化できるようにする

## API Boundary

- `chat-core` はエンドユーザーに直接公開しない
- 経路は `アプリクライアント -> 認証付き Application API -> chat-core` を前提にする
- `chat-core` は service-to-service の bearer token で呼び出し元を信頼する
- 認証済みユーザー文脈は `x-user-id` 相当のヘッダで受け渡す
- 最終的な認可判定は `chat-core` が `conversation_members` を使って行う

## Search Architecture Direction

- 検索対象は会話タイトル、通常メッセージ本文、スレッド本文とする
- 検索結果はヒットしたメッセージ単位で返す
- 検索ソートは `relevance` と `newest` を提供し、デフォルトは `relevance` にする
- relevance の重み付けは `本文ヒット > 会話タイトルヒット` とする
- 削除済みメッセージは検索対象から除外し、編集前本文は残さない
- 将来 OpenSearch へ切り出しても API 契約を変えないようにする

## Observability Requirements

少なくとも次を最初から意識します。

- 構造化ログ
- request 単位の trace
- 失敗した非同期処理の再試行可能性
- メッセージ送信、既読更新、配信失敗の主要メトリクス

## Security And Operational Baseline

- 認証は外部の Application API が持ち、`chat-core` は内部 API として使う
- 認可境界は会話参加者ベースで設計する
- TLS 終端はアプリ外の ingress / reverse proxy でもよい
- シークレットは環境変数または secret manager で注入する
- 添付ファイルは将来的にオブジェクトストレージ前提で扱う

## Technical Decisions Pending

今後確定が必要な技術論点:

- Rust edition と crate バージョンの固定値
- Docker / Compose でどこまで `PGroonga` を同梱するか
- 添付ファイル保存先を S3 互換にするか
- 保存期間とバックアップ方針をどう置くか
- ローカル開発用の Docker 構成をどこまで用意するか
