// raden の Image API を PyO3 経由で Python に露出する。
//
// バッファプロトコル (__getbuffer__ / __releasebuffer__) を実装しており、
// Python 側から memoryview や numpy.asarray でピクセルデータに直接アクセスできる。
// Prgb32 / Xrgb32 は (H, W, 4) uint8、A8 は (H, W) uint8 として扱われる。

use std::ffi::CString;
use std::os::raw::c_int;

use pyo3::exceptions::PyIOError;
use pyo3::ffi::{Py_buffer, Py_ssize_t};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::enums::PixelFormat;

/// 画像バッファを Python に露出する。
#[pyclass(name = "Image")]
pub struct Image {
    /// 内部の raden 画像。
    raden: raden::Image,
    /// バッファプロトコル用の shape 配列。
    /// Prgb32/Xrgb32 なら [H, W, 4]、A8 なら [H, W]。
    shape: Vec<Py_ssize_t>,
    /// バッファプロトコル用の strides 配列。
    /// Prgb32/Xrgb32 なら [stride, 4, 1]、A8 なら [stride, 1]。
    strides: Vec<Py_ssize_t>,
    /// バッファプロトコル用のフォーマット文字列 ("B" = unsigned char)。
    format: CString,
}

/// ピクセル形式に応じてバッファプロトコル用の shape / strides / format を計算する。
fn buffer_info(img: &raden::Image) -> (Vec<Py_ssize_t>, Vec<Py_ssize_t>, CString) {
    let h = img.height() as Py_ssize_t;
    let w = img.width() as Py_ssize_t;
    let stride = img.stride() as Py_ssize_t;
    let format = CString::new("B").unwrap();
    match img.format() {
        raden::pixel::PixelFormat::A8 => {
            // A8 は 1 チャネルなので 2 次元 (H, W)。
            (vec![h, w], vec![stride, 1], format)
        }
        _ => {
            // Prgb32 / Xrgb32 は 4 チャネルなので 3 次元 (H, W, 4)。
            (vec![h, w, 4], vec![stride, 4, 1], format)
        }
    }
}

#[pymethods]
impl Image {
    /// 指定サイズと形式の空画像を生成する。
    #[new]
    fn new(width: u32, height: u32, format: &PixelFormat) -> Self {
        let raden = raden::Image::new(width, height, (*format).into());
        let (shape, strides, format) = buffer_info(&raden);
        Self {
            raden,
            shape,
            strides,
            format,
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
    fn format(&self) -> PixelFormat {
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

    // -----------------------------------------------------------------
    // バッファプロトコル (PEP 3118)
    // -----------------------------------------------------------------

    /// memoryview や numpy.asarray からピクセルデータに直接アクセスするための
    /// バッファ情報を設定する。読み取り専用。
    unsafe fn __getbuffer__(
        slf: PyRef<'_, Self>,
        view: *mut Py_buffer,
        flags: c_int,
    ) -> PyResult<()> {
        let _ = flags;
        let data = slf.raden.data();
        unsafe {
            // view.obj に自己参照を設定する。これにより memoryview が Image の
            // 寿命を延ばし、バッファが無効にならないようにする。
            (*view).obj = pyo3::ffi::Py_NewRef(slf.as_ptr());
            // ピクセルデータの先頭ポインタを設定する。
            (*view).buf = data.as_ptr() as *mut std::ffi::c_void;
            // バッファ全体のバイト長を設定する。
            (*view).len = data.len() as Py_ssize_t;
            // 1 要素あたりのバイト数 (uint8 なので 1)。
            (*view).itemsize = 1;
            // 読み取り専用。
            (*view).readonly = 1;
            // 次元数 (Prgb32/Xrgb32 は 3、A8 は 2)。
            (*view).ndim = slf.shape.len() as c_int;
            // フォーマット文字列 ("B" = unsigned char)。
            (*view).format = slf.format.as_ptr() as *mut std::os::raw::c_char;
            // 各次元のサイズ。
            (*view).shape = slf.shape.as_ptr() as *mut Py_ssize_t;
            // 各次元のストライド (バイト単位)。
            (*view).strides = slf.strides.as_ptr() as *mut Py_ssize_t;
            // サブオフセットは使用しない。
            (*view).suboffsets = std::ptr::null_mut();
        }
        Ok(())
    }

    /// バッファの解放処理。shape / strides / format は Image フィールドとして
    /// 所有しているため、ここでの解放は不要。
    unsafe fn __releasebuffer__(&self, view: *mut Py_buffer) {
        let _ = view;
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
        let (shape, strides, format) = buffer_info(&raden);
        Self {
            raden,
            shape,
            strides,
            format,
        }
    }
}
