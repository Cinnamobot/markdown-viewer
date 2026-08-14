# エントリーフォーム回答ドラフト(U-22 プログラミング・コンテスト2026)

> 実際のフォーム入力前に、プロジェクト情報(GitHub URL)を手元に準備してください。
> フォームは一時保存をこまめに行うこと。

## 基本情報

| 項目 | 回答 |
|------|------|
| 作品名 | mdv - 超軽量ターミナル向けMarkdownビューア(タイトル案と一致させる) |
| 応募区分 | 個人 |
| 作品ジャンル | **ユーティリティ** |
| 制作者名 | (本名 or ニックネームを入力。審査通過時は本名で掲載) |
| 所属 | (学校/団体名。なければ「なし」) |
| 生年月日・年齢 | (本人情報。18歳以上のため保護者同意書は不要) |
| 連絡先E-mail / 電話番号 | (確実に連絡が取れるものを入力) |
| 作品URL(GitHub等) | https://github.com/Cinnamobot/markdown-viewer |
| ProtoPedia URL | (登録後に取得したURLを入力) |
| 作品説明動画URL | https://youtu.be/XXXX(YouTube限定公開) |

## 動作&開発環境

| 項目 | 回答 |
|------|------|
| 動作プラットフォーム | Windows 10/11、macOS、Linux(主要ディストリビューション) |
| 動作確認済み環境 | Windows 11 + Windows Terminal、Ubuntu(GitHub Actions CI) |
| 必要ソフトウェア | なし(シングルバイナリ。ターミナルでのみ動作) |
| ビルド環境 | Rust 1.70以降 + Cargo(Linux/macOS/Windows) |
| 開発言語 | Rust 2021 edition |
| 利用ライブラリ | ratatui, crossterm, pulldown-cmark, syntect, notify, tokio ほか |
| 特別なハードウェア | なし |

## インストール手順(審査員向け)

```bash
# 1. ソースコードからビルド(要 Rust 1.70+)
cargo build --release

# 2. バイナリをPATHに配置(例: Linux/macOS)
sudo cp target/release/mdv /usr/local/bin/

# Windowsの場合は mdv.exe を任意のフォルダに置き、PATHに追加
```

## 動作確認手順(審査員向け)

```bash
# 1. 同梱のサンプルデータを開く(全機能のデモが含まれる)
mdv examples/sample.md

# 2. 目次を開いた状態で起動
mdv --show-toc examples/sample.md

# 3. ライブリロードの確認: 別ターミナルで sample.md を編集して保存
#    → ビューアの表示が自動更新される

# 4. ヘルプ表示(キーバインド一覧): ? キー
#    終了: q キー または Ctrl+C
```

サンプルデータ `examples/sample.md` には、見出し・コードブロック(Rust/Python/JavaScript)・
ネストリスト・タスクリスト・テーブル・引用・GitHub Alerts・水平線を含めてあります。

## 参考にしたソフトウェアと相違点(必須項目)

- **既存のマークダウンビューアは参考にしていない。**
- 使用したOSSライブラリはすべて一般的なライブラリ(ratatui / pulldown-cmark /
  syntect / notify / tokio / crossterm / clap)であり、ライセンスは
  「別添参照_LICENSE-DEPENDENCIES.txt」に一覧記載。
- 類似のターミナルMarkdownビューア(glow等)との相違点(アピール用の参考情報):
  - 起動約30ms・メモリ約8MBの超軽量設計
  - GitHub Alerts(Note/Tip/Warning等)の視覚的レンダリング
  - 日本語全角文字を含むテーブルでも崩れないUnicode対応レイアウト
  - シングルバイナリで依存ソフト不要、SSH環境でも動作

## 未応募の確認

- [ ] 本作品はU-22 プログラミング・コンテスト2026以外のコンテストに応募していない
- [ ] ソースコードはGitHubで公開済みだが「作品公開有無は問わない」要件に合致する

## 提出資料チェックリスト(事務局ストレージへアップロード)

- [ ] ソースコード一式(リポジトリのzip: Cargo.lock込み)
- [ ] プログラムファイル一式(Windows / macOS / Linuxの各バイナリ)
- [ ] サンプルデータ(examples/sample.md)
- [ ] 作品説明資料(README.md、USAGE.md)
- [ ] 別添参照_LICENSE-DEPENDENCIES.txt(依存OSSライセンス一覧)
- [ ] ウイルスチェック実施
- [ ] ファイル名は内容が分かる命名にする(「ソースコード」「プログラムファイル」等)
