// raden のフォント API を PyO3 経由で Python に露出する。

use pyo3::create_exception;
use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

// raden のフォントエラーを Python 例外に変換するためのカスタム例外。
create_exception!(raden_py, FontError, pyo3::exceptions::PyException);

fn font_error_to_pyerr(err: raden::FontError) -> PyErr {
    match err {
        raden::FontError::Io(e) => PyIOError::new_err(e.to_string()),
        raden::FontError::InvalidData(msg) => PyValueError::new_err(msg.to_string()),
    }
}

/// フォントファイルのバイトデータ。
#[pyclass(name = "FontData")]
pub struct FontData {
    raden: raden::FontData,
}

#[pymethods]
impl FontData {
    /// ファイルからフォントデータを読み込む。
    #[staticmethod]
    fn from_file(path: &str) -> PyResult<Self> {
        Ok(Self {
            raden: raden::FontData::from_file(path).map_err(font_error_to_pyerr)?,
        })
    }

    /// バイト列からフォントデータを作成する。
    #[staticmethod]
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            raden: raden::FontData::from_bytes(bytes.to_vec()),
        }
    }

    /// バイト列を取得する。
    fn data<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.raden.data())
    }

    fn __repr__(&self) -> String {
        format!("FontData(bytes={})", self.raden.data().len())
    }
}

impl FontData {
    pub(crate) fn get_inner(&self) -> &raden::FontData {
        &self.raden
    }
}

/// パース済みフォントフェイス。
#[pyclass(name = "FontFace")]
pub struct FontFace {
    raden: raden::FontFace,
}

#[pymethods]
impl FontFace {
    /// FontData からフォントフェイスを作成する。
    #[staticmethod]
    #[pyo3(signature = (font_data, index=0))]
    fn from_data(font_data: &FontData, index: u32) -> PyResult<Self> {
        Ok(Self {
            raden: raden::FontFace::from_data(font_data.get_inner(), index)
                .map_err(font_error_to_pyerr)?,
        })
    }

    #[getter]
    fn units_per_em(&self) -> u16 {
        self.raden.units_per_em()
    }

    #[getter]
    fn ascent(&self) -> i16 {
        self.raden.ascent()
    }

    #[getter]
    fn descent(&self) -> i16 {
        self.raden.descent()
    }

    #[getter]
    fn line_gap(&self) -> i16 {
        self.raden.line_gap()
    }

    fn __repr__(&self) -> String {
        format!("FontFace(units_per_em={})", self.raden.units_per_em())
    }
}

impl FontFace {
    pub(crate) fn get_inner(&self) -> &raden::FontFace {
        &self.raden
    }
}

/// サイズ指定済みフォント。
#[pyclass(name = "Font")]
pub struct Font {
    raden: raden::Font,
}

#[pymethods]
impl Font {
    /// FontFace とサイズからフォントを作成する。
    #[staticmethod]
    fn from_face(face: &FontFace, size: f64) -> Self {
        Self {
            raden: raden::Font::from_face(face.get_inner(), size),
        }
    }

    #[getter]
    fn size(&self) -> f64 {
        self.raden.size()
    }

    #[getter]
    fn scale(&self) -> f64 {
        self.raden.scale()
    }

    #[getter]
    fn ascent(&self) -> f64 {
        self.raden.ascent()
    }

    #[getter]
    fn descent(&self) -> f64 {
        self.raden.descent()
    }

    fn __repr__(&self) -> String {
        format!("Font(size={})", self.raden.size())
    }
}

impl Font {
    pub(crate) fn get_inner(&self) -> &raden::Font {
        &self.raden
    }
}
