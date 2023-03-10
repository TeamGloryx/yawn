use super::composition::Composer;
use tupleops::ConcatTuples;

//                       (composer, changed)
pub type ComposerTuple = (&'static Composer, &'static u64);

pub type ComposableFn<A = (), R = ()> = dyn Fn<ConcatTuples<ComposerTuple, A>, Output = R>;
