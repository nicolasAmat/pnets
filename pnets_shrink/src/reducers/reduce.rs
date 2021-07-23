use pnets::{PlaceId, TransitionId};

use crate::modifications::Modification;

/// Apply a reduction rule to the network
pub trait Reduce<Net> {
    /// Compute a reduction rule on the network
    ///
    /// In general, this method should iterate over each place or transition and try to apply the
    /// reduction to each place or transition.
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>);
}

/// All reduction which implement this trait should keep the count of markings (the set of equations
/// should has exactly the same number of solutions than the original network)
pub trait ConservativeReduce<Net>: Reduce<Net> {}

/// Try to apply a reduction rule on a specific place
#[allow(clippy::module_name_repetitions)]
pub trait PlaceReduce<Net>: Reduce<Net> {
    /// Try to apply a reduction rule on a specific place
    fn place_reduce(net: &mut Net, pl: PlaceId, modifications: &mut Vec<Modification>);
}

/// Try to apply a reduction rule on a specific place
#[allow(clippy::module_name_repetitions)]
pub trait TransitionReduce<Net>: Reduce<Net> {
    /// Try to apply a reduction rule on a specific place
    fn transition_reduce(net: &mut Net, tr: TransitionId, modifications: &mut Vec<Modification>);
}
