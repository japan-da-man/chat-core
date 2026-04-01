# 06 Testing And Quality

## 目的

MVP の成立条件を integration test と品質ゲートで担保する。

## やること

- [ ] test 用 PostgreSQL 起動方法を決める
- [ ] `PGroonga` を含むローカル test 環境を整える
- [ ] migration を test 前に適用する仕組みを作る
- [ ] integration test を追加する
- [ ] DM 重複作成防止
- [ ] 通常メッセージ `sequence_no` 順序保証
- [ ] スレッド返信 `reply_seq` 順序保証
- [ ] `client_message_id` 冪等送信
- [ ] 送信者本人のみ編集可能
- [ ] 送信者本人のみ削除可能
- [ ] tombstone 削除
- [ ] 通常未読の前進のみ更新
- [ ] スレッド既読の前進のみ更新
- [ ] `has_unread_threads` の反映
- [ ] 会話一覧ソートにスレッド返信が影響しないこと
- [ ] 検索結果に削除済みが出ないこと
- [ ] 編集後に検索結果が更新されること
- [ ] membership 外の検索結果が返らないこと
- [ ] WebSocket 通知後に再取得で整合回復できること
- [ ] `cargo fmt` を CI 相当で通す
- [ ] `cargo clippy` を CI 相当で通す
- [ ] `cargo test` を CI 相当で通す

## 実装メモ

- 単体テストより integration test を優先する
- command と query は API 経由でまとめて検証する
- search と outbox は成功ケースだけでなく再試行ケースも確認したい
- テストデータは通常会話とスレッド会話を両方含める

## 完了条件

- MVP acceptance criteria が test で裏付けられている
- 主要な認可違反と整合性違反を test で落とせる
- `fmt`, `clippy`, `test` を通す手順が固定されている
