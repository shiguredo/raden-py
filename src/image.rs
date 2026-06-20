// raden の Image API を PyO3 経由で Python に露出する。

use pyo3::prelude::*;
use pyo3::exceptions::PyIOError;
use pyo3::types::PyBytes;

/// 画像バッファを Python に露出する。
#[pyclass(name = "Image")]
pub struct Image {
    raden: raden::Image,
}

#[pymethods]
impl Image {
    /// 指定サイズと形式の空画像を生成する。
    #[new]
    fn new(width: u32, height: u32, format: &crate::enums::PixelFormat) -> Self {
        Self {
            raden: raden::Image::new(width, height, (*format).into()),
        }
    }

    /// 幅を返す。
    pub(crate) fn width(&self) -> u32 {
        self.raden.width()
    }

    /// 高さを返す。
    pub(crate) fn height(&self) -> u32 {
        self.raden.height()
    }

    /// ストライドを返す。
    pub(crate) fn stride(&self) -> usize {
        self.raden.stride()
    }

    /// ピクセル形式を返す。
    fn format(&self) -> crate::enums::PixelFormat {
        self.raden.format().into()
    }

    /// ピクセルデータを bytes として返す (コピー)。
    pub(crate) fn data<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.raden.data())
    }

    /// BMP ファイルとして保存する。
    pub(crate) fn save_bmp(&self, path: &str) -> PyResult<()> {
        self.raden
            .write_to_file(path)
            .map_err(|e| PyIOError::new_err(e.to_string()))
    }
}

impl Image {
    pub(crate) fn get_inner(&self) -> &raden::Image {
        &self.raden
    }

    pub(crate) fn get_inner_mut(&mut self) -> &mut raden::Image {
        &mut self.raden
    }

    pub(crate) fn data_slice(&self) -> &[u8] {
        self.raden.data()
    }


    pub(crate) fn from_inner(raden: raden::Image) -> Self {
        Self { raden }
    }
}

/// モジュール登録関数。
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Image>()?;
    Ok(())
}
