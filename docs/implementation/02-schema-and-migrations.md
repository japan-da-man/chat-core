# 02 Schema And Migrations

## 目的

MVP のドメイン前提を PostgreSQL schema に落とし込み、初期 migration を作る。

## やること

- [ ] `PGroonga` 拡張を有効化する migration を作る
- [ ] `users` テーブルを作る
- [ ] `conversations` テーブルを作る
- [ ] `conversation_members` テーブルを作る
- [ ] `messages` テーブルを作る
- [ ] `message_mentions` テーブルを作る
- [ ] `thread_members` テーブルを作る
- [ ] `search_documents` テーブルを作る
- [ ] `audit_logs` テーブルを作る
- [ ] `outbox_events` テーブルを作る
- [ ] 必要な unique 制約と index を追加する
- [ ] `messages(conversation_id, sequence_no)` index
- [ ] `messages(thread_root_message_id, reply_seq)` index
- [ ] `conversation_members(conversation_id, user_id)` unique
- [ ] `thread_members(thread_root_message_id, user_id)` unique
- [ ] `message_mentions(message_id, mentioned_user_id)` unique
- [ ] `search_documents` の `PGroonga` index
- [ ] 1 対 1 DM を同じ 2 人で重複作成しないための制約方法を決める
- [ ] `messages` の通常メッセージとスレッド返信に対する check 制約を決める
- [ ] 監査ログと outbox の event type 命名を決める

## 実装メモ

- `conversations.conversation_type` は MVP では `dm` と `group`
- `conversations.owner_user_id` は MVP の owner 権限判定に使う
- `conversation_members.role` は MVP では `owner` / `member`
- `messages` は 1 テーブルで通常メッセージとスレッド返信を共存させる
- 通常メッセージは `sequence_no` を持つ
- スレッド返信は `thread_root_message_id` と `reply_seq` を持つ
- `sequence_no` と `reply_seq` は両方 nullable にして check 制約で整合を取る方が実装しやすい
- `search_documents` は将来 OpenSearch に移行する前提で、検索専用の非正規化テーブルとして扱う
- `audit_logs` は `actor_user_id`, `action`, `resource_type`, `resource_id`, `occurred_at`, `metadata` を持つ
- `outbox_events` には最低でも `event_id`, `event_type`, `aggregate_type`, `aggregate_id`, `payload`, `occurred_at`, `processed_at` を持たせる

## 完了条件

- `sqlx migrate run` で初期 schema が作成できる
- 通常メッセージとスレッド返信の整合制約が migration に入っている
- `PGroonga` を使った検索用 index まで作られている
- テーブル名、カラム名、制約名が docs と対応している
