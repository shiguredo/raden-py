# raden-py

[![PyPI](https://img.shields.io/pypi/v/raden.svg)](https://pypi.org/project/raden/)
[![SPEC 0 — Minimum Supported Dependencies](https://img.shields.io/badge/SPEC-0-green?labelColor=%23004811&color=%235CA038)](https://scientific-python.org/specs/spec-0000/)
[![image](https://img.shields.io/pypi/pyversions/raden.svg)](https://pypi.python.org/pypi/raden)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Actions status](https://github.com/shiguredo/raden-py/workflows/wheel/badge.svg)](https://github.com/shiguredo/raden-py/actions)

## About Shiguredo's open source software

We will not respond to PRs or issues that have not been discussed on Discord. Also, Discord is only available in Japanese.

Please read <https://github.com/shiguredo/oss/blob/master/README.en.md> before use.

## 時雨堂のオープンソースソフトウェアについて

利用前に <https://github.com/shiguredo/oss> をお読みください。

## raden-py について

[Cranelift](https://cranelift.dev/) JIT を利用した 2D ベクターグラフィックスライブラリ [raden](https://github.com/shiguredo/raden) の Python バインディングです。

raden-py と [raw-player](https://github.com/shiguredo/raw-player) を組み合わせて 60 fps での映像なども描画可能です。

## 特徴

- Cranelift JIT によるネイティブコード生成
- 29 種類の合成モード (Porter-Duff 合成 + ブレンドモード)
- 基本図形の描画 (四角形、円、楕円、角丸矩形、三角形、扇形、ポリゴン)
- ベクターパスによる自由な図形描画 (直線、ベジェ曲線、円弧、円錐曲線)
- 線形/放射状/円錐グラデーション
- パターン塗りつぶし
- ストローク描画 (線幅、キャップ、ジョイン、ダッシュ)
- アルファブレンディング
- 座標変換 (平行移動、スケール、回転、せん断、行列)
- クリッピング (矩形)
- テキスト描画 (TrueType アウトライン)
- BMP 出力
- バッファプロトコルによる memoryview / numpy 配列への直接アクセス

## 対応プラットフォーム

- macOS 26 arm64
- macOS 15 arm64
- Ubuntu 26.04 x86_64
- Ubuntu 26.04 arm64
- Ubuntu 24.04 x86_64
- Ubuntu 24.04 arm64
- Ubuntu 22.04 x86_64
- Ubuntu 22.04 arm64
- Windows Server 2025 x86_64
- Windows 11 x86_64

## 対応 Python

- 3.14
- 3.14t
- 3.13
- 3.12

## インストール

```bash
uv add raden
```

## 使い方

### 基本的な図形の描画

```python
import raden

# 画像サイズを指定
ctx = raden.Context(640, 360, raden.PixelFormat.Prgb32)

# 背景を黒で塗りつぶし
ctx.set_fill_style(raden.Rgba32.rgb(0, 0, 0))
ctx.fill_all()

# 中心に白い円を描画
ctx.set_fill_style(raden.Rgba32.rgb(255, 255, 255))
ctx.fill_circle(raden.Circle(320, 180, 100))

# BMP として保存
ctx.save_bmp("output.bmp")
```

### 四角形と透明度の例

```python
import raden

ctx = raden.Context(640, 480, raden.PixelFormat.Prgb32)

# 背景を白で塗りつぶし
ctx.set_fill_style(raden.Rgba32.rgb(255, 255, 255))
ctx.fill_all()

# 半透明の赤い四角形を描画
ctx.set_comp_op(raden.CompOp.SrcOver)  # アルファブレンディングを有効化
ctx.set_fill_style(raden.Rgba32(255, 0, 0, 128))  # 赤、50% 透明
ctx.fill_rect(raden.Rect(50, 50, 200, 150))

# 半透明の青い四角形を重ねて描画
ctx.set_fill_style(raden.Rgba32(0, 0, 255, 128))  # 青、50% 透明
ctx.fill_rect(raden.Rect(150, 100, 200, 150))

ctx.save_bmp("output.bmp")
```

### パスを使った図形描画

```python
import raden

w, h = 640, 480
ctx = raden.Context(w, h, raden.PixelFormat.Prgb32)

# パスで三角形を作成
path = raden.Path()
path.move_to(w / 2, h / 4)      # 頂点
path.line_to(w / 4, 3 * h / 4)  # 左下
path.line_to(3 * w / 4, 3 * h / 4)  # 右下
path.close()                    # パスを閉じる

# 背景を暗い灰色に
ctx.set_fill_style(raden.Rgba32.rgb(40, 40, 40))
ctx.fill_all()

# 複数の色で重ねて描画
colors = [(255, 100, 100), (100, 255, 100), (100, 100, 255)]
for i, color in enumerate(colors):
    ctx.save()  # 現在の状態を保存
    ctx.translate(i * 10, i * 10)  # 少しずつずらす
    ctx.set_fill_style(raden.Rgba32(*color, 200 - i * 50))  # 透明度を変える
    ctx.fill_path(path)
    ctx.restore()  # 状態を復元

ctx.save_bmp("output.bmp")
```

### 複数の図形を組み合わせる例

```python
import colorsys
import math

import raden

w, h = 640, 480
ctx = raden.Context(w, h, raden.PixelFormat.Prgb32)

# 背景を黒に
ctx.set_fill_style(raden.Rgba32.rgb(0, 0, 0))
ctx.fill_all()

# 円を円形に配置
ctx.translate(w / 2, h / 2)  # 中心に移動
num_circles = 12
radius = 100

for i in range(num_circles):
    angle = 2 * math.pi * i / num_circles
    x = radius * math.cos(angle)
    y = radius * math.sin(angle)

    # 虹色のグラデーション
    hue = i / num_circles
    r, g, b = colorsys.hsv_to_rgb(hue, 1.0, 1.0)
    ctx.set_fill_style(raden.Rgba32(int(r * 255), int(g * 255), int(b * 255), 200))
    ctx.fill_circle(raden.Circle(x, y, 20))  # 小さな円を描画

ctx.save_bmp("output.bmp")
```

### グラデーションの描画

```python
import raden

ctx = raden.Context(640, 480, raden.PixelFormat.Prgb32)

# 線形グラデーションを作成
grad = raden.Gradient.new_linear(0, 0, 640, 0)
grad.add_stop(0.0, raden.Rgba32.rgb(255, 0, 0))    # 赤
grad.add_stop(0.5, raden.Rgba32.rgb(0, 255, 0))    # 緑
grad.add_stop(1.0, raden.Rgba32.rgb(0, 0, 255))    # 青

ctx.set_fill_style_gradient(grad)
ctx.fill_all()

ctx.save_bmp("output.bmp")
```

### テキスト描画

```python
import raden

w, h = 640, 480
ctx = raden.Context(w, h, raden.PixelFormat.Prgb32)

# フォントをロード
font_data = raden.FontData.from_file("/System/Library/Fonts/Helvetica.ttc")
font_face = raden.FontFace.from_data(font_data, 0)
font = raden.Font.from_face(font_face, 48.0)

# 背景を白で塗りつぶし
ctx.set_fill_style(raden.Rgba32.rgb(255, 255, 255))
ctx.fill_all()

# テキストを描画
ctx.set_fill_style(raden.Rgba32.rgb(0, 0, 0))
ctx.fill_text(50, 100, font, "Hello, Raden!")

ctx.save_bmp("output.bmp")
```

### memoryview / numpy でピクセルデータにアクセス

Image と Context はバッファプロトコル (PEP 3118) に対応している。`memoryview()` や `np.asarray()` でピクセルデータにコピーなしでアクセスできる。

```python
import numpy as np
import raden

ctx = raden.Context(640, 480, raden.PixelFormat.Prgb32)
ctx.set_fill_style(raden.Rgba32.rgb(255, 0, 0))
ctx.fill_all()

# Context.image() で内部の Image を取得し、numpy 配列としてアクセス
arr = np.asarray(ctx.image())
print(arr.shape)  # (480, 640, 4) uint8

# memoryview でもアクセス可能
mv = memoryview(ctx.image())
print(mv.shape)   # (480, 640, 4)
print(mv.format)  # "B" (unsigned char)
```

### 動画表示

ライブラリ本体は動画表示機能を持たない。PyPI の [raw-player](https://pypi.org/project/raw-player/) を使ってウィンドウ表示を行う。

```python
import numpy as np
from raw_player import VideoPlayer

import raden

ctx = raden.Context(640, 480, raden.PixelFormat.Prgb32)
# ... 描画 ...

player = VideoPlayer(640, 480, title="Raden")
player.play()
# np.asarray でコピーなしにピクセルデータを取得
player.enqueue_video_bgra(
    np.asarray(ctx.image()),
    pts_us=0,
)
```

> [!NOTE]
>
> - テキスト描画は TrueType アウトライン (glyf/loca) のみ対応です (CFF、カーニング、シェーピングは未対応)
> - `data()` はピクセルデータのコピーを bytes で返します。コピーなしでアクセスするには `memoryview(img)` または `np.asarray(img)` を使用してください
> - バッファプロトコル経由のビューは Image の寿命に依存します

## API リファレンス

### Context

描画コンテキストを管理するクラス。内部に画像を所有し、描画状態 (スタイル、変換、クリップ) を保持する。

```python
ctx = raden.Context(width, height, format=raden.PixelFormat.Prgb32)
```

#### 画像アクセサ

| メソッド | 説明 |
|----------|------|
| `width()` | 画像の幅を返す |
| `height()` | 画像の高さを返す |
| `data()` | ピクセルデータを bytes として返す (コピー) |
| `save_bmp(path)` | BMP ファイルとして保存する |
| `image()` | 内部の Image を返す (バッファプロトコル対応) |

#### 状態保存・復元

| メソッド | 説明 |
|----------|------|
| `save()` | 現在の描画状態をスタックに保存 |
| `restore()` | スタックから描画状態を復元 |

#### クリップ

| メソッド | 説明 |
|----------|------|
| `clip_to_rect(rect)` | クリップ領域を指定矩形との積集合に縮小 |
| `restore_clipping()` | クリップ領域を画像境界にリセット |

#### 塗りつぶしスタイル

| メソッド | 説明 |
|----------|------|
| `set_fill_style(color)` | 塗りつぶし色を RGBA で設定 |
| `set_fill_style_gradient(gradient)` | 塗りつぶしをグラデーションに設定 |
| `set_fill_style_pattern(pattern)` | 塗りつぶしをパターンに設定 |

#### ストロークスタイル

| メソッド | 説明 |
|----------|------|
| `set_stroke_style(color)` | ストローク色を RGBA で設定 |
| `set_stroke_style_gradient(gradient)` | ストロークをグラデーションに設定 |
| `set_stroke_style_pattern(pattern)` | ストロークをパターンに設定 |

#### 合成・アルファ

| メソッド | 説明 |
|----------|------|
| `set_comp_op(op)` | 合成モードを設定 |
| `set_fill_rule(rule)` | 塗りつぶし規則を設定 |
| `set_global_alpha(a)` | グローバルアルファを設定 (0.0〜1.0) |
| `set_fill_alpha(a)` | fill アルファを設定 (0.0〜1.0) |
| `set_stroke_alpha(a)` | stroke アルファを設定 (0.0〜1.0) |

#### 座標変換

後乗算 (`matrix = matrix * T`) は Blend2D / SVG / Canvas 互換。前乗算 (`post_*`) も提供する。

| メソッド | 説明 |
|----------|------|
| `translate(tx, ty)` | 平行移動 (後乗算) |
| `scale(sx, sy)` | スケーリング (後乗算) |
| `rotate(rad)` | 回転 (後乗算、ラジアン) |
| `rotate_around(rad, cx, cy)` | 指定中心まわりの回転 (後乗算) |
| `skew(kx, ky)` | せん断 (後乗算) |
| `apply_matrix(m)` | 任意行列を後乗算で適用 |
| `reset_matrix()` | 変換行列を単位行列にリセット |
| `post_translate(tx, ty)` | 平行移動 (前乗算) |
| `post_scale(sx, sy)` | スケーリング (前乗算) |
| `post_rotate(rad)` | 回転 (前乗算) |
| `post_skew(kx, ky)` | せん断 (前乗算) |
| `post_transform(m)` | 任意行列を前乗算で適用 |
| `matrix()` | 現在の変換行列を取得 |
| `user_to_meta()` | ユーザ行列を単位行列にリセット |

#### 塗りつぶし描画

| メソッド | 説明 |
|----------|------|
| `clear_all()` | 画像全体をクリア (ピクセル値 0) |
| `clear_rect(rect)` | 指定矩形をクリア |
| `fill_all()` | 全体を塗りつぶし |
| `fill_rect(rect)` | 矩形を塗りつぶし |
| `fill_circle(circle)` | 円を塗りつぶし |
| `fill_ellipse(ellipse)` | 楕円を塗りつぶし |
| `fill_triangle(triangle)` | 三角形を塗りつぶし |
| `fill_round_rect(round_rect)` | 角丸矩形を塗りつぶし |
| `fill_pie(arc)` | 扇形を塗りつぶし |
| `fill_polygon(points)` | ポリゴンを塗りつぶし (Point または (x, y) タプル) |
| `fill_path(path)` | パスを塗りつぶし |
| `fill_text(x, y, font, text)` | テキストを描画 |

#### ストローク属性

| メソッド | 説明 |
|----------|------|
| `set_stroke_width(width)` | ストローク幅を設定 |
| `set_stroke_cap(cap)` | キャップを両端に一括設定 |
| `set_stroke_start_cap(cap)` | 始点キャップを設定 |
| `set_stroke_end_cap(cap)` | 終点キャップを設定 |
| `set_stroke_join(join)` | ジョインを設定 |
| `set_stroke_miter_limit(limit)` | マイターリミットを設定 |
| `set_stroke_dash_array(dash_array)` | ダッシュパターンを設定 |
| `set_stroke_dash_offset(offset)` | ダッシュオフセットを設定 |

#### ストローク描画

| メソッド | 説明 |
|----------|------|
| `stroke_path(path)` | パスのストローク |
| `stroke_rect(rect)` | 矩形のストローク |
| `stroke_circle(circle)` | 円のストローク |
| `stroke_ellipse(ellipse)` | 楕円のストローク |
| `stroke_triangle(triangle)` | 三角形のストローク |
| `stroke_round_rect(round_rect)` | 角丸矩形のストローク |
| `stroke_polygon(points)` | ポリゴンのストローク (閉じる) |
| `stroke_polyline(points)` | 折れ線のストローク (閉じない) |
| `stroke_line(line)` | 線分のストローク |

#### 転送

| メソッド | 説明 |
|----------|------|
| `blit_image_rect(dst, src, src_rect=None)` | ソース画像を dst 矩形に転送 (Nearest) |
| `blit_image_at(x, y, src)` | ソース画像全体を (x, y) に転送 (Nearest) |

#### ゲッター

| メソッド | 説明 |
|----------|------|
| `comp_op()` | 合成モードを返す |
| `fill_rule()` | 塗りつぶし規則を返す |
| `fill_color()` | 塗りつぶし色を返す |
| `stroke_color()` | ストローク色を返す |
| `fill_gradient()` | 塗りつぶしグラデーションを返す (未設定なら None) |
| `fill_pattern()` | 塗りつぶしパターンを返す (未設定なら None) |
| `stroke_gradient()` | ストロークグラデーションを返す (未設定なら None) |
| `stroke_pattern()` | ストロークパターンを返す (未設定なら None) |
| `stroke_width()` | ストローク幅を返す |
| `stroke_miter_limit()` | マイターリミットを返す |
| `stroke_join()` | ジョインを返す |
| `stroke_start_cap()` | 始点キャップを返す |
| `stroke_end_cap()` | 終点キャップを返す |
| `stroke_dash_array()` | ダッシュパターンを返す |
| `stroke_dash_offset()` | ダッシュオフセットを返す |
| `global_alpha()` | グローバルアルファを返す |
| `fill_alpha()` | fill アルファを返す |
| `stroke_alpha()` | stroke アルファを返す |

### Image

画像バッファを管理するクラス。バッファプロトコル (PEP 3118) に対応しており、`memoryview(img)` や `np.asarray(img)` でピクセルデータに直接アクセスできる。Pattern のソースや `blit_image_rect` のソースとしても使用する。

```python
img = raden.Image(width, height, raden.PixelFormat.Prgb32)
```

| メソッド | 説明 |
|----------|------|
| `width()` | 画像の幅を返す |
| `height()` | 画像の高さを返す |
| `stride()` | ストライドを返す |
| `format()` | ピクセル形式を返す |
| `data()` | ピクセルデータを bytes として返す (コピー) |
| `save_bmp(path)` | BMP ファイルとして保存する |

> [!NOTE]
>
> - `memoryview(img)` で読み取り専用の memoryview を取得できる (Prgb32/Xrgb32 は (H, W, 4)、A8 は (H, W))
> - `np.asarray(img)` で numpy 配列を取得できる (データのコピーは発生しない)
> - バッファプロトコル経由のビューは Image の寿命に依存する

### Rgba32

32-bit RGBA カラー値。

```python
raden.Rgba32(r, g, b, a=255)
raden.Rgba32.rgb(r, g, b)  # アルファ 255
```

| プロパティ | 説明 |
|------------|------|
| `r` | 赤成分 |
| `g` | 緑成分 |
| `b` | 青成分 |
| `a` | アルファ成分 |

### 幾何型

#### Rect

```python
raden.Rect(x, y, w, h)
```

プロパティ: `x`, `y`, `w`, `h`

#### Circle

```python
raden.Circle(cx, cy, r)
```

プロパティ: `cx`, `cy`, `r`

#### Ellipse

```python
raden.Ellipse(cx, cy, rx, ry)
```

プロパティ: `cx`, `cy`, `rx`, `ry`

#### RoundRect

```python
raden.RoundRect(x, y, w, h, rx, ry)
```

プロパティ: `x`, `y`, `w`, `h`, `rx`, `ry`

#### Triangle

```python
raden.Triangle(x0, y0, x1, y1, x2, y2)
```

プロパティ: `x0`, `y0`, `x1`, `y1`, `x2`, `y2`

#### Line

```python
raden.Line(x0, y0, x1, y1)
```

プロパティ: `x0`, `y0`, `x1`, `y1`

#### Arc

```python
raden.Arc(cx, cy, rx, ry, start, sweep)
```

プロパティ: `cx`, `cy`, `rx`, `ry`, `start`, `sweep`

#### Point

```python
raden.Point(x, y)
```

プロパティ: `x`, `y` (読み書き可能)

### Matrix2D

2D アフィン変換行列。

```python
m = raden.Matrix2D(m00, m01, m10, m11, m20, m21)
```

| スタティックメソッド | 説明 |
|----------------------|------|
| `identity()` | 単位行列を返す |
| `translation(tx, ty)` | 平行移動行列を返す |
| `scaling(sx, sy)` | スケーリング行列を返す |
| `rotation(angle)` | 回転行列を返す (ラジアン) |
| `skewing(kx, ky)` | せん断行列を返す |

| メソッド | 説明 |
|----------|------|
| `is_identity()` | 単位行列かどうか |
| `multiply(other)` | 行列を合成 (self * other) |
| `map_point(x, y)` | 点 (x, y) を変換して (x, y) を返す |
| `translate(tx, ty)` | 平行移動を後乗算 |
| `scale(sx, sy)` | スケーリングを後乗算 |
| `rotate(angle)` | 回転を後乗算 (ラジアン) |
| `rotate_around(angle, cx, cy)` | 指定中心まわりの回転を後乗算 |
| `skew(kx, ky)` | せん断を後乗算 |
| `apply_matrix(m)` | 任意行列を後乗算 |
| `reset()` | 単位行列にリセット |
| `post_translate(tx, ty)` | 平行移動を前乗算 |
| `post_scale(sx, sy)` | スケーリングを前乗算 |
| `post_rotate(angle)` | 回転を前乗算 |
| `post_skew(kx, ky)` | せん断を前乗算 |
| `post_transform(m)` | 任意行列を前乗算 |
| `invert()` | 逆行列を返す (非可換なら None) |

| プロパティ | 説明 |
|------------|------|
| `m00` / `m01` / `m10` / `m11` / `m20` / `m21` | 行列の各要素 |

### Path

ベクターパスを作成するクラス。

```python
path = raden.Path()
path.move_to(x, y)
path.line_to(x, y)
path.close()
```

| メソッド | 説明 |
|----------|------|
| `clear()` | パスをクリア |
| `is_empty()` | パスが空かどうか |
| `len()` | コマンド数を返す |
| `cmds()` | コマンド列を PathCmd のリストとして返す |
| `points()` | 点列を Point のリストとして返す |
| `move_to(x, y)` | サブパス開始点を設定 |
| `line_to(x, y)` | 直線を追加 |
| `quad_to(cpx, cpy, x, y)` | 2 次ベジェ曲線を追加 |
| `cubic_to(cp1x, cp1y, cp2x, cp2y, x, y)` | 3 次ベジェ曲線を追加 |
| `smooth_quad_to(x, y)` | スムーズ 2 次ベジェを追加 |
| `smooth_cubic_to(cp2x, cp2y, x, y)` | スムーズ 3 次ベジェを追加 |
| `conic_to(cx, cy, ex, ey, w)` | 円錐曲線を追加 (重み w > 0) |
| `arc_to(cx, cy, rx, ry, start, sweep, force_move_to)` | 楕円弧を追加 |
| `close()` | パスを閉じる |
| `add_pie(cx, cy, rx, ry, start, sweep)` | 扇形を追加 |
| `add_triangle(x0, y0, x1, y1, x2, y2)` | 三角形を追加 |
| `add_polygon(points)` | ポリゴンを追加 (Point または (x, y) タプル) |
| `add_polyline(points)` | 折れ線を追加 |
| `add_round_rect(x, y, w, h, rx, ry)` | 角丸矩形を追加 |
| `add_ellipse(cx, cy, rx, ry)` | 楕円を追加 |
| `add_circle(cx, cy, r)` | 円を追加 |
| `translate(dx, dy)` | 全頂点を平行移動 |
| `transform(m)` | 全頂点に行列を適用 |
| `add_path(other)` | 別の Path を追加 |
| `add_path_translated(other, dx, dy)` | 別の Path を平行移動して追加 |
| `add_path_transformed(other, m)` | 別の Path に行列を適用して追加 |
| `control_box()` | 制御点ベースのバウンディングボックスを返す (空なら None) |
| `bounding_box()` | バウンディングボックスを返す (空なら None) |
| `conic_weights()` | 円錐曲線の重みリストを返す |

### Gradient

グラデーションを定義するクラス。

```python
grad = raden.Gradient.new_linear(x0, y0, x1, y1)
grad.add_stop(0.0, raden.Rgba32.rgb(255, 0, 0))  # 赤
grad.add_stop(1.0, raden.Rgba32.rgb(0, 0, 255))  # 青
```

| スタティックメソッド | 説明 |
|----------------------|------|
| `new_linear(x0, y0, x1, y1)` | 線形グラデーションを作成 |
| `new_radial(x0, y0, x1, y1, r0, r1)` | 放射状グラデーションを作成 |
| `new_conic(x0, y0, angle)` | 円錐グラデーションを作成 |

| メソッド | 説明 |
|----------|------|
| `add_stop(offset, color)` | カラーストップを追加 (offset: 0.0〜1.0) |
| `set_extend_mode(mode)` | 拡張モードを設定 |
| `stops()` | カラーストップのリストを返す |
| `values()` | グラデーション値を返す (GradientValues) |

| プロパティ | 説明 |
|------------|------|
| `stop_count` | カラーストップの数 |
| `extend_mode` | 拡張モード |

### GradientStop

グラデーションの色停止点。

```python
stop = raden.GradientStop(0.5, raden.Rgba32.rgb(255, 0, 0))
```

| プロパティ | 説明 |
|------------|------|
| `offset` | 相対位置 (0.0〜1.0) |
| `color` | 色 (Rgba32) |

### GradientValues

グラデーション値の種別。`as_linear()` / `as_radial()` / `as_conic()` で各バリアントを取得する。

| メソッド | 説明 |
|----------|------|
| `as_linear()` | LinearGradientValues として返す (種別不一致なら None) |
| `as_radial()` | RadialGradientValues として返す (種別不一致なら None) |
| `as_conic()` | ConicGradientValues として返す (種別不一致なら None) |

### LinearGradientValues

線形グラデーションの定義値。

| プロパティ | 説明 |
|------------|------|
| `x0`, `y0` | 開始点 |
| `x1`, `y1` | 終了点 |

### RadialGradientValues

放射状グラデーションの定義値。

| プロパティ | 説明 |
|------------|------|
| `x0`, `y0` | 中心 |
| `x1`, `y1` | 焦点 |
| `r0` | 中心半径 |
| `r1` | 焦点半径 |

### ConicGradientValues

円錐グラデーションの定義値。

| プロパティ | 説明 |
|------------|------|
| `x0`, `y0` | 中心 |
| `angle` | 開始角度 (ラジアン) |

### Pattern

画像パターン塗りつぶしを定義するクラス。

```python
pattern = raden.Pattern.from_image(img)
pattern = raden.Pattern.from_data(data, width, height)
```

| スタティックメソッド | 説明 |
|----------------------|------|
| `from_image(image)` | Image からパターンを作成 |
| `from_data(data, width, height, stride=None)` | バイト列からパターンを作成 |

| メソッド | 説明 |
|----------|------|
| `set_origin(tx, ty)` | 原点オフセットを設定 |
| `set_transform(m)` | 変換行列を設定 |
| `set_filter(filter)` | 補間モードを設定 |
| `set_extend_mode(mode)` | 拡張モードを設定 |

| プロパティ | 説明 |
|------------|------|
| `width` | パターンの幅 |
| `height` | パターンの高さ |
| `extend_mode` | 拡張モード |
| `filter` | 補間モード |
| `transform` | 変換行列 |

### FontData

フォントファイルのバイトデータ。

```python
font_data = raden.FontData.from_file(path)
font_data = raden.FontData.from_bytes(bytes)
```

| スタティックメソッド | 説明 |
|----------------------|------|
| `from_file(path)` | ファイルからフォントデータを読み込み |
| `from_bytes(bytes)` | バイト列からフォントデータを作成 |

| メソッド | 説明 |
|----------|------|
| `data()` | バイト列を取得 |

### FontFace

パース済みフォントフェイス。TrueType テーブル (head, hhea, hmtx, cmap, loca, glyf) を解析する。

```python
font_face = raden.FontFace.from_data(font_data, index=0)
```

| スタティックメソッド | 説明 |
|----------------------|------|
| `from_data(font_data, index=0)` | FontData からフォントフェイスを作成 |

| プロパティ | 説明 |
|------------|------|
| `units_per_em` | units per em |
| `ascent` | アセント |
| `descent` | ディセント |
| `line_gap` | 行間 |

### Font

サイズ指定済みフォント。

```python
font = raden.Font.from_face(font_face, size=48.0)
```

| スタティックメソッド | 説明 |
|----------------------|------|
| `from_face(face, size)` | FontFace とサイズからフォントを作成 |

| プロパティ | 説明 |
|------------|------|
| `size` | フォントサイズ |
| `scale` | スケール |
| `ascent` | アセント |
| `descent` | ディセント |

### 列挙型

#### CompOp (合成モード)

29 種類の合成モード。

##### Porter-Duff 合成

| 値 | 説明 |
|----|------|
| `SrcOver` | ソースを上に重ねる (デフォルト) |
| `SrcCopy` | ソースをそのままコピー |
| `SrcIn` | ソースをデスティネーションのアルファでマスク |
| `SrcOut` | ソースをデスティネーションのアルファの逆でマスク |
| `SrcAtop` | ソースをデスティネーションのアルファでマスクしつつ重ねる |
| `DstOver` | デスティネーションを上に重ねる |
| `DstCopy` | デスティネーションをそのまま |
| `DstIn` | デスティネーションをソースのアルファでマスク |
| `DstOut` | デスティネーションをソースのアルファの逆でマスク |
| `DstAtop` | デスティネーションをソースのアルファでマスクしつつ重ねる |
| `Xor` | 排他的論理和 |
| `Clear` | クリア |
| `Plus` | 加算 |

##### ブレンドモード

| 値 | 説明 |
|----|------|
| `Minus` | 飽和減算 |
| `Modulate` | チャネル乗算 |
| `Multiply` | 乗算 |
| `Screen` | スクリーン |
| `Overlay` | オーバーレイ |
| `Darken` | 比較 (暗) |
| `Lighten` | 比較 (明) |
| `ColorDodge` | 覆い焼き |
| `ColorBurn` | 焼き込み |
| `LinearBurn` | リニア焼き込み |
| `LinearLight` | リニアライト |
| `PinLight` | ピンライト |
| `HardLight` | ハードライト |
| `SoftLight` | ソフトライト |
| `Difference` | 差の絶対値 |
| `Exclusion` | 除外 |

#### FillRule (塗りつぶし規則)

| 値 | 説明 |
|----|------|
| `NonZero` | ワインディングナンバーが非ゼロなら内側 (デフォルト) |
| `EvenOdd` | ワインディングナンバーが奇数なら内側 |

#### StrokeCap (ストロークキャップ)

| 値 | 説明 |
|----|------|
| `Butt` | 端で切断 (デフォルト) |
| `Square` | 四角形で延長 |
| `Round` | 丸く延長 |

#### StrokeJoin (ストロークジョイン)

| 値 | 説明 |
|----|------|
| `MiterClip` | マイター結合 (クリップ) (デフォルト) |
| `MiterBevel` | マイター結合 (ベベル) |
| `MiterRound` | マイター結合 (丸) |
| `Bevel` | ベベル結合 |
| `Round` | 丸結合 |

#### PixelFormat (ピクセルフォーマット)

| 値 | 説明 |
|----|------|
| `Prgb32` | 32-bit premultiplied ARGB (デフォルト) |
| `Xrgb32` | 32-bit XRGB (アルファ未使用) |
| `A8` | 8-bit アルファ専用 |

#### ExtendMode (拡張モード)

| 値 | 説明 |
|----|------|
| `Pad` | 端の色で埋める (デフォルト) |
| `Repeat` | 繰り返す |
| `Reflect` | 反転して繰り返す |

#### PatternFilter (パターン補間モード)

| 値 | 説明 |
|----|------|
| `Nearest` | 最近傍 (デフォルト) |
| `Bilinear` | 双一次補間 |

#### PathCmd (パスコマンド種別)

| 値 | 説明 |
|----|------|
| `MoveTo` | サブパス開始点 |
| `LineTo` | 直線 |
| `QuadTo` | 2 次ベジェ曲線 |
| `ConicTo` | 円錐曲線 |
| `CubicTo` | 3 次ベジェ曲線 |
| `Close` | パスを閉じる |

## ビルド

ビルドには Rust 1.91 以降と uv が必要です。

```bash
uv build --wheel
```

開発環境をセットアップしてテストを実行するには:

```bash
uv sync --extra dev
uv run maturin develop
uv run pytest
```

## サンプル

```bash
uv sync --extra dev
uv run maturin develop

# 基本的な図形の描画
uv run python examples/basic.py

# アニメーション表示 (raw-player が必要)
uv run python examples/animation.py

# raden_player の移植例 (raw-player が必要、macOS のみ)
uv run python examples/raden_player.py
```

- `examples/basic.py`: 図形を描画して `basic.bmp` に保存する
- `examples/animation.py`: `raw_player.VideoPlayer` を使ったアニメーション表示例
- `examples/raden_player.py`: `raden/examples/raden_player.rs` を Python に移植したアニメーション表示例

> [!NOTE]
>
> - `animation.py` と `raden_player.py` は [raw-player](https://pypi.org/project/raw-player/) のインストールが必要です
> - `raden_player.py` は macOS のシステムフォントを使用するため macOS でのみ動作します
> - Prgb32 はリトルエンディアン ARGB、つまり BGRA バイト列として `data()` から取得できます

## raden ライセンス

Apache License 2.0

```text
Copyright 2026-2026, Shiguredo Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

## raden-py ライセンス

Apache License 2.0

```text
Copyright 2026-2026, Shiguredo Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
