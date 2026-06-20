"""
raden の Python バインディング。
"""

from collections.abc import Sequence
from typing import Any, Final, final

@final
class Arc:
    """
    円弧を Python に露出する。
    """
    def __new__(cls, /, cx: float, cy: float, rx: float, ry: float, start: float, sweep: float) -> Arc: ...
    def __repr__(self, /) -> str: ...
    @property
    def cx(self, /) -> float: ...
    @property
    def cy(self, /) -> float: ...
    @property
    def rx(self, /) -> float: ...
    @property
    def ry(self, /) -> float: ...
    @property
    def start(self, /) -> float: ...
    @property
    def sweep(self, /) -> float: ...

@final
class Circle:
    """
    円を Python に露出する。
    """
    def __new__(cls, /, cx: float, cy: float, r: float) -> Circle: ...
    def __repr__(self, /) -> str: ...
    @property
    def cx(self, /) -> float: ...
    @property
    def cy(self, /) -> float: ...
    @property
    def r(self, /) -> float: ...

@final
class CompOp:
    """
    合成オペレーション。
    """
    Clear: Final[CompOp]
    ColorBurn: Final[CompOp]
    ColorDodge: Final[CompOp]
    Darken: Final[CompOp]
    Difference: Final[CompOp]
    DstAtop: Final[CompOp]
    DstCopy: Final[CompOp]
    DstIn: Final[CompOp]
    DstOut: Final[CompOp]
    DstOver: Final[CompOp]
    Exclusion: Final[CompOp]
    HardLight: Final[CompOp]
    Lighten: Final[CompOp]
    LinearBurn: Final[CompOp]
    LinearLight: Final[CompOp]
    Minus: Final[CompOp]
    Modulate: Final[CompOp]
    Multiply: Final[CompOp]
    Overlay: Final[CompOp]
    PinLight: Final[CompOp]
    Plus: Final[CompOp]
    Screen: Final[CompOp]
    SoftLight: Final[CompOp]
    SrcAtop: Final[CompOp]
    SrcCopy: Final[CompOp]
    SrcIn: Final[CompOp]
    SrcOut: Final[CompOp]
    SrcOver: Final[CompOp]
    Xor: Final[CompOp]
    def __eq__(self, /, other: object) -> bool: ...
    def __int__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __repr__(self, /) -> str: ...

@final
class Context:
    """
    描画コンテキストを Python に露出する。
    """
    def __new__(cls, /, width: int, height: int, format: PixelFormat = ...) -> Context:
        """
        指定サイズと形式の画像を backing store として Context を生成する。
        """
    def __repr__(self, /) -> str: ...
    def apply_matrix(self, /, m: Matrix2D) -> None:
        """
        任意の行列を現在の変換行列に後乗算で適用する。
        """
    def blit_image_at(self, /, x: float, y: float, src: Image) -> None:
        """
        ソース画像全体を (x, y) に転送する。
        """
    def blit_image_rect(self, /, dst: Rect, src: Image, src_rect: Rect |None = None) -> None:
        """
        ソース画像を dst 矩形に転送する。
        """
    def clear_all(self, /) -> None:
        """
        画像全体をクリアする。
        """
    def clear_rect(self, /, rect: Rect) -> None:
        """
        指定矩形をクリアする。
        """
    def clip_to_rect(self, /, rect: Rect) -> None:
        """
        クリップ領域を指定矩形との積集合に縮小する。
        """
    def comp_op(self, /) -> CompOp:
        """
        合成モードを返す。
        """
    def data(self, /) -> bytes:
        """
        ピクセルデータを bytes として返す (コピー)。
        """
    def fill_alpha(self, /) -> float:
        """
        fill アルファを返す。
        """
    def fill_all(self, /) -> None:
        """
        画像全体を塗りつぶす。
        """
    def fill_circle(self, /, circle: Circle) -> None:
        """
        円を塗りつぶす。
        """
    def fill_color(self, /) -> Rgba32:
        """
        塗りつぶし色を返す。
        """
    def fill_gradient(self, /) -> Gradient |None:
        """
        塗りつぶしグラデーションを返す。未設定の場合は None。
        """
    def fill_pattern(self, /) -> Pattern |None:
        """
        塗りつぶしパターンを返す。未設定の場合は None。
        """
    def fill_rule(self, /) -> FillRule:
        """
        塗りつぶし規則を返す。
        """
    def fill_ellipse(self, /, ellipse: Ellipse) -> None:
        """
        楕円を塗りつぶす。
        """
    def fill_path(self, /, path: Path) -> None:
        """
        パスを塗りつぶす。
        """
    def fill_pie(self, /, arc: Arc) -> None:
        """
        扇形を塗りつぶす。
        """
    def fill_polygon(self, /, points: Any) -> None:
        """
        ポリゴンを塗りつぶす。点列は Point または (x, y) タプル。
        """
    def fill_rect(self, /, rect: Rect) -> None:
        """
        矩形を塗りつぶす。
        """
    def fill_round_rect(self, /, rr: RoundRect) -> None:
        """
        角丸矩形を塗りつぶす。
        """
    def fill_text(self, /, x: float, y: float, font: Font, text: str) -> None:
        """
        テキストを塗りつぶし描画する。
        """
    def fill_triangle(self, /, t: Triangle) -> None:
        """
        三角形を塗りつぶす。
        """
    def global_alpha(self, /) -> float:
        """
        グローバルアルファを返す。
        """
    def height(self, /) -> int:
        """
        高さを返す。
        """
    def image(self, /) -> Image:
        """
        内部の Image を返す。
        """
    def matrix(self, /) -> Matrix2D:
        """
        現在の変換行列を取得する。
        """
    def post_rotate(self, /, angle: float) -> None:
        """
        回転を前乗算で適用する。角度はラジアン。
        """
    def post_scale(self, /, sx: float, sy: float) -> None:
        """
        スケーリングを前乗算で適用する。
        """
    def post_skew(self, /, kx: float, ky: float) -> None:
        """
        せん断を前乗算で適用する。
        """
    def post_transform(self, /, m: Matrix2D) -> None:
        """
        任意の行列を前乗算で適用する。
        """
    def post_translate(self, /, tx: float, ty: float) -> None:
        """
        平行移動を前乗算で適用する。
        """
    def reset_matrix(self, /) -> None:
        """
        変換行列を単位行列にリセットする。
        """
    def restore(self, /) -> None:
        """
        スタックから描画状態を復元する。スタックが空の場合は何もしない。
        """
    def restore_clipping(self, /) -> None:
        """
        クリップ領域を画像境界にリセットする。
        """
    def rotate(self, /, angle: float) -> None:
        """
        回転を現在の変換行列に後乗算で適用する。角度はラジアン。
        """
    def rotate_around(self, /, angle: float, cx: float, cy: float) -> None:
        """
        指定中心まわりの回転を後乗算で適用する。
        """
    def save(self, /) -> None:
        """
        現在の描画状態をスタックに保存する。
        """
    def save_bmp(self, /, path: str) -> None:
        """
        BMP ファイルとして保存する。
        """
    def scale(self, /, sx: float, sy: float) -> None:
        """
        スケーリングを現在の変換行列に後乗算で適用する。
        """
    def set_comp_op(self, /, op: CompOp) -> None:
        """
        合成オペレーションを設定する。
        """
    def set_fill_alpha(self, /, a: float) -> None:
        """
        fill アルファを設定する。
        """
    def set_fill_rule(self, /, rule: FillRule) -> None:
        """
        塗りつぶし規則を設定する。
        """
    def set_fill_style(self, /, color: Rgba32) -> None:
        """
        塗りつぶし色を設定する。
        """
    def set_fill_style_gradient(self, /, gradient: Gradient) -> None:
        """
        塗りつぶしスタイルをグラデーションに設定する。
        """
    def set_fill_style_pattern(self, /, pattern: Pattern) -> None:
        """
        塗りつぶしスタイルをパターンに設定する。
        """
    def set_global_alpha(self, /, a: float) -> None:
        """
        グローバルアルファを設定する。
        """
    def set_stroke_alpha(self, /, a: float) -> None:
        """
        stroke アルファを設定する。
        """
    def set_stroke_cap(self, /, cap: StrokeCap) -> None:
        """
        ストロークの端点形状を両端に一括設定する。
        """
    def set_stroke_dash_array(self, /, dash_array: Sequence[float]) -> None:
        """
        ストロークのダッシュパターンを設定する。
        """
    def set_stroke_dash_offset(self, /, offset: float) -> None:
        """
        ストロークのダッシュオフセットを設定する。
        """
    def set_stroke_end_cap(self, /, cap: StrokeCap) -> None:
        """
        ストロークの終点キャップを設定する。
        """
    def set_stroke_join(self, /, join: StrokeJoin) -> None:
        """
        ストロークの接続形状を設定する。
        """
    def set_stroke_miter_limit(self, /, limit: float) -> None:
        """
        ストロークのマイターリミットを設定する。
        """
    def set_stroke_start_cap(self, /, cap: StrokeCap) -> None:
        """
        ストロークの始点キャップを設定する。
        """
    def set_stroke_style(self, /, color: Rgba32) -> None:
        """
        ストローク色を設定する。
        """
    def set_stroke_style_gradient(self, /, gradient: Gradient) -> None:
        """
        ストロークスタイルをグラデーションに設定する。
        """
    def set_stroke_style_pattern(self, /, pattern: Pattern) -> None:
        """
        ストロークスタイルをパターンに設定する。
        """
    def set_stroke_width(self, /, width: float) -> None:
        """
        ストローク幅を設定する。
        """
    def skew(self, /, kx: float, ky: float) -> None:
        """
        せん断を現在の変換行列に後乗算で適用する。
        """
    def stroke_circle(self, /, circle: Circle) -> None:
        """
        円をストローク描画する。
        """
    def stroke_ellipse(self, /, ellipse: Ellipse) -> None:
        """
        楕円をストローク描画する。
        """
    def stroke_line(self, /, line: Line) -> None:
        """
        線分をストローク描画する。
        """
    def stroke_path(self, /, path: Path) -> None:
        """
        パスをストローク描画する。
        """
    def stroke_polygon(self, /, points: Any) -> None:
        """
        ポリゴンをストローク描画する。点列は Point または (x, y) タプル。
        """
    def stroke_polyline(self, /, points: Any) -> None:
        """
        折れ線をストローク描画する。点列は Point または (x, y) タプル。
        """
    def stroke_rect(self, /, rect: Rect) -> None:
        """
        矩形をストローク描画する。
        """
    def stroke_round_rect(self, /, rr: RoundRect) -> None:
        """
        角丸矩形をストローク描画する。
        """
    def stroke_triangle(self, /, t: Triangle) -> None:
        """
        三角形をストローク描画する。
        """
    def stroke_alpha(self, /) -> float:
        """
        stroke アルファを返す。
        """
    def stroke_color(self, /) -> Rgba32:
        """
        ストローク色を返す。
        """
    def stroke_dash_array(self, /) -> list[float]:
        """
        ストロークのダッシュパターンを返す。
        """
    def stroke_dash_offset(self, /) -> float:
        """
        ストロークのダッシュオフセットを返す。
        """
    def stroke_end_cap(self, /) -> StrokeCap:
        """
        ストロークの終点キャップを返す。
        """
    def stroke_gradient(self, /) -> Gradient |None:
        """
        ストロークグラデーションを返す。未設定の場合は None。
        """
    def stroke_join(self, /) -> StrokeJoin:
        """
        ストロークの接続形状を返す。
        """
    def stroke_miter_limit(self, /) -> float:
        """
        ストロークのマイターリミットを返す。
        """
    def stroke_pattern(self, /) -> Pattern |None:
        """
        ストロークパターンを返す。未設定の場合は None。
        """
    def stroke_start_cap(self, /) -> StrokeCap:
        """
        ストロークの始点キャップを返す。
        """
    def stroke_width(self, /) -> float:
        """
        ストローク幅を返す。
        """
    def translate(self, /, tx: float, ty: float) -> None:
        """
        平行移動を現在の変換行列に後乗算で適用する。
        """
    def user_to_meta(self, /) -> None:
        """
        ユーザ行列を単位行列にリセットする。
        """
    def width(self, /) -> int:
        """
        幅を返す。
        """

@final
class Ellipse:
    """
    楕円を Python に露出する。
    """
    def __new__(cls, /, cx: float, cy: float, rx: float, ry: float) -> Ellipse: ...
    def __repr__(self, /) -> str: ...
    @property
    def cx(self, /) -> float: ...
    @property
    def cy(self, /) -> float: ...
    @property
    def rx(self, /) -> float: ...
    @property
    def ry(self, /) -> float: ...

@final
class ExtendMode:
    """
    グラデーション/パターンの範囲外処理モード。
    """
    Pad: Final[ExtendMode]
    """
    端の色で埋める (デフォルト)。
    """
    Reflect: Final[ExtendMode]
    """
    反転して繰り返す。
    """
    Repeat: Final[ExtendMode]
    """
    繰り返す。
    """
    def __eq__(self, /, other: object) -> bool: ...
    def __int__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __repr__(self, /) -> str: ...

@final
class FillRule:
    """
    塗りつぶし規則。
    """
    EvenOdd: Final[FillRule]
    NonZero: Final[FillRule]
    def __eq__(self, /, other: object) -> bool: ...
    def __int__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __repr__(self, /) -> str: ...

@final
class Font:
    """
    サイズ指定済みフォント。
    """
    def __repr__(self, /) -> str: ...
    @property
    def ascent(self, /) -> float: ...
    @property
    def descent(self, /) -> float: ...
    @staticmethod
    def from_face(face: FontFace, size: float) -> Font:
        """
        FontFace とサイズからフォントを作成する。
        """
    @property
    def scale(self, /) -> float: ...
    @property
    def size(self, /) -> float: ...

@final
class FontData:
    """
    フォントファイルのバイトデータ。
    """
    def __repr__(self, /) -> str: ...
    def data(self, /) -> bytes:
        """
        バイト列を取得する。
        """
    @staticmethod
    def from_bytes(bytes: bytes) -> FontData:
        """
        バイト列からフォントデータを作成する。
        """
    @staticmethod
    def from_file(path: str) -> FontData:
        """
        ファイルからフォントデータを読み込む。
        """

@final
class FontFace:
    """
    パース済みフォントフェイス。
    """
    def __repr__(self, /) -> str: ...
    @property
    def ascent(self, /) -> int: ...
    @property
    def descent(self, /) -> int: ...
    @staticmethod
    def from_data(font_data: FontData, index: int = 0) -> FontFace:
        """
        FontData からフォントフェイスを作成する。
        """
    @property
    def line_gap(self, /) -> int: ...
    @property
    def units_per_em(self, /) -> int: ...

class FontError(Exception):
    """フォント関連のエラー。"""

@final
class Gradient:
    """
    グラデーションを Python に露出する。
    """
    def add_stop(self, /, offset: float, color: Rgba32) -> None:
        """
        色停止点を追加する。
        """
    @staticmethod
    def new_conic(x0: float, y0: float, angle: float) -> Gradient:
        """
        Conic Gradient を生成する。
        """
    @staticmethod
    def new_linear(x0: float, y0: float, x1: float, y1: float) -> Gradient:
        """
        Linear Gradient を生成する。
        """
    @staticmethod
    def new_radial(x0: float, y0: float, x1: float, y1: float, r0: float, r1: float) -> Gradient:
        """
        Radial Gradient を生成する。
        """
    def set_extend_mode(self, /, mode: ExtendMode) -> None:
        """
        拡張モードを設定する。
        """
    def stops(self, /) -> list[GradientStop]:
        """
        色停止点のリストを返す。
        """
    @property
    def stop_count(self, /) -> int:
        """
        色停止点の数を返す。
        """
    @property
    def extend_mode(self, /) -> ExtendMode:
        """
        拡張モードを返す。
        """
    def values(self, /) -> GradientValues:
        """
        グラデーション値を返す。
        """

@final
class GradientStop:
    """
    グラデーションの色停止点を Python に露出する。
    """
    def __new__(cls, /, offset: float, color: Rgba32) -> GradientStop: ...
    def __repr__(self, /) -> str: ...
    @property
    def color(self, /) -> Rgba32: ...
    @property
    def offset(self, /) -> float: ...

@final
class GradientValues:
    """
    グラデーション値の種別を Python に露出する。
    """
    def as_conic(self, /) -> ConicGradientValues |None: ...
    def as_linear(self, /) -> LinearGradientValues |None: ...
    def as_radial(self, /) -> RadialGradientValues |None: ...
    def __repr__(self, /) -> str: ...

@final
class LinearGradientValues:
    """
    線形グラデーションの定義値を Python に露出する。
    """
    def __repr__(self, /) -> str: ...
    @property
    def x0(self, /) -> float: ...
    @property
    def x1(self, /) -> float: ...
    @property
    def y0(self, /) -> float: ...
    @property
    def y1(self, /) -> float: ...

@final
class Image:
    """
    画像バッファを Python に露出する。
    """
    def __new__(cls, /, width: int, height: int, format: PixelFormat) -> Image:
        """
        指定サイズと形式の空画像を生成する。
        """
    def data(self, /) -> bytes:
        """
        ピクセルデータを bytes として返す (コピー)。
        """
    def format(self, /) -> PixelFormat:
        """
        ピクセル形式を返す。
        """
    def height(self, /) -> int:
        """
        高さを返す。
        """
    def save_bmp(self, /, path: str) -> None:
        """
        BMP ファイルとして保存する。
        """
    def stride(self, /) -> int:
        """
        ストライドを返す。
        """
    def width(self, /) -> int:
        """
        幅を返す。
        """
    def __getbuffer__(self, /, view: Any, flags: int) -> None: ...
    def __releasebuffer__(self, /, view: Any) -> None: ...

@final
class Line:
    """
    線分を Python に露出する。
    """
    def __new__(cls, /, x0: float, y0: float, x1: float, y1: float) -> Line: ...
    def __repr__(self, /) -> str: ...
    @property
    def x0(self, /) -> float: ...
    @property
    def x1(self, /) -> float: ...
    @property
    def y0(self, /) -> float: ...
    @property
    def y1(self, /) -> float: ...

@final
class Matrix2D:
    """
    2D アフィン変換行列を Python に露出する。
    """
    def __new__(cls, /, m00: float, m01: float, m10: float, m11: float, m20: float, m21: float) -> Matrix2D:
        """
        全要素を指定して行列を生成する。
        """
    def __repr__(self, /) -> str: ...
    def apply_matrix(self, /, m: Matrix2D) -> None:
        """
        任意の行列を後乗算で適用する。
        """
    @staticmethod
    def identity() -> Matrix2D:
        """
        単位行列を返す。
        """
    def invert(self, /) -> Matrix2D |None:
        """
        逆行列を計算する。行列式がゼロの場合は None を返す。
        """
    def is_identity(self, /) -> bool:
        """
        単位行列かどうかを判定する。
        """
    @property
    def m00(self, /) -> float:
        """
        各要素を取得する。
        """
    @property
    def m01(self, /) -> float: ...
    @property
    def m10(self, /) -> float: ...
    @property
    def m11(self, /) -> float: ...
    @property
    def m20(self, /) -> float: ...
    @property
    def m21(self, /) -> float: ...
    def map_point(self, /, x: float, y: float) -> tuple[float, float]:
        """
        点 (x, y) を変換する。
        """
    def multiply(self, /, other: Matrix2D) -> Matrix2D:
        """
        行列を合成する (self * other)。
        """
    def post_rotate(self, /, angle: float) -> None:
        """
        回転を前乗算で適用する。角度はラジアン。
        """
    def post_scale(self, /, sx: float, sy: float) -> None:
        """
        スケーリングを前乗算で適用する。
        """
    def post_skew(self, /, kx: float, ky: float) -> None:
        """
        せん断を前乗算で適用する。
        """
    def post_transform(self, /, m: Matrix2D) -> None:
        """
        任意の行列を前乗算で適用する。
        """
    def post_translate(self, /, tx: float, ty: float) -> None:
        """
        平行移動を前乗算で適用する。
        """
    def reset(self, /) -> None:
        """
        変換行列を単位行列にリセットする。
        """
    def rotate(self, /, angle: float) -> None:
        """
        回転を後乗算で適用する。角度はラジアン。
        """
    def rotate_around(self, /, angle: float, cx: float, cy: float) -> None:
        """
        指定中心まわりの回転を後乗算で適用する。角度はラジアン。
        """
    @staticmethod
    def rotation(angle: float) -> Matrix2D:
        """
        回転行列を返す。角度はラジアン。
        """
    def scale(self, /, sx: float, sy: float) -> None:
        """
        スケーリングを後乗算で適用する。
        """
    @staticmethod
    def scaling(sx: float, sy: float) -> Matrix2D:
        """
        スケーリング行列を返す。
        """
    def skew(self, /, kx: float, ky: float) -> None:
        """
        せん断を後乗算で適用する。
        """
    @staticmethod
    def skewing(kx: float, ky: float) -> Matrix2D:
        """
        せん断行列を返す。
        """
    def translate(self, /, tx: float, ty: float) -> None:
        """
        平行移動を後乗算で適用する。
        """
    @staticmethod
    def translation(tx: float, ty: float) -> Matrix2D:
        """
        平行移動行列を返す。
        """

@final
class Path:
    """
    2D パスを Python に露出する。
    """
    def __new__(cls, /) -> Path: ...
    def add_circle(self, /, cx: float, cy: float, r: float) -> None:
        """
        円を追加する。
        """
    def add_ellipse(self, /, cx: float, cy: float, rx: float, ry: float) -> None:
        """
        楕円を追加する。
        """
    def add_path(self, /, other: Path) -> None:
        """
        別の Path を追加する。
        """
    def add_path_transformed(self, /, other: Path, m: Matrix2D) -> None:
        """
        別の Path に行列を適用して追加する。
        """
    def add_path_translated(self, /, other: Path, dx: float, dy: float) -> None:
        """
        別の Path を平行移動して追加する。
        """
    def add_pie(self, /, cx: float, cy: float, rx: float, ry: float, start: float, sweep: float) -> None:
        """
        扇形 (pie) を追加する。
        """
    def add_polygon(self, /, points: Any) -> None:
        """
        点列を結んで閉じたポリゴンを追加する。
        """
    def add_polyline(self, /, points: Any) -> None:
        """
        点列を結んだ折れ線を追加する。
        """
    def add_round_rect(self, /, x: float, y: float, w: float, h: float, rx: float, ry: float) -> None:
        """
        角丸矩形を追加する。
        """
    def add_triangle(self, /, x0: float, y0: float, x1: float, y1: float, x2: float, y2: float) -> None:
        """
        三角形を追加する。
        """
    def arc_to(self, /, cx: float, cy: float, rx: float, ry: float, start: float, sweep: float, force_move_to: bool) -> None:
        """
        楕円弧を現在点から接続する。
        """
    def clear(self, /) -> None:
        """
        パスをクリアする。
        """
    def close(self, /) -> None:
        """
        パスを閉じる。
        """
    def cmds(self, /) -> list[PathCmd]:
        """
        コマンド列を PathCmd のリストとして返す。
        """
    def conic_to(self, /, cx: float, cy: float, ex: float, ey: float, w: float) -> None:
        """
        円錐曲線 (有理二次ベジェ) を追加する。重み w は正数。
        """
    def cubic_to(self, /, cp1x: float, cp1y: float, cp2x: float, cp2y: float, x: float, y: float) -> None:
        """
        3 次ベジェ曲線を追加する。
        """
    def is_empty(self, /) -> bool:
        """
        パスが空かどうかを返す。
        """
    def len(self, /) -> int:
        """
        コマンド数を返す。
        """
    def line_to(self, /, x: float, y: float) -> None:
        """
        直線を追加する。
        """
    def move_to(self, /, x: float, y: float) -> None:
        """
        現在点を移動する。
        """
    def points(self, /) -> list[Point]:
        """
        点列を Point のリストとして返す。
        """
    def quad_to(self, /, cpx: float, cpy: float, x: float, y: float) -> None:
        """
        2 次ベジェ曲線を追加する。
        """
    def smooth_cubic_to(self, /, cp2x: float, cp2y: float, x: float, y: float) -> None:
        """
        前の三次ベジェの第 2 制御点を反射したスムーズ三次ベジェを追加する。
        """
    def smooth_quad_to(self, /, x: float, y: float) -> None:
        """
        前の二次ベジェの制御点を反射したスムーズ二次ベジェを追加する。
        """
    def bounding_box(self, /) -> Rect |None:
        """
        バウンディングボックスを返す。パスが空の場合は None。
        """
    def conic_weights(self, /) -> list[float]:
        """
        円錐曲線の重みリストを返す。
        """
    def control_box(self, /) -> Rect |None:
        """
        制御点ベースのバウンディングボックスを返す。パスが空の場合は None。
        """
    def transform(self, /, m: Matrix2D) -> None:
        """
        すべての頂点に行列を適用する。
        """
    def translate(self, /, dx: float, dy: float) -> None:
        """
        すべての頂点を平行移動する。
        """

@final
class PathCmd:
    """
    パスコマンドの種別を Python に露出する。
    """
    Close: Final[PathCmd]
    ConicTo: Final[PathCmd]
    CubicTo: Final[PathCmd]
    LineTo: Final[PathCmd]
    MoveTo: Final[PathCmd]
    QuadTo: Final[PathCmd]
    def __eq__(self, /, other: object) -> bool: ...
    def __int__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __repr__(self, /) -> str: ...

@final
class ConicGradientValues:
    """
    円錐グラデーションの定義値を Python に露出する。
    """
    def __repr__(self, /) -> str: ...
    @property
    def angle(self, /) -> float: ...
    @property
    def x0(self, /) -> float: ...
    @property
    def y0(self, /) -> float: ...

@final
class Pattern:
    """
    画像パターンを Python に露出する。
    """
    @staticmethod
    def from_data(data: bytes, width: int, height: int, stride: int |None = None) -> Pattern:
        """
        生の BGRA/PRGB32 バイト列からパターンを生成する。
        """
    @staticmethod
    def from_image(image: Image) -> Pattern:
        """
        Image からパターンを生成する。
        """
    @property
    def height(self, /) -> int: ...
    def set_extend_mode(self, /, mode: ExtendMode) -> None:
        """
        拡張モードを設定する。
        """
    def set_filter(self, /, filter: PatternFilter) -> None:
        """
        補間モードを設定する。
        """
    def set_origin(self, /, tx: float, ty: float) -> None:
        """
        原点オフセットを設定する。
        """
    def set_transform(self, /, m: Matrix2D) -> None:
        """
        ユーザ空間からテクスチャ空間への変換行列を設定する。
        """
    @property
    def width(self, /) -> int: ...
    @property
    def extend_mode(self, /) -> ExtendMode:
        """
        拡張モードを返す。
        """
    @property
    def filter(self, /) -> PatternFilter:
        """
        補間モードを返す。
        """
    @property
    def transform(self, /) -> Matrix2D:
        """
        変換行列を返す。
        """

@final
class PatternFilter:
    """
    パターンのピクセル補間モード。
    """
    Bilinear: Final[PatternFilter]
    """
    双一次補間。
    """
    Nearest: Final[PatternFilter]
    """
    最近傍 (デフォルト)。
    """
    def __eq__(self, /, other: object) -> bool: ...
    def __int__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __repr__(self, /) -> str: ...

@final
class PixelFormat:
    """
    ピクセルフォーマット。
    """
    A8: Final[PixelFormat]
    """
    アルファのみ 8bit/ピクセル。
    """
    Prgb32: Final[PixelFormat]
    """
    32-bit premultiplied ARGB。u32 で 0xAARRGGBB。
    """
    Xrgb32: Final[PixelFormat]
    """
    32-bit XRGB。アルファは未使用。
    """
    def __eq__(self, /, other: object) -> bool: ...
    def __int__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __repr__(self, /) -> str: ...

@final
class Point:
    """
    2D の点を Python に露出する。
    """
    def __new__(cls, /, x: float, y: float) -> Point: ...
    def __repr__(self, /) -> str: ...
    @property
    def x(self, /) -> float: ...
    @x.setter
    def x(self, /, value: float) -> None: ...
    @property
    def y(self, /) -> float: ...
    @y.setter
    def y(self, /, value: float) -> None: ...

@final
class RadialGradientValues:
    """
    放射状グラデーションの定義値を Python に露出する。
    """
    def __repr__(self, /) -> str: ...
    @property
    def r0(self, /) -> float: ...
    @property
    def r1(self, /) -> float: ...
    @property
    def x0(self, /) -> float: ...
    @property
    def x1(self, /) -> float: ...
    @property
    def y0(self, /) -> float: ...
    @property
    def y1(self, /) -> float: ...

@final
class Rect:
    """
    浮動小数点の矩形を Python に露出する。
    """
    def __new__(cls, /, x: float, y: float, w: float, h: float) -> Rect: ...
    def __repr__(self, /) -> str: ...
    @property
    def h(self, /) -> float: ...
    @property
    def w(self, /) -> float: ...
    @property
    def x(self, /) -> float: ...
    @property
    def y(self, /) -> float: ...

@final
class Rgba32:
    """
    32bit RGBA カラー値を Python に露出する。
    """
    def __eq__(self, /, other: object) -> bool: ...
    def __hash__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __new__(cls, /, r: int, g: int, b: int, a: int = 255) -> Rgba32:
        """
        コンストラクタ。アルファ省略時は 255 とする。
        """
    def __repr__(self, /) -> str:
        """
        文字列表現を返す。
        """
    @property
    def a(self, /) -> int:
        """
        アルファ成分を取得する。
        """
    @property
    def b(self, /) -> int:
        """
        青成分を取得する。
        """
    @property
    def g(self, /) -> int:
        """
        緑成分を取得する。
        """
    @property
    def r(self, /) -> int:
        """
        赤成分を取得する。
        """
    @staticmethod
    def rgb(r: int, g: int, b: int) -> Rgba32:
        """
        RGB 値からアルファ 255 の色を生成するファクトリ。
        """

@final
class RoundRect:
    """
    角丸矩形を Python に露出する。
    """
    def __new__(cls, /, x: float, y: float, w: float, h: float, rx: float, ry: float) -> RoundRect: ...
    def __repr__(self, /) -> str: ...
    @property
    def h(self, /) -> float: ...
    @property
    def rx(self, /) -> float: ...
    @property
    def ry(self, /) -> float: ...
    @property
    def w(self, /) -> float: ...
    @property
    def x(self, /) -> float: ...
    @property
    def y(self, /) -> float: ...

@final
class StrokeCap:
    """
    ストロークの端点形状。
    """
    Butt: Final[StrokeCap]
    Round: Final[StrokeCap]
    Square: Final[StrokeCap]
    def __eq__(self, /, other: object) -> bool: ...
    def __int__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __repr__(self, /) -> str: ...

@final
class StrokeJoin:
    """
    ストロークの接続形状。
    """
    Bevel: Final[StrokeJoin]
    MiterBevel: Final[StrokeJoin]
    MiterClip: Final[StrokeJoin]
    MiterRound: Final[StrokeJoin]
    Round: Final[StrokeJoin]
    def __eq__(self, /, other: object) -> bool: ...
    def __int__(self, /) -> int: ...
    def __ne__(self, /, other: object) -> bool: ...
    def __repr__(self, /) -> str: ...

@final
class Triangle:
    """
    三角形を Python に露出する。
    """
    def __new__(cls, /, x0: float, y0: float, x1: float, y1: float, x2: float, y2: float) -> Triangle: ...
    def __repr__(self, /) -> str: ...
    @property
    def x0(self, /) -> float: ...
    @property
    def x1(self, /) -> float: ...
    @property
    def x2(self, /) -> float: ...
    @property
    def y0(self, /) -> float: ...
    @property
    def y1(self, /) -> float: ...
    @property
    def y2(self, /) -> float: ...

