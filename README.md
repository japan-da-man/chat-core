# chat-core

Rust で開発するチャット API です。Slack 寄りの会話型プロダクトを題材にしつつ、実運用を意識した API 設計、整合性、拡張性を学べる構成を目指します。

## Status

現在は設計フェーズです。まずは破壊的変更になりやすい境界を固め、MVP を小さく実装してから機能を広げます。

## Core Direction

- `Conversation` を中心に DM / グループを共通モデルで扱う
- 更新系と参照系を分離する軽量 `CQRS` を採用する
- メッセージは `message_id` と会話内 `sequence_no` を併用して扱う
- スレッド返信は `thread_root_message_id` と `reply_seq` で扱う
- 通常会話の既読は `last_read_message_seq`、スレッド既読は `last_read_reply_seq` で管理する
- 正本は PostgreSQL とし、WebSocket は配信手段として扱う
- 会話作成時も WebSocket で通知し、受信側は通知をきっかけに DB 正本から会話情報を再取得する
- 再送は `client_message_id` による冪等性で吸収する
- 検索は MVP では `PostgreSQL + PGroonga` で始め、将来 OpenSearch へ移行可能な形で作る
- Push 通知や検索更新などの副作用は非同期化し、`transactional outbox` を前提にする

## MVP Scope

Phase 1 では次を対象にします。

- 外部認証済みユーザーの内部 API 経由登録
- DM / グループ会話作成
- グループ会話のタイトル管理と参加者追加
- テキストメッセージ送信
- スレッド返信
- メッセージ編集 / 削除
- メンション記録
- 会話内メッセージ一覧取得
- スレッド一覧取得
- 会話一覧取得
- 通常会話 / スレッド既読更新
- 未読件数取得
- 検索
- WebSocket による新着通知

## Documents

- [Architecture](docs/architecture.md): ドメインモデル、整合性、配信、データ設計の方針
- [Requirements](docs/requirements.md): MVP スコープ、固定した前提、将来拡張
- [Technical Requirements](docs/technical-requirements.md): 技術スタック、実行環境、運用前提
- [Implementation Plan](docs/implementation/README.md): 実装を進めるための段階別タスクリスト

## Next Steps

この後は次の流れで進めます。

1. 未確定論点を絞って確定する
2. Rust プロジェクトとディレクトリ構成を作る
3. PostgreSQL migration と基本スキーマを作る
4. 会話、スレッド、検索、監査ログを含む MVP の write path と read path を実装する
5. WebSocket 通知と outbox worker をつなぐ
6. 並行送信、再送、スレッド未読、検索反映の integration test を整備する

## Current Stack Direction

現時点の採用方針は次です。

- Rust
- axum
- tower / tower-http
- tokio
- sqlx
- PostgreSQL
- PGroonga
- serde
- tracing
- thiserror / anyhow

詳細は [docs/technical-requirements.md](docs/technical-requirements.md) を参照してください。
