#![allow(incomplete_features, clippy::type_repetition_in_bounds)]
#![feature(
    const_cmp,
    const_default,
    const_destruct,
    const_ops,
    const_slice_make_iter,
    const_trait_impl,
    generic_const_exprs
)]

mod aligner;
mod concise_vec;
mod heap_strategy_provider;
mod into_iter;
mod lenfield;
mod lenty_provide;
mod raw_data;
mod stp;

pub use concise_vec::ConciseVec;
