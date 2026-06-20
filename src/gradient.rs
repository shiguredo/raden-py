// raden の Gradient API を Python に露出する。
//
// Linear / Radial / Conic の 3 種類のグラデーション値クラスと、
// Gradient クラス (色停止点、拡張モード) を提供する。

use pyo3::prelude::*;

use crate::enums::ExtendMode;
use crate::style::Rgba32;

/// グラデーションの色停止点を Python に露出する。
#[pyclass(from_py_object, name = "GradientStop")]
#[derive(Clone, Copy, Debug)]
pub struct GradientStop {
    raden: raden::GradientStop,
}

#[pymethods]
impl GradientStop {
    #[new]
    fn new(offset: f64, color: &Rgba32) -> Self {
        Self {
            raden: raden::GradientStop {
                offset: offset.clamp(0.0, 1.0),
                color: color.inner(),
            },
        }
    }

    #[getter]
    fn offset(&self) -> f64 {
        self.raden.offset
    }

    #[getter]
    fn color(&self) -> Rgba32 {
        Rgba32::from_inner(self.raden.color)
    }

    fn __repr__(&self) -> String {
        format!(
            "GradientStop(offset={:.6}, color={:?})",
            self.raden.offset,
            Rgba32::from_inner(self.raden.color)
        )
    }
}

impl GradientStop {
    pub(crate) fn from_inner(raden: raden::GradientStop) -> Self {
        Self { raden }
    }
}

/// 線形グラデーションの定義値を Python に露出する。
#[pyclass(name = "LinearGradientValues", skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
pub struct LinearGradientValues {
    raden: raden::LinearGradientValues,
}

#[pymethods]
impl LinearGradientValues {
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
            "LinearGradientValues(x0={:.6}, y0={:.6}, x1={:.6}, y1={:.6})",
            self.raden.x0, self.raden.y0, self.raden.x1, self.raden.y1
        )
    }
}

/// 放射状グラデーションの定義値を Python に露出する。
#[pyclass(name = "RadialGradientValues", skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
pub struct RadialGradientValues {
    raden: raden::RadialGradientValues,
}

#[pymethods]
impl RadialGradientValues {
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
    fn r0(&self) -> f64 {
        self.raden.r0
    }

    #[getter]
    fn r1(&self) -> f64 {
        self.raden.r1
    }

    fn __repr__(&self) -> String {
        format!(
            "RadialGradientValues(x0={:.6}, y0={:.6}, x1={:.6}, y1={:.6}, r0={:.6}, r1={:.6})",
            self.raden.x0,
            self.raden.y0,
            self.raden.x1,
            self.raden.y1,
            self.raden.r0,
            self.raden.r1
        )
    }
}

/// 円錐グラデーションの定義値を Python に露出する。
#[pyclass(name = "ConicGradientValues", skip_from_py_object)]
#[derive(Clone, Copy, Debug)]
pub struct ConicGradientValues {
    raden: raden::ConicGradientValues,
}

#[pymethods]
impl ConicGradientValues {
    #[getter]
    fn x0(&self) -> f64 {
        self.raden.x0
    }

    #[getter]
    fn y0(&self) -> f64 {
        self.raden.y0
    }

    #[getter]
    fn angle(&self) -> f64 {
        self.raden.angle
    }

    fn __repr__(&self) -> String {
        format!(
            "ConicGradientValues(x0={:.6}, y0={:.6}, angle={:.6})",
            self.raden.x0, self.raden.y0, self.raden.angle
        )
    }
}

/// グラデーション値の種別を Python に露出する。
///
/// Linear / Radial / Conic のいずれかのバリアントを保持する。
/// Python 側では ``isinstance()`` で種別を判定できる。
#[pyclass(name = "GradientValues")]
pub struct GradientValues {
    inner: raden::GradientValues,
}

#[pymethods]
impl GradientValues {
    /// 線形グラデーション値として取得する。種別が一致しない場合は None。
    fn as_linear(&self) -> Option<LinearGradientValues> {
        match &self.inner {
            raden::GradientValues::Linear(v) => Some(LinearGradientValues { raden: *v }),
            _ => None,
        }
    }

    /// 放射状グラデーション値として取得する。種別が一致しない場合は None。
    fn as_radial(&self) -> Option<RadialGradientValues> {
        match &self.inner {
            raden::GradientValues::Radial(v) => Some(RadialGradientValues { raden: *v }),
            _ => None,
        }
    }

    /// 円錐グラデーション値として取得する。種別が一致しない場合は None。
    fn as_conic(&self) -> Option<ConicGradientValues> {
        match &self.inner {
            raden::GradientValues::Conic(v) => Some(ConicGradientValues { raden: *v }),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            raden::GradientValues::Linear(v) => {
                format!(
                    "GradientValues(Linear(x0={:.6}, y0={:.6}, x1={:.6}, y1={:.6}))",
                    v.x0, v.y0, v.x1, v.y1
                )
            }
            raden::GradientValues::Radial(v) => {
                format!(
                    "GradientValues(Radial(x0={:.6}, y0={:.6}, x1={:.6}, y1={:.6}, r0={:.6}, r1={:.6}))",
                    v.x0, v.y0, v.x1, v.y1, v.r0, v.r1
                )
            }
            raden::GradientValues::Conic(v) => {
                format!(
                    "GradientValues(Conic(x0={:.6}, y0={:.6}, angle={:.6}))",
                    v.x0, v.y0, v.angle
                )
            }
        }
    }
}

/// グラデーションを Python に露出する。
#[pyclass(name = "Gradient")]
pub struct Gradient {
    raden: raden::Gradient,
}

#[pymethods]
impl Gradient {
    /// Linear Gradient を生成する。
    #[staticmethod]
    fn new_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self {
            raden: raden::Gradient::new_linear(x0, y0, x1, y1),
        }
    }

    /// Radial Gradient を生成する。
    #[staticmethod]
    fn new_radial(x0: f64, y0: f64, x1: f64, y1: f64, r0: f64, r1: f64) -> Self {
        Self {
            raden: raden::Gradient::new_radial(x0, y0, x1, y1, r0, r1),
        }
    }

    /// Conic Gradient を生成する。
    #[staticmethod]
    fn new_conic(x0: f64, y0: f64, angle: f64) -> Self {
        Self {
            raden: raden::Gradient::new_conic(x0, y0, angle),
        }
    }

    /// 色停止点を追加する。
    fn add_stop(&mut self, offset: f64, color: &Rgba32) {
        self.raden.add_stop(offset, color.inner());
    }

    /// 拡張モードを設定する。
    fn set_extend_mode(&mut self, mode: &ExtendMode) {
        self.raden.set_extend_mode((*mode).into());
    }

    /// 色停止点のリストを返す。
    fn stops(&self) -> Vec<GradientStop> {
        self.raden
            .stops()
            .iter()
            .map(|&s| GradientStop::from_inner(s))
            .collect()
    }

    /// 色停止点の数を返す。
    #[getter]
    fn stop_count(&self) -> usize {
        self.raden.stops().len()
    }

    /// 拡張モードを返す。
    #[getter]
    fn extend_mode(&self) -> ExtendMode {
        self.raden.extend_mode().into()
    }

    /// グラデーション値を返す。
    fn values(&self) -> GradientValues {
        GradientValues {
            inner: self.raden.values().clone(),
        }
    }
}

impl Gradient {
    pub(crate) fn get_inner(&self) -> &raden::Gradient {
        &self.raden
    }

    /// raden::Gradient から Python 側の wrapper を生成する。
    pub(crate) fn from_inner(raden: raden::Gradient) -> Self {
        Self { raden }
    }
}
