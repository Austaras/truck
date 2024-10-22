//! Integrated modeling algorithms by geometry and topology
//!
//! There are some examples in `truck-modeling/examples`.

#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(clippy::all, rust_2018_idioms)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

/// re-export `truck_base`.
pub mod base {
    pub use truck_base::{
        assert_near, assert_near2, bounding_box::BoundingBox, cgmath64::*, tolerance::*,
    };
    pub use truck_geotrait::*;
}
pub use base::*;

/// geometrical elements
pub mod geometry;
pub use geometry::*;

/// topological elements
pub mod topology {
    use crate::{Point3, Curve, Surface};
    truck_topology::prelude!(Point3, Curve, Surface, pub);
}
pub use topology::*;

/// topological utility: [`Mapped`], [`Sweep`], and [`ClosedSweep`].
///
/// [`Mapped`]: ./topo_traits/trait.Mapped.html
/// [`Sweep`]: ./topo_traits/trait.Sweep.html
/// [`ClosedSweep`]: ./topo_traits/trait.ClosedSweep.html
pub mod topo_traits {
    /// Mapping, duplicates and moves a topological element.
    pub trait Mapped<P, C, S>: Sized {
        /// Returns a new topology whose points are mapped by `point_closure`,
        /// curves are mapped by `curve_closure`,
        /// and surfaces are mapped by `surface_closure`.
        #[doc(hidden)]
        fn mapped<FP: Fn(&P) -> P, FC: Fn(&C) -> C, FS: Fn(&S) -> S>(
            &self,
            point_mapping: &FP,
            curve_mapping: &FC,
            surface_mapping: &FS,
        ) -> Self;

        /// Returns another topology whose points, curves, and surfaces are cloned.
        fn topological_clone(&self) -> Self
        where
            P: Clone,
            C: Clone,
            S: Clone, {
            self.mapped(&Clone::clone, &Clone::clone, &Clone::clone)
        }
    }

    /// Abstract sweeping, builds a circle-arc, a prism, a half torus, and so on.
    pub trait Sweep<P, C, S> {
        /// The struct of sweeped topology.
        type Swept;
        /// Transform topologies and connect vertices and edges in boundaries.
        fn sweep<
            FP: Fn(&P) -> P,
            FC: Fn(&C) -> C,
            FS: Fn(&S) -> S,
            CP: Fn(&P, &P) -> C,
            CE: Fn(&C, &C) -> S,
        >(
            &self,
            point_mapping: &FP,
            curve_mapping: &FC,
            surface_mapping: &FS,
            connect_points: &CP,
            connect_curve: &CE,
        ) -> Self::Swept;
    }

    /// Abstract multi sweeping, builds a circle-arc, a prism, a half torus, and so on.
    pub trait MultiSweep<P, C, S> {
        /// The struct of sweeped topology.
        type Swept;
        /// Transform topologies and connect vertices and edges in boundaries.
        fn multi_sweep<
            FP: Fn(&P) -> P,
            FC: Fn(&C) -> C,
            FS: Fn(&S) -> S,
            CP: Fn(&P, &P) -> C,
            CE: Fn(&C, &C) -> S,
        >(
            &self,
            point_mapping: &FP,
            curve_mapping: &FC,
            surface_mapping: &FS,
            connect_points: &CP,
            connect_curve: &CE,
            division: usize,
        ) -> Self::Swept;
    }

    /// closed sweep, builds a closed torus, and so on.
    pub trait ClosedSweep<P, C, S>: MultiSweep<P, C, S> {
        /// Transform topologies and connect vertices and edges in boundaries.
        fn closed_sweep<
            FP: Fn(&P) -> P,
            FC: Fn(&C) -> C,
            FS: Fn(&S) -> S,
            CP: Fn(&P, &P) -> C,
            CE: Fn(&C, &C) -> S,
        >(
            &self,
            point_mapping: &FP,
            curve_mapping: &FC,
            surface_mapping: &FS,
            connect_points: &CP,
            connect_curves: &CE,
            division: usize,
        ) -> Self::Swept;
    }
}
pub use topo_traits::*;

/// `Result` with crate's errors.
pub type Result<T> = std::result::Result<T, errors::Error>;

/// the building model utility API
pub mod builder;
mod closed_sweep;
/// declare errors
pub mod errors;
mod geom_impls;
mod mapped;
mod multi_sweep;
mod sweep;
mod topo_impls;
