---
name: spec-reviewer
description: Review specs and design docs for ambiguity, contradictions, and missing cases. Use when the user asks for a spec review, design review, requirements review, or 仕様レビューをしたい.
---

# Spec Reviewer

要件書、仕様書、設計書を実装前にレビューするための skill。コードレビューではなく、文書レビューに特化する。

## Use when

- 仕様レビューをしたい
- 設計書の穴を見つけたい
- 曖昧さや矛盾を洗いたい
- 実装前レビューをしたい

## Review priorities

次の観点を優先する。

1. 曖昧な文言
2. 文書内の矛盾
3. UI / UX 上の分岐漏れ
4. 権限モデル不足
5. API 契約不足
6. エラー仕様不足
7. 将来拡張性の弱さ

## Review method

1. 文書の目的とスコープを確認する。
2. 用語定義が揃っているかを見る。
3. 正常系だけでなく、異常系、権限制約、状態遷移を確認する。
4. 外部 API、DB、通知、ジョブなどの境界面を洗う。
5. 仕様として未決定の論点と、実装者が誤って補完しそうな論点を分ける。

## Output format

指摘は重大度付きで列挙する。出力順は重大なものから。

```md
## Findings

### <Critical|High|Medium|Low> <短いタイトル>
- 問題:
- なぜ重要か:
- どこが曖昧 / 不足 / 矛盾しているか:
- 追加で決めるべきこと:

## Open Questions
- ...

## Change Summary
- ...
```

## What to flag

- `適切に`, `必要に応じて`, `ユーザーにわかりやすく` のような曖昧語
- 権限ごとの差が未記載
- エラー時の戻り値や UI 表示が未定義
- API 入出力、制約、バージョニング方針が未定義
- 非同期処理や再実行時の仕様が曖昧
- 将来の追加要件で破綻しやすい前提

## What not to do

- 実装案を先に膨らませて曖昧さを隠さない。
- 文書に書かれていない仕様を断定しない。
- 細かな文体修正を主目的にしない。

## Quality bar

- レビュー対象がそのまま実装に渡ることを想定し、実装者が迷う点を優先して指摘する。
- 指摘には、どの情報が不足しているかを明確に書く。
- 可能なら「判断が必要な論点」と「単純な追記不足」を分ける。
