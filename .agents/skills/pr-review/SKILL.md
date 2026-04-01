---
name: pr-review
description: Review code changes in a PR with a fixed checklist. Use when the user asks for a PR review, code review, diff review, or @codex review style feedback.
---

# PR Review

Pull Request や差分レビューの観点を固定し、重要な指摘から返すための skill。

## Use when

- PR をレビューしたい
- 差分レビューをしたい
- 実装レビューの観点を固定したい
- `@codex review` 相当の観点で見たい

## Review priorities

次の順で確認する。

1. 仕様逸脱
2. 認可 / 認証漏れ
3. 例外処理不足
4. リソースリーク
5. N+1 や不要な全件走査
6. テスト不足
7. 破壊的変更
8. docs / migration の漏れ

## Review method

1. まず変更の意図と対象範囲を把握する。
2. 次に仕様との差分を確認する。
3. 境界条件、失敗時、権限差分、データ更新、副作用を追う。
4. 最後に性能、保守性、ドキュメント更新漏れを見る。

## Output format

指摘を先に出し、要約は短くする。

```md
## Findings

### <Critical|High|Medium|Low> <短いタイトル>
- 問題:
- なぜ重要か:
- 修正の方向性:
- テスト / docs / migration の更新要否:

## Open Questions
- ...

## Summary
- ...
```

## What to check

- 仕様と違う振る舞いになっていないか
- 認証済みだが未認可の操作が通らないか
- 失敗時に例外が握りつぶされていないか
- ファイル、コネクション、ストリーム、goroutine、タスクが解放されるか
- ループ内クエリや不要な全件読み込みがないか
- 重要ケースのテストが増えているか
- 既存クライアントを壊す変更がないか
- 仕様書、migration、運用手順の更新漏れがないか

## What not to do

- 軽微なスタイル指摘を先に出さない。
- 変更意図を確認せずに主観でリファクタ提案を大量に出さない。
- 指摘がないのに無理にコメントを作らない。

## Quality bar

- findings first で返す。
- 重大なものから順に並べる。
- 可能なら再現条件や影響範囲を簡潔に添える。
