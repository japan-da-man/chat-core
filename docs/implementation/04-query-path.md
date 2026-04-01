# 04 Query Path

## 目的

クライアント表示に必要な読み取り系 API を実装する。

## やること

- [ ] 会話一覧 query を実装する
- [ ] 会話一覧で `unread_count` を返す
- [ ] 会話一覧で `has_unread_threads` を返す
- [ ] スレッド返信で会話一覧ソートが変わらないことを反映する
- [ ] 会話詳細 query を実装する
- [ ] 通常メッセージ一覧 query を実装する
- [ ] cursor pagination を実装する
- [ ] スレッド返信一覧 query を実装する
- [ ] `reply_seq` ベースの pagination を実装する
- [ ] 未読件数 query を実装する
- [ ] 通常未読とスレッド未読の責務を分ける
- [ ] 検索 query を実装する
- [ ] 全会話横断検索を実装する
- [ ] 会話内検索を実装する
- [ ] `sort=relevance` と `sort=newest` を実装する
- [ ] 検索結果に必要な項目を返す
- [ ] `message_id`
- [ ] `conversation_id`
- [ ] `thread_root_message_id`
- [ ] `snippet`
- [ ] `sender_user_id`
- [ ] `created_at`
- [ ] `conversation_title`
- [ ] 検索結果の最終可視性判定を `conversation_members` で行う

## 実装メモ

- 通常会話一覧の `unread_count` にスレッド返信を混ぜない
- `has_unread_threads` は `thread_members.last_read_reply_seq` から出す
- 検索結果は常にヒットしたメッセージ単位
- relevance は DB / 検索基盤のスコアを使い、同点は新しいものを優先する
- 削除済みメッセージは query でも検索でも除外する

## 完了条件

- クライアントが会話一覧、メッセージ一覧、スレッド一覧、検索結果を描画できる
- 通常未読とスレッド未読が分離されている
- 検索結果に必要な遷移情報が入っている
