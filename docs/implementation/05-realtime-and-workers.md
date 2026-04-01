# 05 Realtime And Workers

## 目的

WebSocket 配信、outbox worker、検索更新を接続し、MVP の非同期経路を成立させる。

## やること

- [ ] WebSocket 接続管理を実装する
- [ ] 接続時に内部認証済みユーザー文脈を関連付ける
- [ ] `conversation.created` 通知を実装する
- [ ] 通常メッセージ作成通知を実装する
- [ ] スレッド返信作成通知を実装する
- [ ] メッセージ編集通知を実装する
- [ ] メッセージ削除通知を実装する
- [ ] 通知 payload は「再取得のきっかけ」になる最小情報にする
- [ ] outbox consumer を実装する
- [ ] retry 方針を実装する
- [ ] 失敗時の再実行で二重処理しないようにする
- [ ] 検索更新 worker を実装する
- [ ] create 時に `search_documents` を upsert する
- [ ] edit 時に `search_documents` を更新する
- [ ] delete 時に `search_documents` を除外または tombstone 化する
- [ ] メンション通知 worker を実装する
- [ ] 監査ログは同期保存なので worker に逃がさない

## 実装メモ

- クライアントは通知を受けたら DB 正本を再取得する前提
- WebSocket payload は UI 表示データを過剰に持たせない
- outbox の event type は message create / edit / delete / conversation create / mention のように分ける
- worker は最初は同一プロセス内タスクでよい
- 将来 Redis や別プロセスに切り出しやすいように consumer インターフェースを分ける

## 完了条件

- write path で積んだ outbox event が実際に消費される
- WebSocket 通知を受けてクライアント側の再取得フローを組める
- 検索 index が送信、編集、削除に追従する
