// raden の幾何型を PyO3 経由で Python に露出する。

use pyo3::prelude::*;

/// 浮動小数点の矩形を Python に露出する。
#[pyclass(name = "Rect")]
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    raden: raden::Rect,
}

#[pymethods]
impl Rect {
    #[new]
    fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            raden: raden::Rect::new(x, y, w, h),
        }
    }

    #[getter]
    fn x(&self) -> f64 {
        self.raden.x
    }

    #[getter]
    fn y(&self) -> f64 {
        self.raden.y
    }

    #[getter]
    fn w(&self) -> f64 {
        self.raden.w
    }

    #[getter]
    fn h(&self) -> f64 {
        self.raden.h
    }

    fn __repr__(&self) -> String {
        format!(
            "Rect(x={:.6}, y={:.6}, w={:.6}, h={:.6})",
            self.raden.x, self.raden.y, self.raden.w, self.raden.h
        )
    }
}

impl Rect {
    pub(crate) fn get_inner(&self) -> raden::Rect {
        self.raden
    }

}

/// 円を Python に露出する。
#[pyclass(name = "Circle")]
#[derive(Clone, Copy, Debug)]
pub struct Circle {
    raden: raden::Circle,
}

#[pymethods]
impl Circle {
    #[new]
    fn new(cx: f64, cy: f64, r: f64) -> Self {
        Self {
            raden: raden::Circle::new(cx, cy, r),
        }
    }

    #[getter]
    fn cx(&self) -> f64 {
        self.raden.cx
    }

    #[getter]
    fn cy(&self) -> f64 {
        self.raden.cy
    }

    #[getter]
    fn r(&self) -> f64 {
        self.raden.r
    }

    fn __repr__(&self) -> String {
        format!(
            "Circle(cx={:.6}, cy={:.6}, r={:.6})",
            self.raden.cx, self.raden.cy, self.raden.r
        )
    }
}

impl Circle {
    pub(crate) fn get_inner(&self) -> raden::Circle {
        self.raden
    }

}

/// 楕円を Python に露出する。
#[pyclass(name = "Ellipse")]
#[derive(Clone, Copy, Debug)]
pub struct Ellipse {
    raden: raden::Ellipse,
}

#[pymethods]
impl Ellipse {
    #[new]
    fn new(cx: f64, cy: f64, rx: f64, ry: f64) -> Self {
        Self {
            raden: raden::Ellipse::new(cx, cy, rx, ry),
        }
    }

    #[getter]
    fn cx(&self) -> f64 {
        self.raden.cx
    }

    #[getter]
    fn cy(&self) -> f64 {
        self.raden.cy
    }

    #[getter]
    fn rx(&self) -> f64 {
        self.raden.rx
    }

    #[getter]
    fn ry(&self) -> f64 {
        self.raden.ry
    }

    fn __repr__(&self) -> String {
        format!(
            "Ellipse(cx={:.6}, cy={:.6}, rx={:.6}, ry={:.6})",
            self.raden.cx, self.raden.cy, self.raden.rx, self.raden.ry
        )
    }
}

impl Ellipse {
    pub(crate) fn get_inner(&self) -> raden::Ellipse {
        self.raden
    }

}

/// 角丸矩形を Python に露出する。
#[pyclass(name = "RoundRect")]
#[derive(Clone, Copy, Debug)]
pub struct RoundRect {
    raden: raden::RoundRect,
}

#[pymethods]
impl RoundRect {
    #[new]
    fn new(x: f64, y: f64, w: f64, h: f64, rx: f64, ry: f64) -> Self {
        Self {
            raden: raden::RoundRect::new(x, y, w, h, rx, ry),
        }
    }

    #[getter]
    fn x(&self) -> f64 {
        self.raden.x
    }

    #[getter]
    fn y(&self) -> f64 {
        self.raden.y
    }

    #[getter]
    fn w(&self) -> f64 {
        self.raden.w
    }

    #[getter]
    fn h(&self) -> f64 {
        self.raden.h
    }

    #[getter]
    fn rx(&self) -> f64 {
        self.raden.rx
    }

    #[getter]
    fn ry(&self) -> f64 {
        self.raden.ry
    }

    fn __repr__(&self) -> String {
        format!(
            "RoundRect(x={:.6}, y={:.6}, w={:.6}, h={:.6}, rx={:.6}, ry={:.6})",
            self.raden.x, self.raden.y, self.raden.w, self.raden.h, self.raden.rx, self.raden.ry
        )
    }
}

impl RoundRect {
    pub(crate) fn get_inner(&self) -> raden::RoundRect {
        self.raden
    }

}

/// 三角形を Python に露出する。
#[pyclass(name = "Triangle")]
#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    raden: raden::Triangle,
}

#[pymethods]
impl Triangle {
    #[new]
    fn new(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            raden: raden::Triangle::new(x0, y0, x1, y1, x2, y2),
        }
    }

    #[getter]
    fn x0(&self) -> f64 {
        self.raden.x0
    }

    #[getter]
    fn y0(&self) -> f64 {
        self.raden.y0
    }

    #[getter]
    fn x1(&self) -> f64 {
        self.raden.x1
    }

    #[getter]
    fn y1(&self) -> f64 {
        self.raden.y1
    }

    #[getter]
    fn x2(&self) -> f64 {
        self.raden.x2
    }

    #[getter]
    fn y2(&self) -> f64 {
        self.raden.y2
    }

    fn __repr__(&self) -> String {
        format!(
            "Triangle(x0={:.6}, y0={:.6}, x1={:.6}, y1={:.6}, x2={:.6}, y2={:.6})",
            self.raden.x0, self.raden.y0, self.raden.x1, self.raden.y1, self.raden.x2, self.raden.y2
        )
    }
}

impl Triangle {
    pub(crate) fn get_inner(&self) -> raden::Triangle {
        self.raden
    }

}

/// 線分を Python に露出する。
#[pyclass(name = "Line")]
#[derive(Clone, Copy, Debug)]
pub struct Line {
    raden: raden::Line,
}

#[pymethods]
impl Line {
    #[new]
    fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self {
            raden: raden::Line::new(x0, y0, x1, y1),
        }
    }

    #[getter]
    fn x0(&self) -> f64 {
        self.raden.x0
    }

    #[getter]
    fn y0(&self) -> f64 {
        self.raden.y0
    }

    #[getter]
    fn x1(&self) -> f64 {
        self.raden.x1
    }

    #[getter]
    fn y1(&self) -> f64 {
        self.raden.y1
    }

    fn __repr__(&self) -> String {
        format!(
            "Line(x0={:.6}, y0={:.6}, x1={:.6}, y1={:.6})",
            self.raden.x0, self.raden.y0, self.raden.x1, self.raden.y1
        )
    }
}

impl Line {
    pub(crate) fn get_inner(&self) -> raden::Line {
        self.raden
    }

}

/// 円弧を Python に露出する。
#[pyclass(name = "Arc")]
#[derive(Clone, Copy, Debug)]
pub struct Arc {
    raden: raden::Arc,
}

#[pymethods]
impl Arc {
    #[new]
    fn new(cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64) -> Self {
        Self {
            raden: raden::Arc::new(cx, cy, rx, ry, start, sweep),
        }
    }

    #[getter]
    fn cx(&self) -> f64 {
        self.raden.cx
    }

    #[getter]
    fn cy(&self) -> f64 {
        self.raden.cy
    }

    #[getter]
    fn rx(&self) -> f64 {
        self.raden.rx
    }

    #[getter]
    fn ry(&self) -> f64 {
        self.raden.ry
    }

    #[getter]
    fn start(&self) -> f64 {
        self.raden.start
    }

    #[getter]
    fn sweep(&self) -> f64 {
        self.raden.sweep
    }

    fn __repr__(&self) -> String {
        format!(
            "Arc(cx={:.6}, cy={:.6}, rx={:.6}, ry={:.6}, start={:.6}, sweep={:.6})",
            self.raden.cx, self.raden.cy, self.raden.rx, self.raden.ry, self.raden.start, self.raden.sweep
        )
    }
}

impl Arc {
    pub(crate) fn get_inner(&self) -> raden::Arc {
        self.raden
    }

}

/// 2D の点を Python に露出する。
#[pyclass(name = "Point")]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn __repr__(&self) -> String {
        format!("Point(x={:.6}, y={:.6})", self.x, self.y)
    }
}

impl Point {
}

/// Python オブジェクト (Point または (x, y) タプル) から raden::Point を取り出すヘルパー。
pub(crate) fn extract_point(ob: &Bound<'_, PyAny>) -> PyResult<raden::Point> {
    if let Ok(cell) = ob.downcast::<Point>() {
        let p = cell.borrow();
        Ok(raden::Point::new(p.x, p.y))
    } else {
        let (x, y): (f64, f64) = ob.extract()?;
        Ok(raden::Point::new(x, y))
    }
}

/// Python オブジェクトのリストから raden::Point の Vec を取り出すヘルパー。
pub(crate) fn extract_points(seq: &Bound<'_, PyAny>) -> PyResult<Vec<raden::Point>> {
    let mut points = Vec::new();
    for item in seq.try_iter()? {
        points.push(extract_point(&item?)?);
    }
    Ok(points)
}

/// モジュール登録関数。
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rect>()?;
    m.add_class::<Circle>()?;
    m.add_class::<Ellipse>()?;
    m.add_class::<RoundRect>()?;
    m.add_class::<Triangle>()?;
    m.add_class::<Line>()?;
    m.add_class::<Arc>()?;
    m.add_class::<Point>()?;
    Ok(())
}
