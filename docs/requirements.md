# Requirements

## Purpose

この文書は、`chat-core` の MVP と将来拡張を切り分け、先に確定すべき要求を整理するための要求一覧です。

## Product Direction

`chat-core` は Slack/LINE のようなチャット体験を参考にしますが、最初から両者を完全に再現することは目指しません。まずは会話、参加者、メッセージ、既読、新着通知の最小集合を成立させます。

## MVP Functional Requirements

Phase 1 で対象にする要求:

- ユーザーを作成できる
- 会話を作成できる
- 会話作成時に WebSocket で新規会話通知を送れる
- 会話に参加者を追加できる
- テキストメッセージを送信できる
- 会話内メッセージを順序付きで取得できる
- 会話一覧を取得できる
- 既読位置を更新できる
- 未読件数を取得できる
- 新着メッセージを WebSocket で通知できる

## MVP Acceptance Criteria

最初の実装で最低限満たしたい条件:

- 同一会話内のメッセージは `sequence_no` に従って返る
- 同じ `client_message_id` による再送では重複メッセージが作られない
- `last_read_message_seq` の更新は後戻りしない
- 配信が失敗しても再取得で整合が取れる
- 会話作成通知を受けたクライアントは DB 正本から会話情報を再取得して整合を取れる
- 会話一覧で各会話の最新メッセージ位置を参照できる

## Future Requirements

### Phase 2

- メッセージ編集
- メッセージ削除
- リプライ
- リアクション
- 添付ファイル
- メンション
- ピン留め
- 検索

### Phase 3

- スレッド
- 既読者一覧
- スタンプ
- 送信予約
- Bot 連携
- 監査エクスポート
- E2E 暗号化

## Non-functional Requirements To Define

実装前または実装初期に明文化する項目:

- 最大同時接続数
- 1 会話あたりの最大参加人数
- 1 メッセージ最大文字数
- 添付ファイル最大サイズ
- 既読反映の許容遅延
- 新着通知の許容遅延
- 順序保証レベル
- 保存期間
- 削除ポリシー
- 監査ログ要件
- 検索要件
- 将来のマルチテナント対応有無

## Key Domain Decisions Still Open

実装前に判断が必要な論点:

- Slack 寄りか LINE 寄りか
- `tenant` / `workspace` を持つか
- 認証・認可をどこまで `chat-core` の責務に含めるか
- `sequence_no` の採番方式をどうするか
- 参加前ログを閲覧可能にするか
- 退会 / 再参加時の unread と履歴可視性をどう扱うか
- 添付ファイル保存基盤を何にするか
- 検索を MVP の外に出すか

## Domain Checklist

要求整理は次の順番で確定していく想定です。

1. ドメイン
- ユーザー
- 会話
- 会話参加者
- メッセージ
- 既読位置

2. ユースケース
- 会話作成
- メッセージ送信
- メッセージ取得
- 既読更新
- 会話一覧取得

3. 非機能
- 性能
- 順序保証
- 可用性
- 監査性
- スケーラビリティ

4. 将来拡張
- 添付
- リアクション
- 編集削除
- 通知
- 検索
- スレッド

## Implementation Flow

実装は次の順で進める想定です。

1. 未確定論点を絞り込み、MVP の前提を固定する
2. `Cargo.toml`、`src/`、`migrations/`、`tests/` などの土台を作る
3. `users`、`conversations`、`conversation_members`、`messages`、`outbox_events` の初期スキーマを作る
4. 会話作成、メッセージ送信、既読更新の command 側を実装する
5. 会話一覧、会話詳細、メッセージ一覧、未読件数の query 側を実装する
6. `conversation.created` と message event の WebSocket 通知を実装し、受信側が DB 正本を再取得する前提を成立させる
7. integration test で順序保証、冪等送信、再取得による整合回復を確認する
