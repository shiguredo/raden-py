"""raden で図形を描画して BMP ファイルに保存する基本例。"""

import raden


def main() -> None:
    """800x600 のキャンバスに背景・円・矩形・三角形を描画する。"""
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
    ctx.fill_polygon([raden.Point(600, 400), raden.Point(700, 200), raden.Point(750, 450)])

    ctx.save_bmp("basic.bmp")
    print("saved basic.bmp")


if __name__ == "__main__":
    main()
