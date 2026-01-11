# 変更履歴

このプロジェクトへの主要な変更はここに記録されます。

## [Unreleased]

## [0.2.0] - 2026-01-11

### ✨ 機能追加

- **画像表示対応** ([#1](https://github.com/Cinnamobot/markdown-viewer/issues/1), [#16](https://github.com/Cinnamobot/markdown-viewer/pull/16))
  - Markdown内の画像をパス表示で認識可能に
  - `[Image: path.png]` と `Alt: description` の形式で表示
- **GitHub Alerts対応** ([#2](https://github.com/Cinnamobot/markdown-viewer/issues/2), [#16](https://github.com/Cinnamobot/markdown-viewer/pull/16))
  - NOTE, TIP, IMPORTANT, WARNING, CAUTIONの5タイプに対応
  - 各タイプごとに色分けして視覚的に表示
- **シンタックスハイライトの色数向上** ([#3](https://github.com/Cinnamobot/markdown-viewer/issues/3), [#16](https://github.com/Cinnamobot/markdown-viewer/pull/16))
  - デフォルトテーマを`base16-eighties.dark`に変更
  - より色彩豊かなコードブロック表示

### 🐛 バグ修正

- **[CRITICAL]** tokio::spawnタスクのリソースリーク修正 ([#5](https://github.com/Cinnamobot/markdown-viewer/issues/5), [#16](https://github.com/Cinnamobot/markdown-viewer/pull/16))
  - CancellationTokenによるタスク管理を実装
  - チャネル送信結果のチェックを追加
  - Dropトレイトでクリーンシャットダウンを保証
  - メモリリークと無駄なCPU使用を防止
- **[CRITICAL]** crosstermイベントポーリングのブロッキング処理改善 ([#6](https://github.com/Cinnamobot/markdown-viewer/issues/6), [#16](https://github.com/Cinnamobot/markdown-viewer/pull/16))
  - EventStreamによる真の非同期処理を実装
  - EventHandlerクラスの導入
  - UI応答性の大幅改善、ファイル変更検出の遅延解消
- エラーハンドリングと境界値チェックの改善 ([#7](https://github.com/Cinnamobot/markdown-viewer/issues/7), [#16](https://github.com/Cinnamobot/markdown-viewer/pull/16))
  - ファイル変更時のエラーログ出力を追加
  - シンタックスハイライトエラーのログ出力を追加（デバッグビルド）
  - Unicode文字幅計算で制御文字を明示的に処理
- **[CRITICAL]** シンタックスハイライトのテーマが見つからない場合にアプリケーションがクラッシュする問題を修正 ([#4](https://github.com/Cinnamobot/markdown-viewer/issues/4), [#10](https://github.com/Cinnamobot/markdown-viewer/pull/10))
  - フォールバック処理を追加し、利用可能な任意のテーマを使用するように改善
- **[CRITICAL]** ライブリロード時にTOC（目次）の選択インデックスが範囲外になりパニックする問題を修正 ([#9](https://github.com/Cinnamobot/markdown-viewer/issues/9), [#11](https://github.com/Cinnamobot/markdown-viewer/pull/11))
  - ファイル変更で見出しが減少した場合に自動的に調整されるように修正
- ライブリロードのチャネルバッファサイズを10→100に増加し、高頻度のファイル変更に対応 ([#7](https://github.com/Cinnamobot/markdown-viewer/issues/7), [#12](https://github.com/Cinnamobot/markdown-viewer/pull/12))
- 空のドキュメントや空のTOCで適切なメッセージを表示するように改善 ([#7](https://github.com/Cinnamobot/markdown-viewer/issues/7), [#13](https://github.com/Cinnamobot/markdown-viewer/pull/13))
- 不正なMarkdownで生成される空のテーブルに対する境界値チェックを追加 ([#7](https://github.com/Cinnamobot/markdown-viewer/issues/7), [#14](https://github.com/Cinnamobot/markdown-viewer/pull/14))

### 🔧 改善

- **CI/CDパイプラインの導入** ([#17](https://github.com/Cinnamobot/markdown-viewer/pull/17))
  - GitHub Actionsによる自動テスト、Clippy、フォーマットチェック、ビルドを追加
  - Cargoキャッシュによるビルド時間の最適化
  - コード品質の自動保証

### 📦 依存関係

- `tokio-util` 0.7を追加（CancellationToken用）
- `futures` 0.3を追加（EventStream用）
- `crossterm`に`event-stream`機能を追加

## [0.1.0] - 2026-01-10

### ✨ 機能追加

- ⚡ **超高速起動**: 遅延読み込みと最適化により、50ms以下で起動します。
- 🔄 **ライブリロード**: ファイルの変更を自動検知し、即座にビューを更新します。
- 🎨 **シンタックスハイライト**: `syntect`を使用した美しいコードブロック表示を実現しました。
- 📑 **目次ナビゲーション**: `t`キーで目次を表示し、各見出しへ素早くジャンプできます。
- 🌐 **日本語・全角文字対応**: 日本語（全角文字）や矢印などの記号を含むテーブルの表示崩れを完全に修正しました。
- ✨ **リッチなレンダリング**:
  - **テーブル**: 罫線付きの美しいテーブル表示と正確な列整列。
  - **コードブロック**: 言語名ラベル付きのボックス表示とシンタックスハイライト。
  - **タスクリスト**: インタラクティブなチェックボックス表示。
  - **ネストされたリスト**: 正しいインデントと階層表示。
  - **インラインコード**: バッククォートを表示せず、スタイルのみを適用して見やすく表示。

### 🐛 バグ修正

- ネストされたリストで親アイテムが表示されない問題を修正。
- インラインコードのバッククォートが文字通り表示されてしまう問題を修正。
- テーブル内で日本語（全角文字）を使用した際にレイアウトが崩れる問題を修正。
- ターミナルでの矢印記号（↓/↑）の表示幅に関する問題を修正し、テーブルのズレを解消。
- コードブロックに右側のボーダーがなかった問題を修正し、完全なボックス表示に変更。
- 長いインラインコードが省略される際に、スタイルが壊れてしまう問題を修正。
