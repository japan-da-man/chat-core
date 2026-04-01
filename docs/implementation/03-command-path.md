# 03 Command Path

## 目的

書き込み系ユースケースを実装し、整合性の中心を固める。

## やること

- [ ] 内部 API 用の認証 middleware を実装する
- [ ] bearer token 検証を実装する
- [ ] `x-user-id` 相当ヘッダを request context に積む
- [ ] ユーザー登録 command を実装する
- [ ] 会話作成 command を実装する
- [ ] DM 作成時の既存会話再利用を実装する
- [ ] グループ会話作成時の title 必須チェックを実装する
- [ ] グループ会話作成時の初期参加者追加を実装する
- [ ] 参加者追加 command を実装する
- [ ] owner のみ追加可能な認可を実装する
- [ ] 通常メッセージ送信 command を実装する
- [ ] `sequence_no` 採番を実装する
- [ ] スレッド返信 command を実装する
- [ ] `reply_seq` 採番を実装する
- [ ] スレッド未読対象の `thread_members` 更新を実装する
- [ ] `mentioned_user_ids` の membership 検証を実装する
- [ ] `message_mentions` 保存を実装する
- [ ] メッセージ編集 command を実装する
- [ ] 送信者本人のみ編集可能な認可を実装する
- [ ] 編集時の検索更新イベントと監査ログイベントを積む
- [ ] メッセージ削除 command を実装する
- [ ] tombstone 用の soft delete を実装する
- [ ] 送信者本人のみ削除可能な認可を実装する
- [ ] 通常会話既読更新 command を実装する
- [ ] `last_read_message_seq` の前進のみを保証する
- [ ] スレッド既読更新 command を実装する
- [ ] `last_read_reply_seq` の前進のみを保証する
- [ ] 各 command で `audit_logs` と `outbox_events` を同一トランザクションで保存する

## 実装メモ

- command 実装では handler から直接 SQL を散らさない
- 認証は外部、認可は `chat-core` の責務
- WebSocket payload 自体を正本にしないので、outbox payload は最小でもよい
- `client_message_id` の冪等制約は通常メッセージとスレッド返信の両方で効くようにする
- 親メッセージ削除時もスレッド返信は消さない

## 完了条件

- 主要 write API が一通り叩ける
- 認可違反で正しく失敗する
- 冪等送信、既読前進、owner 制約がコード上で保証されている
- すべての write 操作で audit / outbox が記録される
