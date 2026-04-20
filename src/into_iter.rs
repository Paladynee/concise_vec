use core::marker::PhantomData;

use crate::lenty_provide::ProvideLenTy;

pub struct IntoIter<'data, T, LenTy: const ProvideLenTy> {
    pub(crate) _marker: PhantomData<(T, LenTy, &'data mut ())>,
}
