"""raden Python バインディングの基本動作テスト。"""

import os
import tempfile

import pytest
import raden


def test_module_imports() -> None:
    """モジュール import と再エクスポートを確認する。"""
    assert raden.Rgba32 is not None
    assert raden.Matrix2D is not None
    assert raden.Path is not None
    assert raden.Gradient is not None
    assert raden.Context is not None
    assert raden.Image is not None
    assert raden.CompOp is not None
    assert raden.FillRule is not None
    assert raden.StrokeCap is not None
    assert raden.StrokeJoin is not None
    assert raden.PixelFormat is not None
    assert raden.ExtendMode is not None
    assert raden.PatternFilter is not None
    assert raden.GradientStop is not None
    assert raden.GradientValues is not None
    assert raden.LinearGradientValues is not None
    assert raden.RadialGradientValues is not None
    assert raden.ConicGradientValues is not None


def test_rgba32() -> None:
    """Rgba32 の作成とプロパティを確認する。"""
    c = raden.Rgba32(255, 128, 64)
    assert c.r == 255
    assert c.g == 128
    assert c.b == 64
    assert c.a == 255

    c2 = raden.Rgba32.rgb(10, 20, 30)
    assert c2.a == 255

    # Python 側での等値比較とハッシュ化ができることを確認する。
    assert c == raden.Rgba32(255, 128, 64)
    assert c != raden.Rgba32(255, 128, 65)
    assert hash(c) == hash(raden.Rgba32(255, 128, 64))


def test_enums() -> None:
    """各種 enum の比較を確認する。"""
    assert raden.CompOp.SrcOver == raden.CompOp.SrcOver
    assert raden.FillRule.NonZero != raden.FillRule.EvenOdd
    assert raden.StrokeCap.Round != raden.StrokeCap.Butt
    assert raden.PixelFormat.Prgb32 == raden.PixelFormat.Prgb32


def test_matrix2d() -> None:
    """Matrix2D の基本操作を確認する。"""
    m = raden.Matrix2D.translation(10, 20)
    x, y = m.map_point(0, 0)
    assert x == 10
    assert y == 20

    # 逆行列が計算できることを確認する。
    inv = m.invert()
    assert inv is not None
    x2, y2 = inv.map_point(x, y)
    assert abs(x2) < 1e-9
    assert abs(y2) < 1e-9


def test_path() -> None:
    """Path の構築を確認する。"""
    path = raden.Path()
    path.move_to(0, 0)
    path.line_to(100, 0)
    path.line_to(50, 100)
    path.close()
    assert not path.is_empty()
    assert path.len() > 0
    assert len(path.cmds()) == 4
    assert len(path.points()) == 3


def test_context_basic_drawing() -> None:
    """Context で基本的な描画を実行する。"""
    ctx = raden.Context(200, 200, raden.PixelFormat.Prgb32)
    ctx.set_fill_style(raden.Rgba32(255, 255, 255))
    ctx.fill_all()

    ctx.set_fill_style(raden.Rgba32(255, 0, 0))
    ctx.fill_rect(raden.Rect(10, 10, 50, 50))

    ctx.set_fill_style(raden.Rgba32(0, 255, 0))
    ctx.fill_circle(raden.Circle(100, 100, 30))

    path = raden.Path()
    path.move_to(150, 10)
    path.line_to(190, 10)
    path.line_to(170, 90)
    path.close()
    ctx.set_fill_style(raden.Rgba32(0, 0, 255))
    ctx.fill_path(path)

    ctx.clear_all()

    assert ctx.width() == 200
    assert ctx.height() == 200
    data = ctx.data()
    assert len(data) == 200 * 200 * 4


def test_image_save_bmp() -> None:
    """Image.save_bmp で一時ファイルに保存できることを確認する。"""
    img = raden.Image(100, 100, raden.PixelFormat.Prgb32)
    with tempfile.TemporaryDirectory() as tmpdir:
        path = os.path.join(tmpdir, "test.bmp")
        img.save_bmp(path)
        assert os.path.exists(path)
        assert os.path.getsize(path) > 0


def test_context_save_bmp() -> None:
    """Context.save_bmp で一時ファイルに保存できることを確認する。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)
    ctx.set_fill_style(raden.Rgba32(0, 128, 255))
    ctx.fill_all()
    with tempfile.TemporaryDirectory() as tmpdir:
        path = os.path.join(tmpdir, "ctx.bmp")
        ctx.save_bmp(path)
        assert os.path.exists(path)
        assert os.path.getsize(path) > 0


def test_font_loading() -> None:
    """システムフォントが存在する場合のみ読み込みを確認する。"""
    font_path = "/System/Library/Fonts/Helvetica.ttc"
    if not os.path.exists(font_path):
        return

    data = raden.FontData.from_file(font_path)
    face = raden.FontFace.from_data(data, 0)
    font = raden.Font.from_face(face, 24.0)

    ctx = raden.Context(200, 100, raden.PixelFormat.Prgb32)
    ctx.set_fill_style(raden.Rgba32(0, 0, 0))
    ctx.fill_text(10, 80, font, "Hello")


def test_polygon_tuple_points() -> None:
    """タプルで指定した点列でポリゴンを描画する。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)
    ctx.set_fill_style(raden.Rgba32(255, 0, 0))
    ctx.fill_polygon([(10, 10), (90, 10), (50, 90)])


def test_polygon_point_objects() -> None:
    """Point オブジェクトで指定した点列でポリゴンを描画する。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)
    ctx.set_fill_style(raden.Rgba32(0, 255, 0))
    ctx.fill_polygon([raden.Point(10, 10), raden.Point(90, 10), raden.Point(50, 90)])


def test_image_memoryview() -> None:
    """Image のバッファプロトコルで memoryview を取得する。"""
    w, h = 80, 60
    img = raden.Image(w, h, raden.PixelFormat.Prgb32)

    mv = memoryview(img)
    # Prgb32 は 3 次元 (H, W, 4)
    assert mv.ndim == 3
    assert mv.shape == (h, w, 4)
    assert mv.format == "B"
    assert mv.readonly is True
    # バッファ全体のバイト数は stride * height と一致する
    assert len(mv) == h
    assert mv.nbytes == img.stride() * h


def test_image_numpy_asarray() -> None:
    """np.asarray で Image のピクセルデータを numpy 配列として取得する。"""
    np = pytest.importorskip("numpy")

    w, h = 100, 50
    img = raden.Image(w, h, raden.PixelFormat.Prgb32)
    arr = np.asarray(img)
    # Prgb32 は (H, W, 4) uint8
    assert arr.shape == (h, w, 4)
    assert arr.dtype == np.uint8

    # data() の bytes と一致することを確認する
    data = img.data()
    arr_from_bytes = np.frombuffer(data, dtype=np.uint8).reshape((h, w, 4))
    assert np.array_equal(arr, arr_from_bytes)


def test_context_image_buffer_protocol() -> None:
    """Context.image() で取得した Image のバッファプロトコルを確認する。"""
    np = pytest.importorskip("numpy")

    w, h = 200, 100
    ctx = raden.Context(w, h, raden.PixelFormat.Prgb32)

    # 赤で塗りつぶす
    ctx.set_fill_style(raden.Rgba32.rgb(255, 0, 0))
    ctx.fill_all()

    img = ctx.image()
    arr = np.asarray(img)
    assert arr.shape == (h, w, 4)
    assert arr.dtype == np.uint8

    # Prgb32 はリトルエンディアン ARGB なのでバイト列は BGRA
    # 赤 (R=255) のピクセルは BGRA = (0, 0, 255, 255)
    assert arr[0, 0, 0] == 0    # B
    assert arr[0, 0, 1] == 0    # G
    assert arr[0, 0, 2] == 255  # R
    assert arr[0, 0, 3] == 255  # A

    # data() の bytes と一致することを確認する
    data = ctx.data()
    arr_from_bytes = np.frombuffer(data, dtype=np.uint8).reshape((h, w, 4))
    assert np.array_equal(arr, arr_from_bytes)


def test_image_a8_buffer_protocol() -> None:
    """A8 フォーマットの Image は 2 次元 (H, W) として扱う。"""
    w, h = 40, 30
    img = raden.Image(w, h, raden.PixelFormat.A8)

    mv = memoryview(img)
    # A8 は 1 チャネルなので 2 次元 (H, W)
    assert mv.ndim == 2
    assert mv.shape == (h, w)
    assert mv.format == "B"


# ---------------------------------------------------------------------------
# Gradient のゲッターと GradientValues
# ---------------------------------------------------------------------------


def test_gradient_stops_and_getters() -> None:
    """Gradient の stops / stop_count / extend_mode ゲッターを確認する。"""
    g = raden.Gradient.new_linear(0, 0, 100, 200)
    g.add_stop(0.0, raden.Rgba32.rgb(255, 0, 0))
    g.add_stop(1.0, raden.Rgba32.rgb(0, 0, 255))

    assert g.stop_count == 2
    assert g.extend_mode == raden.ExtendMode.Pad

    stops = g.stops()
    assert len(stops) == 2
    assert stops[0].offset == 0.0
    assert stops[0].color == raden.Rgba32.rgb(255, 0, 0)
    assert stops[1].offset == 1.0
    assert stops[1].color == raden.Rgba32.rgb(0, 0, 255)

    # 拡張モードを変更して確認
    g.set_extend_mode(raden.ExtendMode.Repeat)
    assert g.extend_mode == raden.ExtendMode.Repeat


def test_gradient_values_linear() -> None:
    """LinearGradientValues を確認する。"""
    g = raden.Gradient.new_linear(10, 20, 30, 40)
    vals = g.values()
    lin = vals.as_linear()
    assert lin is not None
    assert lin.x0 == 10
    assert lin.y0 == 20
    assert lin.x1 == 30
    assert lin.y1 == 40
    # 種別が一致しない場合は None
    assert vals.as_radial() is None
    assert vals.as_conic() is None


def test_gradient_values_radial() -> None:
    """RadialGradientValues を確認する。"""
    g = raden.Gradient.new_radial(10, 20, 30, 40, 0, 50)
    vals = g.values()
    rad = vals.as_radial()
    assert rad is not None
    assert rad.x0 == 10
    assert rad.y0 == 20
    assert rad.x1 == 30
    assert rad.y1 == 40
    assert rad.r0 == 0
    assert rad.r1 == 50
    assert vals.as_linear() is None
    assert vals.as_conic() is None


def test_gradient_values_conic() -> None:
    """ConicGradientValues を確認する。"""
    g = raden.Gradient.new_conic(10, 20, 1.5)
    vals = g.values()
    con = vals.as_conic()
    assert con is not None
    assert con.x0 == 10
    assert con.y0 == 20
    assert abs(con.angle - 1.5) < 1e-9
    assert vals.as_linear() is None
    assert vals.as_radial() is None


# ---------------------------------------------------------------------------
# Pattern のゲッター
# ---------------------------------------------------------------------------


def test_pattern_getters() -> None:
    """Pattern の extend_mode / filter / transform ゲッターを確認する。"""
    img = raden.Image(10, 10, raden.PixelFormat.Prgb32)
    p = raden.Pattern.from_image(img)

    # デフォルト値を確認 (Pattern は Repeat がデフォルト)
    assert p.extend_mode == raden.ExtendMode.Repeat
    assert p.filter == raden.PatternFilter.Nearest
    assert p.transform.is_identity()

    # 設定して確認
    p.set_extend_mode(raden.ExtendMode.Pad)
    assert p.extend_mode == raden.ExtendMode.Pad

    p.set_filter(raden.PatternFilter.Bilinear)
    assert p.filter == raden.PatternFilter.Bilinear

    m = raden.Matrix2D.translation(5, 10)
    p.set_transform(m)
    t = p.transform
    x, y = t.map_point(0, 0)
    assert x == 5
    assert y == 10


# ---------------------------------------------------------------------------
# Path の control_box / bounding_box / conic_weights
# ---------------------------------------------------------------------------


def test_path_control_box() -> None:
    """Path.control_box で制御点ベースのバウンディングボックスを取得する。"""
    path = raden.Path()
    path.move_to(10, 20)
    path.line_to(100, 50)
    path.line_to(50, 100)
    path.close()

    box = path.control_box()
    assert box is not None
    assert box.x == 10
    assert box.y == 20
    assert abs(box.w - 90) < 1e-9
    assert abs(box.h - 80) < 1e-9


def test_path_bounding_box_empty() -> None:
    """空の Path の bounding_box は None を返す。"""
    path = raden.Path()
    assert path.control_box() is None
    assert path.bounding_box() is None


def test_path_conic_weights() -> None:
    """conic_to を使わない Path の conic_weights は空リストを返す。"""
    path = raden.Path()
    path.move_to(0, 0)
    path.line_to(100, 100)
    assert path.conic_weights() == []


# ---------------------------------------------------------------------------
# Context のゲッター
# ---------------------------------------------------------------------------


def test_context_style_getters() -> None:
    """Context のスタイル関連ゲッターを確認する。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)

    ctx.set_fill_style(raden.Rgba32(10, 20, 30, 40))
    ctx.set_stroke_style(raden.Rgba32(50, 60, 70, 80))

    assert ctx.fill_color() == raden.Rgba32(10, 20, 30, 40)
    assert ctx.stroke_color() == raden.Rgba32(50, 60, 70, 80)


def test_context_comp_op_getter() -> None:
    """Context の comp_op / fill_rule ゲッターを確認する。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)

    ctx.set_comp_op(raden.CompOp.SrcCopy)
    assert ctx.comp_op() == raden.CompOp.SrcCopy

    ctx.set_fill_rule(raden.FillRule.EvenOdd)
    assert ctx.fill_rule() == raden.FillRule.EvenOdd


def test_context_alpha_getters() -> None:
    """Context の global_alpha / fill_alpha / stroke_alpha ゲッターを確認する。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)

    ctx.set_global_alpha(0.5)
    assert ctx.global_alpha() == 0.5

    ctx.set_fill_alpha(0.7)
    assert ctx.fill_alpha() == 0.7

    ctx.set_stroke_alpha(0.8)
    assert ctx.stroke_alpha() == 0.8


def test_context_stroke_getters() -> None:
    """Context のストローク関連ゲッターを確認する。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)

    ctx.set_stroke_width(3.5)
    assert ctx.stroke_width() == 3.5

    ctx.set_stroke_miter_limit(5.0)
    assert ctx.stroke_miter_limit() == 5.0

    ctx.set_stroke_join(raden.StrokeJoin.Round)
    assert ctx.stroke_join() == raden.StrokeJoin.Round

    ctx.set_stroke_start_cap(raden.StrokeCap.Round)
    assert ctx.stroke_start_cap() == raden.StrokeCap.Round

    ctx.set_stroke_end_cap(raden.StrokeCap.Square)
    assert ctx.stroke_end_cap() == raden.StrokeCap.Square

    ctx.set_stroke_dash_array([5.0, 3.0])
    assert ctx.stroke_dash_array() == [5.0, 3.0]

    ctx.set_stroke_dash_offset(2.0)
    assert ctx.stroke_dash_offset() == 2.0


def test_context_gradient_pattern_getters() -> None:
    """Context の fill_gradient / fill_pattern ゲッターを確認する。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)

    # 初期状態は None
    assert ctx.fill_gradient() is None
    assert ctx.fill_pattern() is None
    assert ctx.stroke_gradient() is None
    assert ctx.stroke_pattern() is None

    # グラデーションを設定して確認
    g = raden.Gradient.new_linear(0, 0, 100, 0)
    g.add_stop(0.0, raden.Rgba32.rgb(255, 0, 0))
    ctx.set_fill_style_gradient(g)
    fg = ctx.fill_gradient()
    assert fg is not None
    assert fg.stop_count == 1

    # 単色を設定し直すと None に戻る
    ctx.set_fill_style(raden.Rgba32.rgb(0, 0, 0))
    assert ctx.fill_gradient() is None


def test_context_user_to_meta() -> None:
    """user_to_meta でユーザ行列を単位行列にリセットする。"""
    ctx = raden.Context(100, 100, raden.PixelFormat.Prgb32)
    ctx.translate(10, 20)
    assert not ctx.matrix().is_identity()

    ctx.user_to_meta()
    assert ctx.matrix().is_identity()
