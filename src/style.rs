// raden のスタイル関連型 (Rgba32, CompOp, FillRule, StrokeCap, StrokeJoin) を Python に露出する。

use pyo3::prelude::*;

use raden::api::style::{
    CompOp as RCompOp, FillRule as RFillRule, Rgba32 as RRgba32, StrokeCap as RStrokeCap,
    StrokeJoin as RStrokeJoin,
};

/// 32bit RGBA カラー値を Python に露出する。
#[pyclass(name = "Rgba32", eq, hash, frozen)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rgba32 {
    inner: RRgba32,
}

#[pymethods]
impl Rgba32 {
    /// コンストラクタ。アルファ省略時は 255 とする。
    #[new]
    #[pyo3(signature = (r, g, b, a = 255))]
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            inner: RRgba32::new(r, g, b, a),
        }
    }

    /// RGB 値からアルファ 255 の色を生成するファクトリ。
    #[staticmethod]
    fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            inner: RRgba32::rgb(r, g, b),
        }
    }

    /// 赤成分を取得する。
    #[getter]
    fn r(&self) -> u8 {
        self.inner.r()
    }

    /// 緑成分を取得する。
    #[getter]
    fn g(&self) -> u8 {
        self.inner.g()
    }

    /// 青成分を取得する。
    #[getter]
    fn b(&self) -> u8 {
        self.inner.b()
    }

    /// アルファ成分を取得する。
    #[getter]
    fn a(&self) -> u8 {
        self.inner.a()
    }

    /// 文字列表現を返す。
    fn __repr__(&self) -> String {
        format!(
            "Rgba32(r={}, g={}, b={}, a={})",
            self.inner.r(),
            self.inner.g(),
            self.inner.b(),
            self.inner.a()
        )
    }
}

impl Rgba32 {
    pub(crate) fn inner(&self) -> RRgba32 {
        self.inner
    }

    pub(crate) fn from_inner(inner: RRgba32) -> Self {
        Self { inner }
    }


}

/// 合成オペレーション。
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CompOp {
    #[default]
    SrcOver = 0,
    SrcCopy = 1,
    SrcIn = 2,
    SrcOut = 3,
    SrcAtop = 4,
    DstOver = 5,
    DstCopy = 6,
    DstIn = 7,
    DstOut = 8,
    DstAtop = 9,
    Xor = 10,
    Clear = 11,
    Plus = 12,
    Minus = 13,
    Modulate = 14,
    Multiply = 15,
    Screen = 16,
    Overlay = 17,
    Darken = 18,
    Lighten = 19,
    ColorDodge = 20,
    ColorBurn = 21,
    LinearBurn = 22,
    LinearLight = 23,
    PinLight = 24,
    HardLight = 25,
    SoftLight = 26,
    Difference = 27,
    Exclusion = 28,
}

impl From<CompOp> for RCompOp {
    fn from(op: CompOp) -> Self {
        match op {
            CompOp::SrcOver => RCompOp::SrcOver,
            CompOp::SrcCopy => RCompOp::SrcCopy,
            CompOp::SrcIn => RCompOp::SrcIn,
            CompOp::SrcOut => RCompOp::SrcOut,
            CompOp::SrcAtop => RCompOp::SrcAtop,
            CompOp::DstOver => RCompOp::DstOver,
            CompOp::DstCopy => RCompOp::DstCopy,
            CompOp::DstIn => RCompOp::DstIn,
            CompOp::DstOut => RCompOp::DstOut,
            CompOp::DstAtop => RCompOp::DstAtop,
            CompOp::Xor => RCompOp::Xor,
            CompOp::Clear => RCompOp::Clear,
            CompOp::Plus => RCompOp::Plus,
            CompOp::Minus => RCompOp::Minus,
            CompOp::Modulate => RCompOp::Modulate,
            CompOp::Multiply => RCompOp::Multiply,
            CompOp::Screen => RCompOp::Screen,
            CompOp::Overlay => RCompOp::Overlay,
            CompOp::Darken => RCompOp::Darken,
            CompOp::Lighten => RCompOp::Lighten,
            CompOp::ColorDodge => RCompOp::ColorDodge,
            CompOp::ColorBurn => RCompOp::ColorBurn,
            CompOp::LinearBurn => RCompOp::LinearBurn,
            CompOp::LinearLight => RCompOp::LinearLight,
            CompOp::PinLight => RCompOp::PinLight,
            CompOp::HardLight => RCompOp::HardLight,
            CompOp::SoftLight => RCompOp::SoftLight,
            CompOp::Difference => RCompOp::Difference,
            CompOp::Exclusion => RCompOp::Exclusion,
        }
    }
}

impl CompOp {
    pub fn to_raden(&self) -> RCompOp {
        (*self).into()
    }
}

/// 塗りつぶし規則。
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FillRule {
    #[default]
    NonZero = 0,
    EvenOdd = 1,
}

impl From<FillRule> for RFillRule {
    fn from(rule: FillRule) -> Self {
        match rule {
            FillRule::NonZero => RFillRule::NonZero,
            FillRule::EvenOdd => RFillRule::EvenOdd,
        }
    }
}

impl FillRule {
    pub fn to_raden(&self) -> RFillRule {
        (*self).into()
    }
}

/// ストロークの端点形状。
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StrokeCap {
    #[default]
    Butt = 0,
    Square = 1,
    Round = 2,
}

impl From<StrokeCap> for RStrokeCap {
    fn from(cap: StrokeCap) -> Self {
        match cap {
            StrokeCap::Butt => RStrokeCap::Butt,
            StrokeCap::Square => RStrokeCap::Square,
            StrokeCap::Round => RStrokeCap::Round,
        }
    }
}

impl StrokeCap {
    pub fn to_raden(&self) -> RStrokeCap {
        (*self).into()
    }
}

/// ストロークの接続形状。
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StrokeJoin {
    #[default]
    MiterClip = 0,
    MiterBevel = 1,
    MiterRound = 2,
    Bevel = 3,
    Round = 4,
}

impl From<StrokeJoin> for RStrokeJoin {
    fn from(join: StrokeJoin) -> Self {
        match join {
            StrokeJoin::MiterClip => RStrokeJoin::MiterClip,
            StrokeJoin::MiterBevel => RStrokeJoin::MiterBevel,
            StrokeJoin::MiterRound => RStrokeJoin::MiterRound,
            StrokeJoin::Bevel => RStrokeJoin::Bevel,
            StrokeJoin::Round => RStrokeJoin::Round,
        }
    }
}

impl StrokeJoin {
    pub fn to_raden(&self) -> RStrokeJoin {
        (*self).into()
    }
}

/// Python モジュールにこのファイルの型を登録する。
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Rgba32>()?;
    m.add_class::<CompOp>()?;
    m.add_class::<FillRule>()?;
    m.add_class::<StrokeCap>()?;
    m.add_class::<StrokeJoin>()?;
    Ok(())
}
