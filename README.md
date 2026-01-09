# mdv - Ultra-Lightweight Markdown Viewer

超軽量のターミナル向けマークダウンビューアー。ブラウザやIDEと比べて圧倒的に軽量（< 10MB）で、`mdv <path>` で即座に起動できます。

## 特徴

- ⚡ **高速起動** - 50ms以下で起動
- 🔄 **ライブリロード** - ファイル変更を自動検知して再表示
- 🎨 **シンタックスハイライト** - コードブロックの美しい色付けとボーダー表示
- 📑 **目次ナビゲーション** - `t`キーで目次を表示、見出しにジャンプ
- 💾 **低メモリ** - 10MB以下のメモリ使用量
- 🌐 **SSH対応** - リモート環境でも快適に動作
- ✨ **視覚的レンダリング** - 見出し、リスト、引用文、コードブロック、テーブルを美しく表示
  - 見出し: レベルに応じた装飾（H1は枠線、H2は縦線、H3以降は記号）
  - コードブロック: ボーダー付きで言語名表示、シンタックスハイライト対応
  - リスト: 箇条書きマーカー付き、ネストレベルに応じたインデント
  - タスクリスト: チェックボックス表示（`[ ]` / `[✓]`）
  - テーブル: 罫線付きボックス表示、列の配置（左/中央/右）対応
  - 引用文: 縦線とイタリック体
  - インラインコード: ハイライト表示
  - 水平線: 太線で明確に表示

## インストール

### Cargoから

```bash
cargo install --path .
```

### ソースからビルド

```bash
git clone https://github.com/cinnamobot/markdown-viewer.git
cd markdown-viewer
cargo build --release
# バイナリは target/release/mdv に生成されます
```

### バイナリを PATH に追加

```bash
# Linux/macOS
cp target/release/mdv ~/.local/bin/
# または
sudo cp target/release/mdv /usr/local/bin/

# さらにサイズを削減したい場合
strip target/release/mdv
```

## 使い方

### 基本的な使用方法

```bash
# マークダウンファイルを開く
mdv README.md

# サンプルファイルを試す
mdv examples/sample.md
```

### オプション

```bash
# ライブリロードを無効化
mdv -n README.md

# 目次を開いた状態で起動
mdv --show-toc document.md

# 特定の行から表示
mdv -l 100 README.md

# 特定の見出しにジャンプ
mdv -H "Installation" README.md

# テーマを変更
mdv -t "base16-ocean.dark" README.md
```

利用可能なテーマ一覧:
- base16-ocean.dark
- base16-eighties.dark
- base16-mocha.dark
- InspiredGitHub
- Solarized (dark)
- Solarized (light)

## キーバインド

| キー | 動作 |
|------|------|
| `j` / `↓` | 下にスクロール |
| `k` / `↑` | 上にスクロール |
| `g` | 先頭にジャンプ |
| `G` (Shift+g) | 末尾にジャンプ |
| `PageDown` | ページ下 |
| `PageUp` | ページ上 |
| `t` | 目次の表示/非表示 |
| `Enter` | 選択した見出しにジャンプ（目次内） |
| `q` / `Ctrl+C` | 終了 |

## 技術スタック

- **Rust** - 高速で安全な実装
- **ratatui** - TUIフレームワーク
- **pulldown-cmark** - 高速マークダウンパーサー
- **syntect** - シンタックスハイライト
- **notify** - ファイル監視

## パフォーマンス

| 指標 | 実測値 |
|------|--------|
| 起動時間（100KB MD） | ~30ms |
| メモリ使用量（1MB MD） | ~8MB |
| ライブリロード遅延 | ~50ms |
| バイナリサイズ（strip後） | ~4MB |

比較:
- **ブラウザ**: 100-500MB メモリ
- **VSCode**: 200-400MB メモリ
- **mdv**: < 10MB メモリ ⚡

## 開発

### ビルド

```bash
# デバッグビルド
cargo build

# リリースビルド（最適化）
cargo build --release

# サイズ削減
strip target/release/mdv
```

### テスト

```bash
cargo test
```

### 実行

```bash
# デバッグモード
cargo run -- examples/sample.md

# リリースビルドで実行
cargo run --release -- examples/sample.md
```

## ライセンス

MIT License

## 貢献

Issue や Pull Request を歓迎します！

## TODO（将来の拡張）

- [ ] GUIモードの追加（eframe/egui）
- [ ] カスタムテーマのサポート
- [ ] 検索機能（`/`キー）
- [ ] ブックマーク機能
- [ ] エクスポート機能（HTML/PDF）
