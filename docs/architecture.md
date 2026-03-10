# Architecture

## Purpose

この文書は、`chat-core` の実装判断でぶれやすい設計境界を先に固定するための方針書です。UI や個別機能よりも、後から変えづらいドメイン境界、整合性、配信モデルを優先して定義します。

## Core Principles

### 1. 機能より先に変わりにくい境界を決める

最初に固める対象:

- 会話の単位
- メッセージ ID と順序保証
- 既読モデル
- 認可境界
- 同期 API と非同期副作用の責務分離
- 再送時の冪等性

### 2. Conversation をドメインの中心に置く

DM とグループチャットを別モデルに分けず、`Conversation` を共通の土台にします。会話参加者は `conversation_members` で表現し、1 対 1 もグループも同じ整合性ルールで扱います。

### 3. DB を正本にし、配信は再構築可能にする

正本は PostgreSQL です。WebSocket/SSE は新着配信のための補助手段とし、配信失敗時でもクライアントが差分取得または再取得で整合できることを優先します。

### 4. created_at を順序の正本にしない

時刻だけで表示順序を決めると、同時刻競合、再送、保存順と送信順のズレに弱くなります。表示順序、ページング、既読位置は `sequence_no` を基準に扱います。

### 5. 軽量 CQRS を採用する

`chat-core` では、厳密なイベントソーシングまでは採用せず、更新系と参照系を分離する軽量 `CQRS` を採用します。更新系は整合性重視、参照系は取得性能重視で設計します。

- 更新系: repository
- 参照系: query service

会話一覧や未読件数のために、read model や補助カラムを持つことを前提にします。

## Planned Domain Model

想定テーブル:

- `users`
- `conversations`
- `conversation_members`
- `messages`
- `message_attachments` (将来)
- `outbox_events` (非同期副作用用)

### `conversations`

- `id`
- `public_id`
- `conversation_type`
- `title`
- `last_message_seq`
- `last_message_at`
- `created_at`
- `updated_at`

`conversation_type` は DM / group / channel-like conversation へ拡張できる値を想定します。

### `conversation_members`

- `conversation_id`
- `user_id`
- `role`
- `membership_state`
- `last_read_message_seq`
- `last_read_at`
- `joined_at`
- `left_at`

`last_read_message_seq` により、未読件数は `last_message_seq - last_read_message_seq` を基本として扱います。

### `messages`

- `id`
- `message_id`
- `conversation_id`
- `sender_user_id`
- `sequence_no`
- `message_type`
- `text`
- `metadata`
- `client_message_id`
- `reply_to_message_id`
- `edited_at`
- `deleted_at`
- `created_at`

`message_type` と `metadata` により、将来の画像、システムメッセージ、カード、スタンプなどへ拡張できる余地を持たせます。

### `message_attachments` (future)

- `id`
- `message_id`
- `attachment_type`
- `storage_key`
- `mime_type`
- `size_bytes`
- `metadata`

本文と添付を分離し、添付の保存方式や配信方式を独立して拡張できるようにします。

## Message Consistency Model

### Message ID

メッセージには外部公開用の一意 ID を持たせます。候補は UUIDv7 または ULID です。

### Conversation-local Sequence

各会話に対して単調増加な `sequence_no` を持たせます。

用途:

- 表示順序
- cursor pagination
- 差分取得
- 既読位置管理

順序保証は「会話内の commit 順」を基準とします。

### Idempotent Send

送信 API は冪等であることを前提にします。

- クライアントは `client_message_id` を送る
- サーバーは `(conversation_id, sender_user_id, client_message_id)` を一意に扱う
- 再送時は重複保存せず、既存メッセージを返す

### Soft Delete First

削除はまず soft delete を基本にします。

- `deleted_at` を持つ
- クライアントには tombstone 表示を返せるようにする
- 将来の監査保持、添付 purge、全員削除要件に対応しやすくする

## Delivery And Side Effects

### Sync Path

送信 API で同期的に行う処理:

- 入力バリデーション
- 認可
- トランザクション内でのメッセージ保存
- 会話側の `last_message_seq` / `last_message_at` 更新
- outbox へのイベント保存
- API レスポンス返却

### Async Path

非同期で行う処理:

- 会話作成時の WebSocket 通知
- WebSocket 配信
- Push 通知
- メンション通知
- 検索インデックス更新
- 監査ログ
- 分析イベント送信

副作用の欠落を防ぐため、保存とイベント記録は同じトランザクションで行う前提です。

### Conversation Created Notification

会話作成時も、メッセージ送信時と同様に WebSocket で `conversation.created` 相当の通知を送る前提にします。

- サーバーは会話作成完了後にイベントを配信する
- クライアントは通知 payload のみを正本として扱わない
- 通知を受けたクライアントは DB 正本を参照する API から会話一覧または会話詳細を再取得する

これにより、配信の一時欠落やクライアント状態差分があっても、再取得で整合を回復できます。

## Read Model Policy

次の API は将来的に高頻度・高負荷になりやすいため、最初から取得責務を分離します。

- 会話一覧取得
- 会話内メッセージ一覧取得
- 未読件数取得
- 新着差分取得

必要に応じて、`conversations.last_message_seq` や `conversations.last_message_at` のような補助カラム、または専用 read model を導入します。

## Indexing Direction

最低限意識するインデックス:

- `messages(conversation_id, sequence_no)`
- `conversation_members(conversation_id, user_id)` unique
- `conversation_members(user_id)`
- `messages(sender_user_id, created_at)`
- `conversations(updated_at)` または一覧取得向けの補助インデックス

## Migration Principles

- 破壊的な DDL を一発で入れない
- まず追加、次にバックフィル、その後アプリ切り替え
- 不要制約の削除は最後に行う
- DB enum を固定しすぎない
- nullable な拡張点を素直に許容する

## Planned Project Structure

想定ディレクトリ構成:

- `src/presentation`
- `src/application`
- `src/domain`
- `src/infrastructure`
- `src/shared`

handler から直接 SQL を散らさず、ユースケースとインフラを分離した構成にします。
