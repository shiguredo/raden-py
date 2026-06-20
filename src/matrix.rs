// raden の 2D アフィン変換行列を PyO3 経由で Python に露出する。

use pyo3::prelude::*;

/// 2D アフィン変換行列を Python に露出する。
#[pyclass(from_py_object, name = "Matrix2D")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2D {
    raden: raden::Matrix2D,
}

#[pymethods]
impl Matrix2D {
    /// 全要素を指定して行列を生成する。
    #[new]
    fn new(m00: f64, m01: f64, m10: f64, m11: f64, m20: f64, m21: f64) -> Self {
        Self {
            raden: raden::Matrix2D::new(m00, m01, m10, m11, m20, m21),
        }
    }

    /// 単位行列を返す。
    #[staticmethod]
    fn identity() -> Self {
        Self {
            raden: raden::Matrix2D::IDENTITY,
        }
    }

    /// 平行移動行列を返す。
    #[staticmethod]
    fn translation(tx: f64, ty: f64) -> Self {
        Self {
            raden: raden::Matrix2D::translation(tx, ty),
        }
    }

    /// スケーリング行列を返す。
    #[staticmethod]
    fn scaling(sx: f64, sy: f64) -> Self {
        Self {
            raden: raden::Matrix2D::scaling(sx, sy),
        }
    }

    /// 回転行列を返す。角度はラジアン。
    #[staticmethod]
    fn rotation(angle: f64) -> Self {
        Self {
            raden: raden::Matrix2D::rotation(angle),
        }
    }

    /// せん断行列を返す。
    #[staticmethod]
    fn skewing(kx: f64, ky: f64) -> Self {
        Self {
            raden: raden::Matrix2D::skewing(kx, ky),
        }
    }

    /// 単位行列かどうかを判定する。
    fn is_identity(&self) -> bool {
        self.raden.is_identity()
    }

    /// 行列を合成する (self * other)。
    fn multiply(&self, other: &Matrix2D) -> Matrix2D {
        Matrix2D {
            raden: self.raden.multiply(&other.raden),
        }
    }

    /// 点 (x, y) を変換する。
    fn map_point(&self, x: f64, y: f64) -> (f64, f64) {
        self.raden.map_point(x, y)
    }

    /// 平行移動を後乗算で適用する。
    fn translate(&mut self, tx: f64, ty: f64) {
        self.raden.translate(tx, ty);
    }

    /// スケーリングを後乗算で適用する。
    fn scale(&mut self, sx: f64, sy: f64) {
        self.raden.scale(sx, sy);
    }

    /// 回転を後乗算で適用する。角度はラジアン。
    fn rotate(&mut self, angle: f64) {
        self.raden.rotate(angle);
    }

    /// 任意の行列を後乗算で適用する。
    fn apply_matrix(&mut self, m: &Matrix2D) {
        self.raden = self.raden.multiply(&m.raden);
    }

    /// 変換行列を単位行列にリセットする。
    fn reset(&mut self) {
        self.raden.reset();
    }

    /// 指定中心まわりの回転を後乗算で適用する。角度はラジアン。
    fn rotate_around(&mut self, angle: f64, cx: f64, cy: f64) {
        self.raden.rotate_around(angle, cx, cy);
    }

    /// せん断を後乗算で適用する。
    fn skew(&mut self, kx: f64, ky: f64) {
        self.raden.skew(kx, ky);
    }

    /// 平行移動を前乗算で適用する。
    fn post_translate(&mut self, tx: f64, ty: f64) {
        self.raden.post_translate(tx, ty);
    }

    /// スケーリングを前乗算で適用する。
    fn post_scale(&mut self, sx: f64, sy: f64) {
        self.raden.post_scale(sx, sy);
    }

    /// 回転を前乗算で適用する。角度はラジアン。
    fn post_rotate(&mut self, angle: f64) {
        self.raden.post_rotate(angle);
    }

    /// せん断を前乗算で適用する。
    fn post_skew(&mut self, kx: f64, ky: f64) {
        self.raden.post_skew(kx, ky);
    }

    /// 任意の行列を前乗算で適用する。
    fn post_transform(&mut self, m: &Matrix2D) {
        self.raden.post_transform(&m.raden);
    }

    /// 逆行列を計算する。行列式がゼロの場合は None を返す。
    fn invert(&self) -> Option<Matrix2D> {
        self.raden.invert().map(|m| Matrix2D { raden: m })
    }

    /// 各要素を取得する。
    #[getter]
    fn m00(&self) -> f64 {
        self.raden.m00
    }

    #[getter]
    fn m01(&self) -> f64 {
        self.raden.m01
    }

    #[getter]
    fn m10(&self) -> f64 {
        self.raden.m10
    }

    #[getter]
    fn m11(&self) -> f64 {
        self.raden.m11
    }

    #[getter]
    fn m20(&self) -> f64 {
        self.raden.m20
    }

    #[getter]
    fn m21(&self) -> f64 {
        self.raden.m21
    }

    fn __repr__(&self) -> String {
        format!(
            "Matrix2D(m00={:.6}, m01={:.6}, m10={:.6}, m11={:.6}, m20={:.6}, m21={:.6})",
            self.raden.m00,
            self.raden.m01,
            self.raden.m10,
            self.raden.m11,
            self.raden.m20,
            self.raden.m21
        )
    }
}

impl Matrix2D {
    pub fn inner(&self) -> raden::Matrix2D {
        self.raden
    }

    pub fn get_inner(&self) -> raden::Matrix2D {
        self.raden
    }

    pub fn from_inner(raden: raden::Matrix2D) -> Self {
        Self { raden }
    }

    pub fn from_raden(raden: raden::Matrix2D) -> Self {
        Self { raden }
    }
}
