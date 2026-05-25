# Rust Image Processing

Rustで画像処理を学ぶためのプロジェクト。`image` クレートによる画像操作と `minifb` によるウィンドウ表示を組み合わせて、画像処理の基礎を実践する。

## プロジェクト構成

```
rust-image-processing/
├── Cargo.toml              # ワークスペース定義
├── image_core/             # 共通ライブラリ（画像読み込み・変換・表示）
│   ├── Cargo.toml
│   └── src/lib.rs
├── chapters/               # 章ごとの実行クレート
│   └── ch04/
│       ├── Cargo.toml
│       └── src/main.rs
└── assets/                 # サンプル画像
    └── sample1.jpg
```

- **image_core**: 画像の読み込み、リサイズ、グレースケール変換、ウィンドウ表示などの共通関数を提供するライブラリクレート
- **chapters/chXX**: 各章ごとの実行ファイル。`image_core` を依存として使用する

## 環境構築

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

```sh
# Rust のインストール（未インストールの場合）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### セットアップ

```sh
git clone <repository-url>
cd rust-image-processing

# 依存クレートのダウンロードとビルド
cargo build
```

### サンプル画像の配置

`assets/` ディレクトリにサンプル画像を配置する。

```
assets/sample1.jpg
```

## 実行方法

章ごとのクレートを `-p` オプションで指定して実行する。

```sh
# ch04 を実行
cargo run -p ch04
```

ウィンドウが開き、画像が表示される。ESC キーでウィンドウを閉じる。

## image_core の主要関数

| 関数 | 説明 |
|------|------|
| `read_image(path)` | 画像ファイルを読み込んで `DynamicImage` を返す |
| `resize_image(img, w, h)` | 画像を指定サイズにリサイズ |
| `image_to_grayscale(img)` | グレースケールに変換 |
| `display_images(images)` | 複数の `DynamicImage` をウィンドウに縦並びで表示 |

## 開発ツール

```sh
# コード品質チェック
cargo clippy

# フォーマット
cargo fmt

# テスト
cargo test
```

## VSCode でのデバッグ

1. [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) 拡張をインストール
2. `.vscode/launch.json` が設定済み
3. ブレークポイントを設置して F5 でデバッグ開始
