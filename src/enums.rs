// raden の列挙型 (PixelFormat, ExtendMode, PatternFilter) を Python に露出する。

use pyo3::prelude::*;

use raden::api::gradient::ExtendMode as RExtendMode;
use raden::api::pattern::PatternFilter as RPatternFilter;
use raden::pixel::PixelFormat as RPixelFormat;

/// ピクセルフォーマット。
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PixelFormat {
    /// 32-bit premultiplied ARGB。u32 で 0xAARRGGBB。
    #[default]
    Prgb32 = 1,
    /// 32-bit XRGB。アルファは未使用。
    Xrgb32 = 2,
    /// アルファのみ 8bit/ピクセル。
    A8 = 3,
}

impl From<PixelFormat> for RPixelFormat {
    fn from(fmt: PixelFormat) -> Self {
        match fmt {
            PixelFormat::Prgb32 => RPixelFormat::Prgb32,
            PixelFormat::Xrgb32 => RPixelFormat::Xrgb32,
            PixelFormat::A8 => RPixelFormat::A8,
        }
    }
}

impl From<RPixelFormat> for PixelFormat {
    fn from(fmt: RPixelFormat) -> Self {
        match fmt {
            RPixelFormat::Prgb32 => PixelFormat::Prgb32,
            RPixelFormat::Xrgb32 => PixelFormat::Xrgb32,
            RPixelFormat::A8 => PixelFormat::A8,
        }
    }
}

impl PixelFormat {
    pub fn to_raden(&self) -> RPixelFormat {
        (*self).into()
    }

    pub fn from_raden(fmt: RPixelFormat) -> Self {
        fmt.into()
    }
}

/// グラデーション/パターンの範囲外処理モード。
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ExtendMode {
    /// 端の色で埋める (デフォルト)。
    #[default]
    Pad = 0,
    /// 繰り返す。
    Repeat = 1,
    /// 反転して繰り返す。
    Reflect = 2,
}

impl From<ExtendMode> for RExtendMode {
    fn from(mode: ExtendMode) -> Self {
        match mode {
            ExtendMode::Pad => RExtendMode::Pad,
            ExtendMode::Repeat => RExtendMode::Repeat,
            ExtendMode::Reflect => RExtendMode::Reflect,
        }
    }
}

impl From<RExtendMode> for ExtendMode {
    fn from(mode: RExtendMode) -> Self {
        match mode {
            RExtendMode::Pad => ExtendMode::Pad,
            RExtendMode::Repeat => ExtendMode::Repeat,
            RExtendMode::Reflect => ExtendMode::Reflect,
        }
    }
}

impl ExtendMode {
    pub fn to_raden(&self) -> RExtendMode {
        (*self).into()
    }

    pub fn from_raden(mode: RExtendMode) -> Self {
        mode.into()
    }
}

/// パターンのピクセル補間モード。
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PatternFilter {
    /// 最近傍 (デフォルト)。
    #[default]
    Nearest = 0,
    /// 双一次補間。
    Bilinear = 1,
}

impl From<PatternFilter> for RPatternFilter {
    fn from(filter: PatternFilter) -> Self {
        match filter {
            PatternFilter::Nearest => RPatternFilter::Nearest,
            PatternFilter::Bilinear => RPatternFilter::Bilinear,
        }
    }
}

impl From<RPatternFilter> for PatternFilter {
    fn from(filter: RPatternFilter) -> Self {
        match filter {
            RPatternFilter::Nearest => PatternFilter::Nearest,
            RPatternFilter::Bilinear => PatternFilter::Bilinear,
        }
    }
}

impl PatternFilter {
    pub fn to_raden(&self) -> RPatternFilter {
        (*self).into()
    }

    pub fn from_raden(filter: RPatternFilter) -> Self {
        filter.into()
    }
}

/// Python モジュールにこのファイルの型を登録する。
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PixelFormat>()?;
    m.add_class::<ExtendMode>()?;
    m.add_class::<PatternFilter>()?;
    Ok(())
}
