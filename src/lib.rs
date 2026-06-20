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
fn _raden(m: &Bound<'_, PyModule>) -> PyResult<()> {
    context::register(m)?;
    enums::register(m)?;
    font::register(m)?;
    geometry::register(m)?;
    gradient::register(m)?;
    image::register(m)?;
    matrix::register(m)?;
    path::register(m)?;
    pattern::register(m)?;
    style::register(m)?;
    Ok(())
}
