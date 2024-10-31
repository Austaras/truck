use super::*;

impl<C, V: Copy> ExtrudedCurve<C, V> {
    /// Creates a linear extruded curve by extrusion.
    #[inline(always)]
    pub const fn by_extrusion(curve: C, vector: V) -> Self { Self { curve, vector } }

    /// Returns the curve before extruded.
    #[inline(always)]
    pub const fn entity_curve(&self) -> &C { &self.curve }
    /// Into the curve before revoluted.
    #[inline(always)]
    pub fn into_entity_curve(self) -> C { self.curve }

    /// Returns the vector of extruded curve.
    #[inline(always)]
    pub const fn extruding_vector(&self) -> V { self.vector }
}

impl<C> ParametricSurface for ExtrudedCurve<C, C::Vector>
where
    C: ParametricCurve,
    C::Point: EuclideanSpace<Scalar = f64, Diff = C::Vector>,
    C::Vector: VectorSpace<Scalar = f64>,
{
    type Point = C::Point;
    type Vector = C::Vector;
    #[inline(always)]
    fn subs(&self, u: f64, v: f64) -> C::Point { self.curve.subs(u) + self.vector * v }
    #[inline(always)]
    fn uder(&self, u: f64, _: f64) -> C::Vector { self.curve.der(u) }
    #[inline(always)]
    fn vder(&self, _: f64, _: f64) -> C::Vector { self.vector }
    #[inline(always)]
    fn uuder(&self, u: f64, _: f64) -> C::Vector { self.curve.der2(u) }
    #[inline(always)]
    fn uvder(&self, _: f64, _: f64) -> C::Vector { C::Vector::zero() }
    #[inline(always)]
    fn vvder(&self, _: f64, _: f64) -> C::Vector { C::Vector::zero() }
    #[inline(always)]
    fn parameter_range(&self) -> (ParameterRange, ParameterRange) {
        (
            self.curve.parameter_range(),
            (Bound::Included(0.0), Bound::Included(1.0)),
        )
    }
    #[inline(always)]
    fn u_period(&self) -> Option<f64> { self.curve.period() }
}

impl<C: ParametricCurve3D> ParametricSurface3D for ExtrudedCurve<C, Vector3> {
    #[inline(always)]
    fn normal(&self, u: f64, _: f64) -> C::Vector {
        self.curve.der(u).cross(self.vector).normalize()
    }
}

impl<C, V> BoundedSurface for ExtrudedCurve<C, V>
where
    C: BoundedCurve,
    Self: ParametricSurface,
{
}

impl<C: ParameterDivision1D, V> ParameterDivision2D for ExtrudedCurve<C, V> {
    #[inline(always)]
    fn parameter_division(
        &self,
        (urange, vrange): ((f64, f64), (f64, f64)),
        tol: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        (
            self.curve.parameter_division(urange, tol).0,
            vec![vrange.0, vrange.1],
        )
    }
}

impl<P, C> SearchParameter<D2> for ExtrudedCurve<C, P::Diff>
where
    P: EuclideanSpace<Scalar = f64> + MetricSpace<Metric = f64> + Tolerance,
    P::Diff: InnerSpace<Scalar = f64> + Tolerance,
    C: ParametricCurve<Point = P, Vector = P::Diff> + BoundedCurve,
{
    type Point = P;
    #[inline(always)]
    fn search_parameter<H: Into<SPHint2D>>(
        &self,
        point: P,
        hint: H,
        trials: usize,
    ) -> Option<(f64, f64)> {
        let hint = match hint.into() {
            SPHint2D::Parameter(x, y) => (x, y),
            SPHint2D::Range(range0, range1) => {
                algo::surface::presearch(self, point, (range0, range1), PRESEARCH_DIVISION)
            }
            SPHint2D::None => {
                algo::surface::presearch(self, point, self.range_tuple(), PRESEARCH_DIVISION)
            }
        };
        algo::surface::search_parameter(self, point, hint, trials)
    }
}

impl<C: ParametricCurve3D + BoundedCurve> SearchNearestParameter<D2> for ExtrudedCurve<C, Vector3> {
    type Point = Point3;
    #[inline(always)]
    fn search_nearest_parameter<H: Into<SPHint2D>>(
        &self,
        point: Point3,
        hint: H,
        trials: usize,
    ) -> Option<(f64, f64)> {
        let hint = match hint.into() {
            SPHint2D::Parameter(x, y) => (x, y),
            SPHint2D::Range(range0, range1) => {
                algo::surface::presearch(self, point, (range0, range1), PRESEARCH_DIVISION)
            }
            SPHint2D::None => {
                algo::surface::presearch(self, point, self.range_tuple(), PRESEARCH_DIVISION)
            }
        };
        algo::surface::search_nearest_parameter(self, point, hint, trials)
    }
}

impl<C: Invertible> Invertible for ExtrudedCurve<C, Vector3> {
    #[inline(always)]
    fn invert(&mut self) { self.curve.invert() }
    #[inline(always)]
    fn inverse(&self) -> Self {
        Self {
            curve: self.curve.inverse(),
            vector: self.vector,
        }
    }
}

impl<C: Transformed<Matrix4>> Transformed<Matrix4> for ExtrudedCurve<C, Vector3> {
    fn transform_by(&mut self, trans: Matrix4) {
        self.curve.transform_by(trans);
        self.vector = trans.transform_vector(self.vector);
    }
    fn transformed(&self, trans: Matrix4) -> Self {
        Self {
            curve: self.curve.transformed(trans),
            vector: trans.transform_vector(self.vector),
        }
    }
}

#[test]
fn extruded_curve_test() {
    let cpts = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    let spts = vec![
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0)],
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(0.0, 1.0, 1.0)],
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0)],
    ];
    let curve = BSplineCurve::new(KnotVec::bezier_knot(2), cpts);
    let surface0 = ExtrudedCurve::by_extrusion(curve, Vector3::unit_z());
    let surface1 = BSplineSurface::new((KnotVec::bezier_knot(2), KnotVec::bezier_knot(1)), spts);
    assert_eq!(surface0.range_tuple(), surface1.range_tuple());
    const N: usize = 10;
    for i in 0..=N {
        for j in 0..=N {
            let u = i as f64 / N as f64;
            let v = j as f64 / N as f64;
            assert_near!(
                surface0.subs(u, v),
                ParametricSurface::subs(&surface1, u, v)
            );
            assert_near!(surface0.uder(u, v), surface1.uder(u, v));
            assert_near!(surface0.vder(u, v), surface1.vder(u, v));
            assert_near!(surface0.uuder(u, v), surface1.uuder(u, v));
            assert_near!(surface0.uvder(u, v), surface1.uvder(u, v));
            assert_near!(surface0.vvder(u, v), surface1.vvder(u, v));
            assert_near!(surface0.normal(u, v), surface1.normal(u, v));
        }
    }
}
