// raden の Pattern API を Python に露出する。

use pyo3::prelude::*;

/// 画像パターンを Python に露出する。
#[pyclass(name = "Pattern")]
pub struct Pattern {
    raden: raden::Pattern,
}

#[pymethods]
impl Pattern {
    /// Image からパターンを生成する。
    #[staticmethod]
    fn from_image(image: &crate::image::Image) -> Self {
        Self {
            raden: raden::Pattern::new(
                image.data_slice(),
                image.width(),
                image.height(),
                image.stride(),
            ),
        }
    }

    /// 生の BGRA/PRGB32 バイト列からパターンを生成する。
    #[staticmethod]
    #[pyo3(signature = (data, width, height, stride=None))]
    fn from_data(data: &[u8], width: u32, height: u32, stride: Option<usize>) -> Self {
        let stride = stride.unwrap_or_else(|| width as usize * 4);
        Self {
            raden: raden::Pattern::new(data, width, height, stride),
        }
    }

    /// 原点オフセットを設定する。
    fn set_origin(&mut self, tx: f64, ty: f64) {
        self.raden.set_origin(tx, ty);
    }

    /// ユーザ空間からテクスチャ空間への変換行列を設定する。
    fn set_transform(&mut self, m: &crate::matrix::Matrix2D) {
        self.raden.set_transform(m.get_inner());
    }

    /// 補間モードを設定する。
    fn set_filter(&mut self, filter: &crate::enums::PatternFilter) {
        self.raden.set_filter((*filter).into());
    }

    /// 拡張モードを設定する。
    fn set_extend_mode(&mut self, mode: &crate::enums::ExtendMode) {
        self.raden.set_extend_mode((*mode).into());
    }

    #[getter]
    fn width(&self) -> u32 {
        self.raden.width()
    }

    #[getter]
    fn height(&self) -> u32 {
        self.raden.height()
    }

    /// 拡張モードを返す。
    #[getter]
    fn extend_mode(&self) -> crate::enums::ExtendMode {
        self.raden.extend_mode().into()
    }

    /// 補間モードを返す。
    #[getter]
    fn filter(&self) -> crate::enums::PatternFilter {
        self.raden.filter().into()
    }

    /// 変換行列を返す。
    #[getter]
    fn transform(&self) -> crate::matrix::Matrix2D {
        crate::matrix::Matrix2D::from_inner(self.raden.transform())
    }
}

impl Pattern {
    pub(crate) fn get_inner(&self) -> &raden::Pattern {
        &self.raden
    }

    /// raden::Pattern から Python 側の wrapper を生成する。
    pub(crate) fn from_inner(raden: raden::Pattern) -> Self {
        Self { raden }
    }
}
