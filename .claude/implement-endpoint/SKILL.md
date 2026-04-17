---
name: implement-endpoint
description: "1 エンドポイントを実装する定型手順。schema → handler → service → repository → route → test の順で進める。"
argument-hint: "[エンドポイント名]"
disable-model-invocation: true
allowed-tools: Read, Grep, Glob, Edit, Write, Bash
---

# エンドポイント実装手順

1 つのエンドポイントを以下の順序で実装する。

## 入力

- `$ARGUMENTS` で指定されたエンドポイント名
- `/design-api` の出力または `docs/` 配下の API 設計書

引数がない場合は、対象のエンドポイントを確認する。

## 実装順序

以下の順に進める。各ステップで変更対象ファイルを先に列挙してから編集する。

### Step 1: Schema 定義
- Request / Response の型定義（Zod, TypeScript, OpenAPI など）
- 既存スキーマファイルのパターンを踏襲する

### Step 2: Handler / Controller
- リクエスト受付、バリデーション呼び出し、レスポンス整形
- 既存ハンドラのパターンを踏襲する

### Step 3: Service（ビジネスロジック）
- ユースケースの実装
- エラーハンドリング

### Step 4: Repository（データアクセス）
- DB クエリ、外部 API 呼び出し
- 既存リポジトリのパターンを踏襲する

### Step 5: Route 登録
- ルーティング定義への追加
- ミドルウェア（認証・認可）の適用

### Step 6: テスト
- 正常系・異常系・境界値のテスト作成
- 既存テストのパターンを踏襲する

## 各ステップの原則

- 編集前に対象ファイル一覧を列挙する
- 既存の命名規約・import ルール・ディレクトリ構成に従う
- 1 ステップ完了ごとにビルドが通ることを確認する
- 新規ファイル作成時は既存の類似ファイルをテンプレートにする

## 完了確認

- ビルドが通る
- テストが通る
- lint エラーがない
