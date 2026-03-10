# chat-core

Rust で開発するチャット API です。Slack/LINE のような会話型プロダクトを題材にしつつ、実運用を意識した API 設計、整合性、拡張性を学べる構成を目指します。

## Status

現在は設計フェーズです。まずは破壊的変更になりやすい境界を固め、MVP を小さく実装してから機能を広げます。

## Core Direction

- `Conversation` を中心に DM / グループを共通モデルで扱う
- メッセージは `message_id` と会話内 `sequence_no` を併用して扱う
- 既読は `last_read_message_seq` で管理する
- 正本は PostgreSQL とし、WebSocket/SSE は配信手段として扱う
- 再送は `client_message_id` による冪等性で吸収する
- Push 通知や検索更新などの副作用は非同期化し、`transactional outbox` を前提にする

## MVP Scope

Phase 1 では次を対象にします。

- ユーザー作成
- 会話作成
- 会話参加
- テキストメッセージ送信
- 会話内メッセージ一覧取得
- 会話一覧取得
- 既読更新
- 未読件数取得
- WebSocket による新着通知

## Documents

- [Architecture](docs/architecture.md): ドメインモデル、整合性、配信、データ設計の方針
- [Requirements](docs/requirements.md): MVP スコープ、要求整理、未確定論点
- [Technical Requirements](docs/technical-requirements.md): 技術スタック、開発環境、運用前提

## Current Stack Direction

現時点の採用方針は次です。

- Rust
- axum
- tokio
- sqlx
- PostgreSQL
- serde
- tracing
- thiserror / anyhow

詳細は [docs/technical-requirements.md](docs/technical-requirements.md) を参照してください。
