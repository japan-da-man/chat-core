---
name: security-regression-review
description: Review changes specifically for security regressions. Use when the user asks for a security review, security regressions, auth bypass checks, or @codex review for security regressions.
---

# Security Regression Review

セキュリティ退行だけに絞ってレビューするための skill。通常レビューと分け、攻撃可能性と権限境界の崩れを重点的に見る。

## Use when

- セキュリティレビューをしたい
- security regression を確認したい
- auth bypass や secret 漏えいを見たい
- 依存追加や middleware 変更のリスクを見たい

## Review priorities

次の観点を優先する。

1. secrets の露出
2. auth bypass
3. middleware 漏れ
4. PII ログ
5. dependency 追加リスク
6. SSRF / injection / unsafe deserialization
7. permission boundary の崩れ

## Review method

1. 認証、認可、境界防御の変更点を特定する。
2. 外部入力が入る箇所を洗い出す。
3. ログ、監査、例外文言に機微情報が出ないかを見る。
4. 新規依存や設定変更が攻撃面を広げていないか確認する。
5. テストが security-sensitive path をカバーしているか確認する。

## Output format

```md
## Security Findings

### <Critical|High|Medium|Low> <短いタイトル>
- 問題:
- 想定される攻撃または事故:
- 影響範囲:
- 修正の方向性:
- 追加すべきテスト:

## Residual Risks
- ...
```

## What to flag

- API key、token、secret、cookie、credential のハードコードや出力
- 認証チェック前の処理実行
- middleware、policy、guard、filter の適用漏れ
- 個人情報や内部識別子のログ出力
- 外部 URL、SQL、テンプレート、シリアライズ対象への未検証入力
- 新規依存によるサプライチェーンリスク
- テナント境界、ロール境界、管理者境界の崩れ

## What not to do

- 一般的なコード品質レビューに流れない。
- 影響経路を説明せずに「危ないかもしれない」で終わらせない。
- 実際には到達不能な経路を、根拠なく重大扱いしない。

## Quality bar

- exploitability と impact を意識して重大度を付ける。
- 指摘には、攻撃経路または事故シナリオを一文で添える。
- テスト不足も security regression の一部として扱う。
