# raden-py 設計

## 概要

`raden` の Python バインディング。

- ビルド: uv + maturin + pyo3
- 動画表示: PyPI の `raw-player`
- Rust 拡張モジュール名: `raden._raden`
- Python ソースルート: `python/`

## プロジェクト構造

```
raden-py/
├── Cargo.toml           # Rust クレート設定
├── pyproject.toml       # maturin / Python プロジェクト設定
├── src/                 # Rust 拡張ソース
│   └── lib.rs           # エントリポイント
│   └── context.rs
│   └── enums.rs
│   └── font.rs
│   └── geometry.rs
│   └── gradient.rs
│   └── image.rs
│   └── matrix.rs
│   └── path.rs
│   └── pattern.rs
│   └── style.rs
├── python/raden/        # Python パッケージ
│   └── __init__.py
│   └── player.py        # raw-player 連携ヘルパー
├── tests/               # pytest テスト
└── examples/            # サンプルスクリプト
```

## Rust 側で露出する型

- `Rgba32`: `__new__(r, g, b, a=255)`, `rgb(r,g,b)`, プロパティ `r/g/b/a`
- 列挙型: `CompOp`, `FillRule`, `StrokeCap`, `StrokeJoin`, `PixelFormat`, `ExtendMode`, `PatternFilter`
- 幾何型: `Rect`, `Circle`, `Ellipse`, `RoundRect`, `Triangle`, `Line`, `Arc`, `Point`
- `Matrix2D`: コンストラクタと行列操作
- `Path`: パス構築 API
- `Gradient`: Linear / Radial / Conic
- `Pattern`: 画像パターン
- `FontData`, `FontFace`, `Font`, `FontError`
- `Image`: `new(width, height, format)`, `width()`, `height()`, `stride()`, `format()`, `data()` (bytes), `save_bmp(path)`
- `Context`: `Image` と `PipelineRuntime` を内部に保持。raden の `Context` API をなるべくそのまま露出。
  - 描画メソッド: `clear_all`, `clear_rect`, `fill_all`, `fill_rect`, `fill_circle`, `fill_ellipse`, `fill_triangle`, `fill_round_rect`, `fill_pie`, `fill_polygon`, `fill_path`, `fill_text`
  - ストロークメソッド: `stroke_path`, `stroke_rect`, `stroke_circle`, `stroke_ellipse`, `stroke_triangle`, `stroke_round_rect`, `stroke_polygon`, `stroke_polyline`, `stroke_line`
  - スタイル設定: `set_fill_style`, `set_fill_style_gradient`, `set_fill_style_pattern`, `set_stroke_style`, `set_stroke_style_gradient`, `set_stroke_style_pattern`, `set_comp_op`, `set_fill_rule`, `set_global_alpha`, `set_fill_alpha`, `set_stroke_alpha`
  - 状態: `save`, `restore`, `clip_to_rect`, `restore_clipping`
  - 変換: `translate`, `scale`, `rotate`, `skew`, `apply_matrix`, `reset_matrix`, `rotate_around`, `post_translate`, `post_scale`, `post_rotate`, `post_skew`, `post_transform`
  - ストローク属性: `set_stroke_width`, `set_stroke_cap`, `set_stroke_start_cap`, `set_stroke_end_cap`, `set_stroke_join`, `set_stroke_miter_limit`, `set_stroke_dash_array`, `set_stroke_dash_offset`
  - 転送: `blit_image_rect`, `blit_image_at`
  - 画像アクセサ: `width()`, `height()`, `data()` (bytes)

## Python 側の連携

- `raden/__init__.py` ですべての Rust クラスを再エクスポート。
- `raden/player.py` で `RadenPlayer` クラスを提供。
  - `raw_player.VideoPlayer` をラップ。
  - `enqueue(image_or_context)` で `raden` の画像データを BGRA 形式の numpy 配列に変換して enqueue_video_bgra へ渡す。
  - `Context` からは `width()`, `height()`, `data()` を使って変換する。

## テスト・サンプル

- `tests/test_raden.py`: クラス作成、描画、BMP 保存、フォント読み込みなどの単体テスト。
- `examples/basic.py`: 図形を描画して `basic.bmp` を保存。
- `examples/animation.py`: `RadenPlayer` を使ったアニメーション例（表示が必要）。
