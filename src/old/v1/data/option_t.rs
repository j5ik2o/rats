use functor::{Functor, KindFunctorExt};
use kind::{Empty, Kind, HKT};
use kinds::{OptionKind, OptionTKind};

#[derive(Debug, PartialEq)]
struct OptionT<'kind, K, A: 'kind, Z: 'kind = Empty>
where
    K: HKT,
{
    value: Kind<'kind, K, Kind<'kind, OptionKind, A, Empty>, Z>,
}

impl<'kind, F, A, Z> OptionT<'kind, F, A, Z>
where
    F: HKT,
    A: 'kind,
    Z: 'kind,
{
    fn map<B, Func>(self, f: Func) -> OptionT<'kind, F, B, Z>
    where
        F: Functor<F, Z>,
        Func: Fn(A) -> B,
    {
        unimplemented!();
    }
}
