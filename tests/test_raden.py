"""raden Python バインディングの基本動作テスト。"""

import os
import tempfile

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
