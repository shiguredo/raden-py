"""raden_player.rs と同等のアニメーションを Python で再現する例。

raden の examples/raden_player.rs と同じ図形・テキスト・アニメーションを描画し、
raw-player の VideoPlayer で表示する。
"""

import math
import time

import numpy as np
from raw_player import VideoPlayer

import raden

WIDTH = 1920
HEIGHT = 1080
TARGET_FPS = 60
DURATION = 10.0
NUM_WAVES = 5
NUM_BALLS = 8
NUM_SHAPES = 6

FONT_PATH = "/System/Library/Fonts/Helvetica.ttc"


def hsv_to_rgb(h, s, v):
    """HSV を RGB に変換する (h: 0-360, s: 0-1, v: 0-1)。"""
    c = v * s
    x = c * (1.0 - abs((h / 60.0) % 2.0 - 1.0))
    m = v - c

    if h < 60.0:
        r, g, b = c, x, 0.0
    elif h < 120.0:
        r, g, b = x, c, 0.0
    elif h < 180.0:
        r, g, b = 0.0, c, x
    elif h < 240.0:
        r, g, b = 0.0, x, c
    elif h < 300.0:
        r, g, b = x, 0.0, c
    else:
        r, g, b = c, 0.0, x

    return (
        int((r + m) * 255.0),
        int((g + m) * 255.0),
        int((b + m) * 255.0),
    )


def generate_frame(ctx, font_large, font_small, frame_number, actual_fps):
    """raden でアニメーションフレームを生成する。"""
    w = ctx.width()
    h = ctx.height()
    t = frame_number / TARGET_FPS

    # 背景: HSV で色相がゆっくり変化する暗い背景 (SrcCopy)
    ctx.set_comp_op(raden.CompOp.SrcCopy)
    bg_hue = (t * 20.0) % 360.0
    br, bg, bb = hsv_to_rgb(bg_hue, 0.3, 0.15)
    ctx.set_fill_style(raden.Rgba32.rgb(br, bg, bb))
    ctx.fill_rect(raden.Rect(0.0, 0.0, float(w), float(h)))

    ctx.set_comp_op(raden.CompOp.SrcOver)

    # ウェーブパターン
    for wave_idx in range(NUM_WAVES):
        wi = float(wave_idx)
        wave_offset = wi * 0.5
        wave_amplitude = 50.0 + wi * 20.0
        wave_freq = 0.008 + wi * 0.002
        wave_speed = 2.0 + wi * 0.3
        wave_y_base = h * 0.3 + wi * 80.0

        wave_hue = (t * 60.0 + wi * 50.0) % 360.0
        wr, wg, wb = hsv_to_rgb(wave_hue, 0.8, 0.9)
        ctx.set_fill_style(raden.Rgba32(wr, wg, wb, 150))

        x = 0.0
        while x < w:
            y = wave_y_base + wave_amplitude * math.sin(
                wave_freq * x + t * wave_speed + wave_offset
            )
            radius = 8.0 + 4.0 * math.sin(t * 3.0 + x * 0.01)
            ctx.fill_circle(raden.Circle(x, y, radius))
            x += 20.0

    # バウンドする円
    for ball_idx in range(NUM_BALLS):
        bi = float(ball_idx)
        freq_x = 0.5 + bi * 0.15
        freq_y = 0.7 + bi * 0.12
        phase_x = bi * math.pi / 4.0
        phase_y = bi * math.pi / 3.0

        bx = w * 0.5 + (w * 0.35) * math.sin(t * freq_x + phase_x)
        by = h * 0.5 + (h * 0.3) * math.sin(t * freq_y + phase_y)
        ball_radius = 30.0 + 15.0 * math.sin(t * 4.0 + bi)

        ball_hue = (bi * 45.0 + t * 100.0) % 360.0
        cr, cg, cb = hsv_to_rgb(ball_hue, 1.0, 1.0)
        ctx.set_fill_style(raden.Rgba32(cr, cg, cb, 200))
        ctx.fill_circle(raden.Circle(bx, by, ball_radius))

        # 光沢効果
        ctx.save()
        hl_radius = ball_radius * 0.3
        ctx.set_fill_style(raden.Rgba32(255, 255, 255, 100))
        ctx.fill_circle(
            raden.Circle(
                bx - ball_radius * 0.3,
                by - ball_radius * 0.3,
                hl_radius,
            )
        )
        ctx.restore()

    # 回転する円群
    center_x = w * 0.5
    center_y = h * 0.5
    for shape_idx in range(NUM_SHAPES):
        si = float(shape_idx)
        angle = t * (1.0 + si * 0.2) + si * math.pi / 3.0
        dist = 150.0 + 50.0 * math.sin(t * 2.0 + si)
        sx = center_x + dist * math.cos(angle)
        sy = center_y + dist * math.sin(angle)

        shape_hue = (si * 60.0 + t * 80.0) % 360.0
        sr, sg, sb = hsv_to_rgb(shape_hue, 0.9, 0.95)

        shape_radius = 25.0 + 15.0 * math.sin(t * 3.0 + si)
        ctx.set_fill_style(raden.Rgba32(sr, sg, sb, 180))
        ctx.fill_circle(raden.Circle(sx, sy, shape_radius))

    # 経過時間をミリ秒で大きく表示 (影付き)
    elapsed_ms = int(t * 1000.0)
    time_text = f"{elapsed_ms:08d} ms"
    text_x = w * 0.5 - 200.0
    text_y = h * 0.85

    ctx.set_fill_style(raden.Rgba32(0, 0, 0, 200))
    ctx.fill_text(text_x + 3.0, text_y + 3.0, font_large, time_text)
    ctx.set_fill_style(raden.Rgba32(255, 255, 255, 255))
    ctx.fill_text(text_x, text_y, font_large, time_text)

    # 情報表示
    ctx.set_fill_style(raden.Rgba32(255, 255, 255, 200))
    info_text = f"{w}x{h} | {TARGET_FPS} FPS | RAW BGRA"
    ctx.fill_text(30.0, 50.0, font_small, info_text)

    ctx.set_fill_style(raden.Rgba32(200, 200, 200, 255))
    frame_text = f"Frame: {frame_number:06d}"
    ctx.fill_text(30.0, 95.0, font_small, frame_text)

    ctx.set_fill_style(raden.Rgba32(150, 255, 150, 255))
    fps_text = f"FPS: {actual_fps:.1f}"
    ctx.fill_text(30.0, 140.0, font_small, fps_text)


def main():
    """raden_player.rs と同等のアニメーションを表示する。"""
    if not __import__("os").path.exists(FONT_PATH):
        print(f"Font not found: {FONT_PATH}")
        print("This example requires macOS system font")
        return

    player = VideoPlayer(WIDTH, HEIGHT, "Raden Animation")
    print(f"Renderer: {player.renderer_name}")

    player.set_key_callback(lambda keycode: keycode != 27 and keycode != ord("q"))

    player.play()
    print(f"{WIDTH}x{HEIGHT} | {TARGET_FPS} FPS | RAW BGRA")
    print("ESC or q key to exit...")

    font_data = raden.FontData.from_file(FONT_PATH)
    font_face = raden.FontFace.from_data(font_data, 0)
    font_large = raden.Font.from_face(font_face, 72.0)
    font_small = raden.Font.from_face(font_face, 32.0)

    ctx = raden.Context(WIDTH, HEIGHT, raden.PixelFormat.Prgb32)

    start_time = time.perf_counter()
    frame_interval = 1.0 / TARGET_FPS
    frame_number = 0

    try:
        while player.is_open:
            now = time.perf_counter()
            elapsed = now - start_time
            if elapsed >= DURATION:
                break

            if now < start_time + frame_number * frame_interval:
                time.sleep(start_time + frame_number * frame_interval - now)

            total_elapsed = time.perf_counter() - start_time
            current_fps = frame_number / total_elapsed if total_elapsed > 0.0 else 0.0

            generate_frame(ctx, font_large, font_small, frame_number, current_fps)

            pts_us = int((time.perf_counter() - start_time) * 1_000_000)
            frame = (
                np.frombuffer(ctx.data(), dtype=np.uint8)
                .reshape((ctx.height(), ctx.width(), 4))
                .copy()
            )
            player.enqueue_video_bgra(frame, pts_us)

            if not player.poll_events():
                break

            frame_number += 1
    finally:
        player.close()

    total_time = time.perf_counter() - start_time
    if total_time > 0.0:
        stats = player.stats()
        print()
        print(f"Generated frames: {frame_number}")
        print(f"Average FPS: {frame_number / total_time:.2f}")
        print(f"Dropped frames: {stats['dropped_frames']}")
        print(f"Repeated frames: {stats['repeated_frames']}")


if __name__ == "__main__":
    main()
