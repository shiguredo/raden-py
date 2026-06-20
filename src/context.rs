// raden の描画コンテキストを PyO3 経由で Python に露出する。
//
// raden::Context は Image と PipelineRuntime への可変参照を持つため、
// Python 側で所有するにはライフタイム問題が生じる。
// そこで Python Context は Image と PipelineRuntime を直接所有し、
// 描画メソッド呼び出しのたびに一時的な raden::Context を作成して状態をコピーする。
// 描画状態 (fill/stroke/行列/クリップなど) は Python Context が保持し、
// save/restore も Python 側でスタック管理する。

use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::enums::PixelFormat;

/// Python Context が保持する描画状態のスナップショット。
#[derive(Clone)]
struct ContextState {
    comp_op: raden::CompOp,
    fill_rule: raden::FillRule,
    fill_color: raden::Rgba32,
    fill_gradient: Option<raden::Gradient>,
    fill_pattern: Option<raden::Pattern>,
    stroke_color: raden::Rgba32,
    stroke_gradient: Option<raden::Gradient>,
    stroke_pattern: Option<raden::Pattern>,
    stroke_width: f64,
    stroke_start_cap: raden::StrokeCap,
    stroke_end_cap: raden::StrokeCap,
    stroke_join: raden::StrokeJoin,
    stroke_miter_limit: f64,
    stroke_dash_array: Vec<f64>,
    stroke_dash_offset: f64,
    matrix: raden::Matrix2D,
    global_alpha: f64,
    fill_alpha: f64,
    stroke_alpha: f64,
    /// None の場合は画像境界 (メタクリップ) を表す。
    clip_rect: Option<raden::Rect>,
    /// メタクリップの幅 (画像幅)。
    image_width: u32,
    /// メタクリップの高さ (画像高さ)。
    image_height: u32,
}

impl ContextState {
    fn new(width: u32, height: u32) -> Self {
        Self {
            comp_op: raden::CompOp::SrcOver,
            fill_rule: raden::FillRule::default(),
            fill_color: raden::Rgba32::new(0, 0, 0, 255),
            fill_gradient: None,
            fill_pattern: None,
            stroke_color: raden::Rgba32::new(0, 0, 0, 255),
            stroke_gradient: None,
            stroke_pattern: None,
            stroke_width: 1.0,
            stroke_start_cap: raden::StrokeCap::default(),
            stroke_end_cap: raden::StrokeCap::default(),
            stroke_join: raden::StrokeJoin::default(),
            stroke_miter_limit: 4.0,
            stroke_dash_array: Vec::new(),
            stroke_dash_offset: 0.0,
            matrix: raden::Matrix2D::IDENTITY,
            global_alpha: 1.0,
            fill_alpha: 1.0,
            stroke_alpha: 1.0,
            clip_rect: None,
            image_width: width,
            image_height: height,
        }
    }

    /// 一時的な raden::Context に現在の状態を反映する。
    fn apply_to(&self, ctx: &mut raden::Context) {
        ctx.set_comp_op(self.comp_op);
        ctx.set_fill_rule(self.fill_rule);
        ctx.set_fill_style(self.fill_color);
        if let Some(ref g) = self.fill_gradient {
            ctx.set_fill_style_gradient(g);
        }
        if let Some(ref p) = self.fill_pattern {
            ctx.set_fill_style_pattern(p);
        }
        ctx.set_stroke_style(self.stroke_color);
        if let Some(ref g) = self.stroke_gradient {
            ctx.set_stroke_style_gradient(g);
        }
        if let Some(ref p) = self.stroke_pattern {
            ctx.set_stroke_style_pattern(p);
        }
        ctx.set_stroke_width(self.stroke_width);
        ctx.set_stroke_start_cap(self.stroke_start_cap);
        ctx.set_stroke_end_cap(self.stroke_end_cap);
        ctx.set_stroke_join(self.stroke_join);
        ctx.set_stroke_miter_limit(self.stroke_miter_limit);
        ctx.set_stroke_dash_array(&self.stroke_dash_array);
        ctx.set_stroke_dash_offset(self.stroke_dash_offset);
        ctx.reset_matrix();
        ctx.apply_matrix(&self.matrix);
        ctx.set_global_alpha(self.global_alpha);
        ctx.set_fill_alpha(self.fill_alpha);
        ctx.set_stroke_alpha(self.stroke_alpha);
        ctx.restore_clipping();
        if let Some(rect) = self.clip_rect {
            ctx.clip_to_rect(&rect);
        }
    }
}

/// 描画コンテキストを Python に露出する。
#[pyclass(unsendable, name = "Context")]
pub struct Context {
    /// 所有する画像。
    image: Py<crate::image::Image>,
    /// パイプラインランタイム。
    runtime: raden::PipelineRuntime,
    /// 現在の描画状態。
    state: ContextState,
    /// save/restore 用の状態スタック。
    state_stack: Vec<ContextState>,
}

impl Context {
    /// 一時的な raden::Context を作成し、描画処理を実行するヘルパー。
    fn with_ctx<F, R>(&mut self, py: Python, f: F) -> R
    where
        F: FnOnce(&mut raden::Context) -> R,
    {
        let mut img_ref = self.image.bind(py).borrow_mut();
        let img = &mut *img_ref;
        let mut ctx = raden::Context::new(img.get_inner_mut(), &mut self.runtime);
        self.state.apply_to(&mut ctx);
        f(&mut ctx)
    }
}

#[pymethods]
impl Context {
    /// 指定サイズと形式の画像を backing store として Context を生成する。
    #[new]
    #[pyo3(signature = (width, height, format = crate::enums::PixelFormat::Prgb32))]
    fn new(py: Python, width: u32, height: u32, format: PixelFormat) -> PyResult<Self> {
        let image = crate::image::Image::from_inner(raden::Image::new(
            width,
            height,
            format.into(),
        ));
        let py_image = Py::new(py, image)?;
        Ok(Self {
            image: py_image,
            runtime: raden::PipelineRuntime::new(),
            state: ContextState::new(width, height),
            state_stack: Vec::new(),
        })
    }

    // -------------------------------------------------------------------------
    // 画像アクセサ
    // -------------------------------------------------------------------------

    /// 幅を返す。
    fn width(&self, py: Python) -> u32 {
        self.image.bind(py).borrow().width()
    }

    /// 高さを返す。
    fn height(&self, py: Python) -> u32 {
        self.image.bind(py).borrow().height()
    }

    /// ピクセルデータを bytes として返す (コピー)。
    fn data<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        self.image.bind(py).borrow().data(py)
    }

    /// BMP ファイルとして保存する。
    fn save_bmp(&self, py: Python, path: &str) -> PyResult<()> {
        self.image.bind(py).borrow().save_bmp(path)
    }

    // -------------------------------------------------------------------------
    // 状態保存・復元
    // -------------------------------------------------------------------------

    /// 現在の描画状態をスタックに保存する。
    fn save(&mut self) {
        self.state_stack.push(self.state.clone());
    }

    /// スタックから描画状態を復元する。スタックが空の場合は何もしない。
    fn restore(&mut self) {
        if let Some(state) = self.state_stack.pop() {
            self.state = state;
        }
    }

    // -------------------------------------------------------------------------
    // クリップ
    // -------------------------------------------------------------------------

    /// クリップ領域を指定矩形との積集合に縮小する。
    fn clip_to_rect(&mut self, rect: &crate::geometry::Rect) {
        let rect_inner = rect.get_inner();
        let x0 = rect_inner.x.floor() as i32;
        let y0 = rect_inner.y.floor() as i32;
        let x1 = (rect_inner.x + rect_inner.w).ceil() as i32;
        let y1 = (rect_inner.y + rect_inner.h).ceil() as i32;

        let (cx0, cy0, cx1, cy1) = match self.state.clip_rect {
            Some(r) => (
                r.x as i32,
                r.y as i32,
                (r.x + r.w) as i32,
                (r.y + r.h) as i32,
            ),
            None => (0, 0, self.state.image_width as i32, self.state.image_height as i32),
        };

        let nx0 = cx0.max(x0);
        let ny0 = cy0.max(y0);
        let nx1 = cx1.min(x1);
        let ny1 = cy1.min(y1);

        if nx0 < nx1 && ny0 < ny1 {
            self.state.clip_rect = Some(raden::Rect::new(
                nx0 as f64,
                ny0 as f64,
                (nx1 - nx0) as f64,
                (ny1 - ny0) as f64,
            ));
        } else {
            self.state.clip_rect = Some(raden::Rect::new(0.0, 0.0, 0.0, 0.0));
        }
    }

    /// クリップ領域を画像境界にリセットする。
    fn restore_clipping(&mut self) {
        self.state.clip_rect = None;
    }

    // -------------------------------------------------------------------------
    // スタイル設定
    // -------------------------------------------------------------------------

    /// 塗りつぶし色を設定する。
    fn set_fill_style(&mut self, color: &crate::style::Rgba32) {
        self.state.fill_color = color.inner();
        self.state.fill_gradient = None;
        self.state.fill_pattern = None;
    }

    /// 塗りつぶしスタイルをグラデーションに設定する。
    fn set_fill_style_gradient(&mut self, gradient: &crate::gradient::Gradient) {
        self.state.fill_gradient = Some(gradient.get_inner().clone());
        self.state.fill_pattern = None;
    }

    /// 塗りつぶしスタイルをパターンに設定する。
    fn set_fill_style_pattern(&mut self, pattern: &crate::pattern::Pattern) {
        self.state.fill_pattern = Some(pattern.get_inner().clone());
        self.state.fill_gradient = None;
    }

    /// ストローク色を設定する。
    fn set_stroke_style(&mut self, color: &crate::style::Rgba32) {
        self.state.stroke_color = color.inner();
        self.state.stroke_gradient = None;
        self.state.stroke_pattern = None;
    }

    /// ストロークスタイルをグラデーションに設定する。
    fn set_stroke_style_gradient(&mut self, gradient: &crate::gradient::Gradient) {
        self.state.stroke_gradient = Some(gradient.get_inner().clone());
        self.state.stroke_pattern = None;
    }

    /// ストロークスタイルをパターンに設定する。
    fn set_stroke_style_pattern(&mut self, pattern: &crate::pattern::Pattern) {
        self.state.stroke_pattern = Some(pattern.get_inner().clone());
        self.state.stroke_gradient = None;
    }

    /// 合成オペレーションを設定する。
    fn set_comp_op(&mut self, op: crate::style::CompOp) {
        self.state.comp_op = op.into();
    }

    /// 塗りつぶし規則を設定する。
    fn set_fill_rule(&mut self, rule: crate::style::FillRule) {
        self.state.fill_rule = rule.into();
    }

    /// グローバルアルファを設定する。
    fn set_global_alpha(&mut self, a: f64) {
        self.state.global_alpha = a.clamp(0.0, 1.0);
    }

    /// fill アルファを設定する。
    fn set_fill_alpha(&mut self, a: f64) {
        self.state.fill_alpha = a.clamp(0.0, 1.0);
    }

    /// stroke アルファを設定する。
    fn set_stroke_alpha(&mut self, a: f64) {
        self.state.stroke_alpha = a.clamp(0.0, 1.0);
    }

    // -------------------------------------------------------------------------
    // 変換
    // -------------------------------------------------------------------------

    /// 平行移動を現在の変換行列に後乗算で適用する。
    fn translate(&mut self, tx: f64, ty: f64) {
        self.state.matrix.translate(tx, ty);
    }

    /// スケーリングを現在の変換行列に後乗算で適用する。
    fn scale(&mut self, sx: f64, sy: f64) {
        self.state.matrix.scale(sx, sy);
    }

    /// 回転を現在の変換行列に後乗算で適用する。角度はラジアン。
    fn rotate(&mut self, angle: f64) {
        self.state.matrix.rotate(angle);
    }

    /// せん断を現在の変換行列に後乗算で適用する。
    fn skew(&mut self, kx: f64, ky: f64) {
        self.state.matrix.skew(kx, ky);
    }

    /// 任意の行列を現在の変換行列に後乗算で適用する。
    fn apply_matrix(&mut self, m: &crate::matrix::Matrix2D) {
        self.state.matrix = self.state.matrix.multiply(&m.get_inner());
    }

    /// 変換行列を単位行列にリセットする。
    fn reset_matrix(&mut self) {
        self.state.matrix.reset();
    }

    /// 指定中心まわりの回転を後乗算で適用する。
    fn rotate_around(&mut self, angle: f64, cx: f64, cy: f64) {
        self.state.matrix.rotate_around(angle, cx, cy);
    }

    /// 平行移動を前乗算で適用する。
    fn post_translate(&mut self, tx: f64, ty: f64) {
        self.state.matrix.post_translate(tx, ty);
    }

    /// スケーリングを前乗算で適用する。
    fn post_scale(&mut self, sx: f64, sy: f64) {
        self.state.matrix.post_scale(sx, sy);
    }

    /// 回転を前乗算で適用する。角度はラジアン。
    fn post_rotate(&mut self, angle: f64) {
        self.state.matrix.post_rotate(angle);
    }

    /// せん断を前乗算で適用する。
    fn post_skew(&mut self, kx: f64, ky: f64) {
        self.state.matrix.post_skew(kx, ky);
    }

    /// 任意の行列を前乗算で適用する。
    fn post_transform(&mut self, m: &crate::matrix::Matrix2D) {
        self.state.matrix.post_transform(&m.get_inner());
    }

    // -------------------------------------------------------------------------
    // 塗りつぶし描画
    // -------------------------------------------------------------------------

    /// 画像全体をクリアする。
    fn clear_all(&mut self, py: Python) {
        self.with_ctx(py, |ctx| {
            ctx.clear_all();
        });
    }

    /// 指定矩形をクリアする。
    fn clear_rect(&mut self, py: Python, rect: &crate::geometry::Rect) {
        self.with_ctx(py, |ctx| {
            ctx.clear_rect(&rect.get_inner());
        });
    }

    /// 画像全体を塗りつぶす。
    fn fill_all(&mut self, py: Python) {
        self.with_ctx(py, |ctx| {
            ctx.fill_all();
        });
    }

    /// 矩形を塗りつぶす。
    fn fill_rect(&mut self, py: Python, rect: &crate::geometry::Rect) {
        self.with_ctx(py, |ctx| {
            ctx.fill_rect(&rect.get_inner());
        });
    }

    /// 円を塗りつぶす。
    fn fill_circle(&mut self, py: Python, circle: &crate::geometry::Circle) {
        self.with_ctx(py, |ctx| {
            ctx.fill_circle(&circle.get_inner());
        });
    }

    /// 楕円を塗りつぶす。
    fn fill_ellipse(&mut self, py: Python, ellipse: &crate::geometry::Ellipse) {
        self.with_ctx(py, |ctx| {
            ctx.fill_ellipse(&ellipse.get_inner());
        });
    }

    /// 三角形を塗りつぶす。
    fn fill_triangle(&mut self, py: Python, t: &crate::geometry::Triangle) {
        self.with_ctx(py, |ctx| {
            ctx.fill_triangle(&t.get_inner());
        });
    }

    /// 角丸矩形を塗りつぶす。
    fn fill_round_rect(&mut self, py: Python, rr: &crate::geometry::RoundRect) {
        self.with_ctx(py, |ctx| {
            ctx.fill_round_rect(&rr.get_inner());
        });
    }

    /// 扇形を塗りつぶす。
    fn fill_pie(&mut self, py: Python, arc: &crate::geometry::Arc) {
        self.with_ctx(py, |ctx| {
            ctx.fill_pie(&arc.get_inner());
        });
    }

    /// ポリゴンを塗りつぶす。点列は Point または (x, y) タプル。
    fn fill_polygon(&mut self, py: Python, points: &Bound<'_, PyAny>) -> PyResult<()> {
        let points = crate::geometry::extract_points(points)?;
        self.with_ctx(py, |ctx| {
            ctx.fill_polygon(&points);
        });
        Ok(())
    }

    /// パスを塗りつぶす。
    fn fill_path(&mut self, py: Python, path: &crate::path::Path) {
        self.with_ctx(py, |ctx| {
            ctx.fill_path(path.get_inner());
        });
    }

    /// テキストを塗りつぶし描画する。
    fn fill_text(&mut self, py: Python, x: f64, y: f64, font: &crate::font::Font, text: &str) {
        self.with_ctx(py, |ctx| {
            ctx.fill_text(x, y, font.get_inner(), text);
        });
    }

    // -------------------------------------------------------------------------
    // ストローク属性
    // -------------------------------------------------------------------------

    /// ストローク幅を設定する。
    fn set_stroke_width(&mut self, width: f64) {
        self.state.stroke_width = width;
    }

    /// ストロークの端点形状を両端に一括設定する。
    fn set_stroke_cap(&mut self, cap: crate::style::StrokeCap) {
        let cap = cap.into();
        self.state.stroke_start_cap = cap;
        self.state.stroke_end_cap = cap;
    }

    /// ストロークの始点キャップを設定する。
    fn set_stroke_start_cap(&mut self, cap: crate::style::StrokeCap) {
        self.state.stroke_start_cap = cap.into();
    }

    /// ストロークの終点キャップを設定する。
    fn set_stroke_end_cap(&mut self, cap: crate::style::StrokeCap) {
        self.state.stroke_end_cap = cap.into();
    }

    /// ストロークの接続形状を設定する。
    fn set_stroke_join(&mut self, join: crate::style::StrokeJoin) {
        self.state.stroke_join = join.into();
    }

    /// ストロークのマイターリミットを設定する。
    fn set_stroke_miter_limit(&mut self, limit: f64) {
        self.state.stroke_miter_limit = limit;
    }

    /// ストロークのダッシュパターンを設定する。
    fn set_stroke_dash_array(&mut self, dash_array: Vec<f64>) {
        self.state.stroke_dash_array = dash_array;
    }

    /// ストロークのダッシュオフセットを設定する。
    fn set_stroke_dash_offset(&mut self, offset: f64) {
        self.state.stroke_dash_offset = offset;
    }

    // -------------------------------------------------------------------------
    // ストローク描画
    // -------------------------------------------------------------------------

    /// パスをストローク描画する。
    fn stroke_path(&mut self, py: Python, path: &crate::path::Path) {
        self.with_ctx(py, |ctx| {
            ctx.stroke_path(path.get_inner());
        });
    }

    /// 矩形をストローク描画する。
    fn stroke_rect(&mut self, py: Python, rect: &crate::geometry::Rect) {
        self.with_ctx(py, |ctx| {
            ctx.stroke_rect(&rect.get_inner());
        });
    }

    /// 円をストローク描画する。
    fn stroke_circle(&mut self, py: Python, circle: &crate::geometry::Circle) {
        self.with_ctx(py, |ctx| {
            ctx.stroke_circle(&circle.get_inner());
        });
    }

    /// 楕円をストローク描画する。
    fn stroke_ellipse(&mut self, py: Python, ellipse: &crate::geometry::Ellipse) {
        self.with_ctx(py, |ctx| {
            ctx.stroke_ellipse(&ellipse.get_inner());
        });
    }

    /// 三角形をストローク描画する。
    fn stroke_triangle(&mut self, py: Python, t: &crate::geometry::Triangle) {
        self.with_ctx(py, |ctx| {
            ctx.stroke_triangle(&t.get_inner());
        });
    }

    /// 角丸矩形をストローク描画する。
    fn stroke_round_rect(&mut self, py: Python, rr: &crate::geometry::RoundRect) {
        self.with_ctx(py, |ctx| {
            ctx.stroke_round_rect(&rr.get_inner());
        });
    }

    /// ポリゴンをストローク描画する。点列は Point または (x, y) タプル。
    fn stroke_polygon(&mut self, py: Python, points: &Bound<'_, PyAny>) -> PyResult<()> {
        let points = crate::geometry::extract_points(points)?;
        self.with_ctx(py, |ctx| {
            ctx.stroke_polygon(&points);
        });
        Ok(())
    }

    /// 折れ線をストローク描画する。点列は Point または (x, y) タプル。
    fn stroke_polyline(&mut self, py: Python, points: &Bound<'_, PyAny>) -> PyResult<()> {
        let points = crate::geometry::extract_points(points)?;
        self.with_ctx(py, |ctx| {
            ctx.stroke_polyline(&points);
        });
        Ok(())
    }

    /// 線分をストローク描画する。
    fn stroke_line(&mut self, py: Python, line: &crate::geometry::Line) {
        self.with_ctx(py, |ctx| {
            ctx.stroke_line(&line.get_inner());
        });
    }

    // -------------------------------------------------------------------------
    // 転送
    // -------------------------------------------------------------------------

    /// ソース画像を dst 矩形に転送する。
    #[pyo3(signature = (dst, src, src_rect=None))]
    fn blit_image_rect(
        &mut self,
        py: Python,
        dst: &crate::geometry::Rect,
        src: &crate::image::Image,
        src_rect: Option<&crate::geometry::Rect>,
    ) {
        let src_rect = src_rect.map(|r| r.get_inner());
        self.with_ctx(py, |ctx| {
            ctx.blit_image_rect(&dst.get_inner(), src.get_inner(), src_rect);
        });
    }

    /// ソース画像全体を (x, y) に転送する。
    fn blit_image_at(&mut self, py: Python, x: f64, y: f64, src: &crate::image::Image) {
        self.with_ctx(py, |ctx| {
            ctx.blit_image_at(x, y, src.get_inner());
        });
    }

    // -------------------------------------------------------------------------
    // ユーティリティ
    // -------------------------------------------------------------------------

    /// 現在の変換行列を取得する。
    fn matrix(&self) -> crate::matrix::Matrix2D {
        crate::matrix::Matrix2D::from_inner(self.state.matrix)
    }

    fn __repr__(&self) -> String {
        format!("Context(width={}, height={})", self.state.image_width, self.state.image_height)
    }
}

/// モジュール登録関数。
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Context>()?;
    Ok(())
}
