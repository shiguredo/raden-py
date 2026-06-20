"""raw_player.VideoPlayer を使ったアニメーション例。

このスクリプトは raden-py ライブラリ本体ではなく、examples 専用のコード。
raw-player は dev 依存として提供されている。
"""

from __future__ import annotations

import math
import time

import numpy as np
from raw_player import VideoPlayer

import raden

WIDTH = 640
HEIGHT = 480
FPS = 60


def draw(ctx: raden.Context, t: float) -> None:
    """フレームごとに Context に描画する。"""
    ctx.set_fill_style(raden.Rgba32(255, 255, 255))
    ctx.fill_all()

    cx = ctx.width() / 2.0
    cy = ctx.height() / 2.0
    radius = 100.0 + 50.0 * math.sin(t * 2.0)

    ctx.set_fill_style(raden.Rgba32(0, 128, 255))
    ctx.fill_circle(raden.Circle(cx, cy, radius))

    ctx.set_fill_style(raden.Rgba32(255, 200, 0))
    ctx.fill_rect(raden.Rect(cx - 40, cy - 40, 80, 80))


def main() -> None:
    """VideoPlayer を使ってアニメーションを表示する。"""
    player = VideoPlayer(WIDTH, HEIGHT, title="Raden Animation")
    player.play()

    ctx = raden.Context(WIDTH, HEIGHT, raden.PixelFormat.Prgb32)
    start_time = time.perf_counter()
    frame_interval = 1.0 / FPS

    try:
        while player.is_open:
            now = time.perf_counter()
            t = now - start_time

            draw(ctx, t)

            # raden の画像データはリトルエンディアン ARGB、つまり BGRA バイト列。
            frame = (
                np.frombuffer(ctx.data(), dtype=np.uint8)
                .reshape((ctx.height(), ctx.width(), 4))
                .copy()
            )
            player.enqueue_video_bgra(frame, int(now * 1_000_000))

            if not player.poll_events():
                break

            elapsed = time.perf_counter() - now
            sleep = frame_interval - elapsed
            if sleep > 0:
                time.sleep(sleep)
    finally:
        player.close()


if __name__ == "__main__":
    main()
