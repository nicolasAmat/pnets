use pnets::io::Parse;
use pnets::standard::Net;
use pnets_shrunk::modifications::{
    Agglomeration, InequalityReduction, Modification, Reduction, TransitionElimination,
};
use pnets_shrunk::reducers::standard::{
    IdentityPlaceReducer, IdentityTransitionReducer, ParallelPlaceReducer,
    ParallelTransitionReducer, R7Reducer, SimpleChainReducer,
};
use pnets_shrunk::reducers::Reduce;

#[test]
fn simple_chain_agglomeration() {
    let parser = pnets_tina::Parser::new(include_str!("simple_chain_agglomeration.net").as_bytes());

    let mut net = Net::from(parser.parse().unwrap());
    let mut modifications = vec![];
    SimpleChainReducer::reduce(&mut net, &mut modifications);
    assert_eq!(
        modifications[0],
        Modification::Agglomeration(Agglomeration {
            deleted_places: vec![
                (net.search_place_by_name("p1").unwrap(), 1),
                (net.search_place_by_name("p2").unwrap(), 1)
            ],
            new_place: net.places.last_idx().unwrap(),
            constant: 0,
            factor: 1,
        })
    );
}

#[test]
fn parallel_transitions_fusion() {
    let parser =
        pnets_tina::Parser::new(include_str!("parallel_transitions_fusion.net").as_bytes());

    let mut net = Net::from(parser.parse().unwrap());
    let mut modifications = vec![];
    ParallelTransitionReducer::reduce(&mut net, &mut modifications);
    assert_eq!(
        modifications[0],
        Modification::TransitionElimination(TransitionElimination {
            deleted_transitions: vec![net.search_transition_by_name("t1").unwrap()]
        })
    );
}

#[test]
fn parallel_places_fusion() {
    let parser = pnets_tina::Parser::new(include_str!("parallel_places_fusion.net").as_bytes());

    let mut net = Net::from(parser.parse().unwrap());
    let mut modifications = vec![];
    ParallelPlaceReducer::reduce(&mut net, &mut modifications);
    assert_eq!(
        modifications[0],
        Modification::Reduction(Reduction {
            deleted_places: vec![(net.search_place_by_name("p1").unwrap(), 1)],
            equals_to: vec![(net.search_place_by_name("p2").unwrap(), 1)],
            constant: 0,
        })
    );
}

#[test]
fn constant_places_elimination() {
    let parser =
        pnets_tina::Parser::new(include_str!("constant_places_elimination.net").as_bytes());

    let mut net = Net::from(parser.parse().unwrap());
    let mut modifications = vec![];
    IdentityPlaceReducer::reduce(&mut net, &mut modifications);
    assert_eq!(
        modifications[0],
        Modification::Reduction(Reduction {
            deleted_places: vec![(net.search_place_by_name("p1").unwrap(), 1)],
            equals_to: vec![],
            constant: 5,
        })
    );
}

#[test]
fn r7_convert() {
    let parser = pnets_tina::Parser::new(include_str!("r7_convert.net").as_bytes());

    let mut net = Net::from(parser.parse().unwrap());
    let mut modifications = vec![];
    R7Reducer::reduce(&mut net, &mut modifications);
    assert_eq!(
        modifications[0],
        Modification::InequalityReduction(InequalityReduction {
            deleted_places: vec![(net.search_place_by_name("p1").unwrap(), 1)],
            kept_places: vec![],
            constant: 5,
        })
    );
}

#[test]
fn self_loop_place_elimination() {
    let parser =
        pnets_tina::Parser::new(include_str!("self_loop_place_eliminations.net").as_bytes());

    let mut net = Net::from(parser.parse().unwrap());
    let mut modifications = vec![];
    IdentityPlaceReducer::reduce(&mut net, &mut modifications);

    assert_eq!(
        modifications[0],
        Modification::Reduction(Reduction {
            deleted_places: vec![(net.search_place_by_name("p1").unwrap(), 1)],
            equals_to: vec![],
            constant: 5,
        })
    );
}

#[test]
fn self_loop_transition_elimination() {
    let parser =
        pnets_tina::Parser::new(include_str!("self_loop_transition_eliminations.net").as_bytes());

    let mut net = Net::from(parser.parse().unwrap());
    let mut modifications = vec![];
    IdentityTransitionReducer::reduce(&mut net, &mut modifications);

    assert_eq!(
        modifications[0],
        Modification::TransitionElimination(TransitionElimination {
            deleted_transitions: vec![net.search_transition_by_name("t1").unwrap()]
        })
    );
}
