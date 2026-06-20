// raden の Python バインディング。
//
// 各サブモジュールで raden の公開 API を PyO3 経由で Python に露出する。
// エントリポイントは `raden._raden` という名前の拡張モジュールとなる。

use pyo3::prelude::*;

mod context;
mod enums;
mod font;
mod geometry;
mod gradient;
mod image;
mod matrix;
mod path;
mod pattern;
mod style;

/// Python から `import raden._raden` される拡張モジュール。
#[pymodule(name = "_raden")]
pub mod _raden {
    #[pymodule_export]
    use crate::context::Context;

    #[pymodule_export]
    use crate::enums::{ExtendMode, PatternFilter, PixelFormat};

    #[pymodule_export]
    use crate::font::{Font, FontData, FontError, FontFace};

    #[pymodule_export]
    use crate::geometry::{Arc, Circle, Ellipse, Line, Point, Rect, RoundRect, Triangle};

    #[pymodule_export]
    use crate::gradient::{
        ConicGradientValues, Gradient, GradientStop, GradientValues, LinearGradientValues,
        RadialGradientValues,
    };

    #[pymodule_export]
    use crate::image::Image;

    #[pymodule_export]
    use crate::matrix::Matrix2D;

    #[pymodule_export]
    use crate::path::{Path, PathCmd};

    #[pymodule_export]
    use crate::pattern::Pattern;

    #[pymodule_export]
    use crate::style::{CompOp, FillRule, Rgba32, StrokeCap, StrokeJoin};
}
