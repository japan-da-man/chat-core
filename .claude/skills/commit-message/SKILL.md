---
name: commit-message
description: "Conventional Commits 形式のコミットメッセージと PR 説明文を生成する。コミット時や PR 作成時に使う。"
argument-hint: "[commit or pr]"
disable-model-invocation: true
allowed-tools: Bash
---

# コミットメッセージ / PR 説明文

## 差分の確認

- コミットメッセージの場合: !`git diff --cached --stat`
- PR 説明文の場合: !`git log --oneline main..HEAD`

## コミットメッセージ（Conventional Commits）

`$ARGUMENTS` が `commit` または引数なしの場合:

1. `git diff --cached` でステージ済みの変更を確認する
2. 以下の形式でコミットメッセージを生成する

```
<type>(<scope>): <summary>

<body>
```

### type 一覧
- `feat`: 新機能
- `fix`: バグ修正
- `refactor`: リファクタリング
- `test`: テスト追加・修正
- `docs`: ドキュメント変更
- `chore`: ビルド・設定・依存関係の変更
- `perf`: パフォーマンス改善
- `ci`: CI 設定の変更

### ルール
- summary は日本語で 50 文字以内
- body は「なぜ」を書く（「何を」は diff で分かる）
- 1 コミット 1 責務

## PR 説明文

`$ARGUMENTS` が `pr` の場合:

1. `git log --oneline main..HEAD` で含まれるコミットを確認する
2. 以下の形式で PR 説明文を生成する

```
## 概要
<1-3 行で変更の目的>

## 変更内容
- <箇条書きで主要な変更>

## テスト
- <テスト方法・確認事項>

## 影響範囲
- <影響を受けるコンポーネント>

## 関連
- <関連 issue や docs>
```
