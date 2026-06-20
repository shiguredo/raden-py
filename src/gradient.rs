// raden の Gradient API を Python に露出する。

use pyo3::prelude::*;

/// グラデーションの色停止点を Python に露出する。
#[pyclass(name = "GradientStop")]
#[derive(Clone, Copy, Debug)]
pub struct GradientStop {
    raden: raden::GradientStop,
}

#[pymethods]
impl GradientStop {
    #[new]
    fn new(offset: f64, color: &crate::style::Rgba32) -> Self {
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
    fn color(&self) -> crate::style::Rgba32 {
        crate::style::Rgba32::from_inner(self.raden.color)
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
    fn add_stop(&mut self, offset: f64, color: &crate::style::Rgba32) {
        self.raden.add_stop(offset, color.inner());
    }

    /// 拡張モードを設定する。
    fn set_extend_mode(&mut self, mode: &crate::enums::ExtendMode) {
        self.raden.set_extend_mode((*mode).into());
    }
}

impl Gradient {
    pub(crate) fn get_inner(&self) -> &raden::Gradient {
        &self.raden
    }




}

/// モジュール登録関数。
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GradientStop>()?;
    m.add_class::<Gradient>()?;
    Ok(())
}
