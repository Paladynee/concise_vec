#![allow(incomplete_features)]
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
mod lenfield;
mod lenty_provide;
mod raw_data;
mod into_iter;
mod stp;

use aligner::AlignTyProvider;
use aligner::Aligner;
pub use concise_vec::ConciseVec;
use lenty_provide::ProvideLenTy;
use stp::SizedTP;
