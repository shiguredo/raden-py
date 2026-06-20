// raden の Path API を PyO3 経由で Python に露出する。

use pyo3::prelude::*;

/// パスコマンドの種別を Python に露出する。
#[pyclass(eq, eq_int, from_py_object, name = "PathCmd")]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PathCmd {
    MoveTo = 0,
    LineTo = 1,
    QuadTo = 2,
    ConicTo = 3,
    CubicTo = 4,
    Close = 5,
}

impl From<raden::PathCmd> for PathCmd {
    fn from(cmd: raden::PathCmd) -> Self {
        match cmd {
            raden::PathCmd::MoveTo => PathCmd::MoveTo,
            raden::PathCmd::LineTo => PathCmd::LineTo,
            raden::PathCmd::QuadTo => PathCmd::QuadTo,
            raden::PathCmd::ConicTo => PathCmd::ConicTo,
            raden::PathCmd::CubicTo => PathCmd::CubicTo,
            raden::PathCmd::Close => PathCmd::Close,
        }
    }
}

impl From<PathCmd> for raden::PathCmd {
    fn from(cmd: PathCmd) -> Self {
        match cmd {
            PathCmd::MoveTo => raden::PathCmd::MoveTo,
            PathCmd::LineTo => raden::PathCmd::LineTo,
            PathCmd::QuadTo => raden::PathCmd::QuadTo,
            PathCmd::ConicTo => raden::PathCmd::ConicTo,
            PathCmd::CubicTo => raden::PathCmd::CubicTo,
            PathCmd::Close => raden::PathCmd::Close,
        }
    }
}

/// 2D パスを Python に露出する。
#[pyclass(name = "Path")]
pub struct Path {
    raden: raden::Path,
}

#[pymethods]
impl Path {
    #[new]
    fn new() -> Self {
        Self {
            raden: raden::Path::new(),
        }
    }

    /// パスをクリアする。
    fn clear(&mut self) {
        self.raden.clear();
    }

    /// パスが空かどうかを返す。
    fn is_empty(&self) -> bool {
        self.raden.is_empty()
    }

    /// コマンド数を返す。
    fn len(&self) -> usize {
        self.raden.len()
    }

    /// コマンド列を PathCmd のリストとして返す。
    fn cmds(&self) -> Vec<PathCmd> {
        self.raden.cmds().iter().map(|&cmd| cmd.into()).collect()
    }

    /// 点列を Point のリストとして返す。
    fn points(&self) -> Vec<crate::geometry::Point> {
        self.raden
            .points()
            .iter()
            .map(|&p| crate::geometry::Point { x: p.x, y: p.y })
            .collect()
    }

    /// 現在点を移動する。
    fn move_to(&mut self, x: f64, y: f64) {
        self.raden.move_to(x, y);
    }

    /// 直線を追加する。
    fn line_to(&mut self, x: f64, y: f64) {
        self.raden.line_to(x, y);
    }

    /// 3 次ベジェ曲線を追加する。
    fn cubic_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.raden.cubic_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    /// 2 次ベジェ曲線を追加する。
    fn quad_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.raden.quad_to(cpx, cpy, x, y);
    }

    /// 前の二次ベジェの制御点を反射したスムーズ二次ベジェを追加する。
    fn smooth_quad_to(&mut self, x: f64, y: f64) {
        self.raden.smooth_quad_to(x, y);
    }

    /// 前の三次ベジェの第 2 制御点を反射したスムーズ三次ベジェを追加する。
    fn smooth_cubic_to(&mut self, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.raden.smooth_cubic_to(cp2x, cp2y, x, y);
    }

    /// 円錐曲線 (有理二次ベジェ) を追加する。重み w は正数。
    fn conic_to(&mut self, cx: f64, cy: f64, ex: f64, ey: f64, w: f64) {
        self.raden.conic_to(cx, cy, ex, ey, w);
    }

    /// 楕円弧を現在点から接続する。
    #[allow(clippy::too_many_arguments)]
    fn arc_to(
        &mut self,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        start: f64,
        sweep: f64,
        force_move_to: bool,
    ) {
        self.raden
            .arc_to(cx, cy, rx, ry, start, sweep, force_move_to);
    }

    /// パスを閉じる。
    fn close(&mut self) {
        self.raden.close();
    }

    /// 扇形 (pie) を追加する。
    fn add_pie(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64) {
        self.raden.add_pie(cx, cy, rx, ry, start, sweep);
    }

    /// すべての頂点を平行移動する。
    fn translate(&mut self, dx: f64, dy: f64) {
        self.raden.translate(dx, dy);
    }

    /// すべての頂点に行列を適用する。
    fn transform(&mut self, m: &crate::matrix::Matrix2D) {
        self.raden.transform(&m.get_inner());
    }

    /// 別の Path を追加する。
    fn add_path(&mut self, other: &Path) {
        self.raden.add_path(&other.raden);
    }

    /// 別の Path を平行移動して追加する。
    fn add_path_translated(&mut self, other: &Path, dx: f64, dy: f64) {
        self.raden.add_path_translated(&other.raden, dx, dy);
    }

    /// 別の Path に行列を適用して追加する。
    fn add_path_transformed(&mut self, other: &Path, m: &crate::matrix::Matrix2D) {
        self.raden
            .add_path_transformed(&other.raden, &m.get_inner());
    }

    /// 三角形を追加する。
    fn add_triangle(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.raden.add_triangle(x0, y0, x1, y1, x2, y2);
    }

    /// 点列を結んで閉じたポリゴンを追加する。
    fn add_polygon(&mut self, points: &Bound<'_, PyAny>) -> PyResult<()> {
        let points = crate::geometry::extract_points(points)?;
        self.raden.add_polygon(&points);
        Ok(())
    }

    /// 点列を結んだ折れ線を追加する。
    fn add_polyline(&mut self, points: &Bound<'_, PyAny>) -> PyResult<()> {
        let points = crate::geometry::extract_points(points)?;
        self.raden.add_polyline(&points);
        Ok(())
    }

    /// 角丸矩形を追加する。
    fn add_round_rect(&mut self, x: f64, y: f64, w: f64, h: f64, rx: f64, ry: f64) {
        self.raden.add_round_rect(x, y, w, h, rx, ry);
    }

    /// 楕円を追加する。
    fn add_ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) {
        self.raden.add_ellipse(cx, cy, rx, ry);
    }

    /// 円を追加する。
    fn add_circle(&mut self, cx: f64, cy: f64, r: f64) {
        self.raden.add_circle(cx, cy, r);
    }

    /// 制御点ベースのバウンディングボックスを返す。パスが空の場合は None。
    fn control_box(&self) -> Option<crate::geometry::Rect> {
        self.raden
            .control_box()
            .map(crate::geometry::Rect::from_inner)
    }

    /// バウンディングボックスを返す。パスが空の場合は None。
    fn bounding_box(&self) -> Option<crate::geometry::Rect> {
        self.raden
            .bounding_box()
            .map(crate::geometry::Rect::from_inner)
    }

    /// 円錐曲線の重みリストを返す。
    fn conic_weights(&self) -> Vec<f64> {
        self.raden.conic_weights().to_vec()
    }
}

impl Path {
    pub(crate) fn get_inner(&self) -> &raden::Path {
        &self.raden
    }
}
