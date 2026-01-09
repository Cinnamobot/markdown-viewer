# mdv 使用ガイド

## クイックスタート

```bash
# ビルド
cargo build --release

# サンプルファイルを開く（視覚的なレンダリングを確認）
./target/release/mdv examples/sample.md

# READMEを開く
./target/release/mdv README.md
```

## 視覚的レンダリング

mdvは以下のマークダウン要素を視覚的にレンダリングします：

- **見出し (H1-H6)**
  - H1: 枠線付き、シアン色、太字、下線
  - H2: 縦線付き、ライトシアン色、太字
  - H3: 矢印付き、ブルー色、太字
  - H4以降: 記号付き、グレー色

- **コードブロック**
  - ボーダー付きボックス表示
  - 言語名をヘッダーに表示
  - シンタックスハイライト（syntect使用、色付けコード表示）

- **リスト**
  - カラフルな箇条書きマーカー（●）
  - ネストレベルに応じたインデント（2スペース/レベル）
  - 複数レベルのネスト対応

- **タスクリスト（チェックボックス）**
  - `- [ ]` → `[ ] ` 未チェック（黄色）
  - `- [x]` → `[✓] ` チェック済み（緑色）

- **引用文**
  - 縦線とイタリック体、黄色系で表示

- **インラインコード**
  - 黄色の背景とボールド表示

- **水平線**
  - 太い線（━）で明確に表示

- **テーブル**
  - 罫線付きボックス表示（┌─┬─┐ 形式）
  - ヘッダー行はシアン色で太字
  - 列の配置（左寄せ/中央/右寄せ）に対応
  - 列幅は自動調整（最小3文字、最大30文字）
  - 各行間に区切り線を表示

## インストール

バイナリをPATHに追加:

```bash
# Linux/macOS
cp target/release/mdv ~/.local/bin/
# または
sudo cp target/release/mdv /usr/local/bin/

# その後、どこからでも使用可能
mdv README.md
```

## 基本的な使い方

### ファイルを開く

```bash
mdv your-document.md
```

### ライブリロードを無効にする

```bash
mdv -n your-document.md
```

ファイル編集中にビューアーが勝手に更新されるのを防ぎます。

### 目次を表示して起動

```bash
mdv --show-toc your-document.md
```

起動時から目次が表示されます。後から`t`キーで切り替えも可能。

### 特定の行にジャンプ

```bash
mdv -l 100 your-document.md
```

100行目から表示を開始します。

### 見出しにジャンプ

```bash
mdv -H "Installation" README.md
```

"Installation"という見出しにジャンプします（部分一致）。

## キーボード操作

### 基本移動

- `j` または `↓` - 1行下にスクロール
- `k` または `↑` - 1行上にスクロール
- `PageDown` - 1ページ下にスクロール
- `PageUp` - 1ページ上にスクロール
- `g` - ファイルの先頭にジャンプ
- `G` (Shift+g) - ファイルの末尾にジャンプ

### 目次操作

- `t` - 目次の表示/非表示を切り替え
- 目次表示中に `j`/`k` - 見出しを選択
- 目次表示中に `Enter` - 選択した見出しにジャンプ

### 終了

- `q` - 終了
- `Ctrl+C` - 終了

## 実用例

### AIエージェントの出力を確認

```bash
# AIエージェントにマークダウン形式で出力させる
your-ai-agent > output.md

# mdvで確認（ライブリロード有効）
mdv output.md
```

AIエージェントが出力を更新するたびに、自動的に表示が更新されます。

### ドキュメント執筆

別のターミナルでエディタを開き、mdvでリアルタイムプレビュー:

```bash
# ターミナル1
vim document.md

# ターミナル2
mdv document.md
```

保存するたびに自動的に表示が更新されます。

### ssh経由での使用

```bash
ssh your-server
mdv /path/to/document.md
```

リモートサーバーでも快適に動作します。

## テーマ変更

利用可能なテーマ:

- base16-ocean.dark (デフォルト)
- base16-eighties.dark
- base16-mocha.dark
- InspiredGitHub
- Solarized (dark)
- Solarized (light)

```bash
mdv -t "InspiredGitHub" README.md
```

## トラブルシューティング

### 文字化けする

ターミナルのロケールがUTF-8に設定されているか確認:

```bash
locale | grep UTF-8
```

### 色が正しく表示されない

ターミナルがtrue colorに対応しているか確認。または、別のテーマを試す。

### ライブリロードが動作しない

- `-n` オプションを付けていないか確認
- ファイルシステムがinotify対応か確認（一部のネットワークドライブでは動作しない可能性）

## パフォーマンス

### ベンチマーク結果

- 起動時間: ~11ms
- メモリ使用量: ~8MB
- バイナリサイズ: ~3MB

### 比較

| ツール | メモリ使用量 | 起動時間 |
|--------|------------|---------|
| mdv | 8MB | 11ms |
| ブラウザ | 200-500MB | 1-3秒 |
| VSCode | 200-400MB | 2-5秒 |

## 開発

### デバッグビルド

```bash
cargo build
./target/debug/mdv README.md
```

### リリースビルド

```bash
cargo build --release
./target/release/mdv README.md
```

### テスト実行

```bash
cargo test
```
