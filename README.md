# raden-py

[raden](https://github.com/shiguredo/raden) の Python バインディング。

PyO3 / maturin / uv を使ってビルドする。

## 要件

- Rust 1.91 以降
- uv
- Python 3.12 以降

## 開発

```bash
uv sync --extra dev
uv run maturin develop
uv run pytest
```

## 基本的な使い方

```python
import raden

ctx = raden.Context(800, 600, raden.PixelFormat.Prgb32)

# 背景を白で塗りつぶす。
ctx.set_fill_style(raden.Rgba32.rgb(255, 255, 255))
ctx.fill_all()

# 赤い円を描画する。
ctx.set_fill_style(raden.Rgba32.rgb(255, 0, 0))
ctx.fill_circle(raden.Circle(200, 200, 100))

# 緑の矩形を描画する。
ctx.set_fill_style(raden.Rgba32.rgb(0, 255, 0))
ctx.fill_rect(raden.Rect(350, 150, 200, 150))

# 青い三角形を描画する。
ctx.set_fill_style(raden.Rgba32.rgb(0, 0, 255))
ctx.fill_polygon([
    raden.Point(600, 400),
    raden.Point(700, 200),
    raden.Point(750, 450),
])

ctx.save_bmp("basic.bmp")
```

## 動画表示（examples のみ）

ライブラリ本体は動画表示機能を持たない。examples では PyPI の [raw-player](https://pypi.org/project/raw-player/) を使ってウィンドウ表示を行う。

```python
import numpy as np
from raw_player import VideoPlayer

import raden

ctx = raden.Context(640, 480, raden.PixelFormat.Prgb32)
# ... 描画 ...

player = VideoPlayer(640, 480, title="Raden")
player.play()
player.enqueue_video_bgra(
    np.frombuffer(ctx.data(), dtype=np.uint8).reshape((ctx.height(), ctx.width(), 4)),
    pts_us=0,
)
```

## テスト

```bash
uv run pytest
```

## サンプル

- `examples/basic.py`: 図形を描画して `basic.bmp` に保存する。
- `examples/animation.py`: `raw_player.VideoPlayer` を使ったアニメーション表示例。
