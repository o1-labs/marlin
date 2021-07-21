/*****************************************************************************************************************

This source file, for now, implements URS unit test suite driver. The following tests are implemented:

1. urs_test
   This unit test generates a Universal Reference String, computes its update and
   proceeds to the verification of URS update consistency against its zk-proof with
   the batched bilinear pairing map checks.

*****************************************************************************************************************/

use algebra::{bn_382::Bn_382, PairingEngine};
use colored::Colorize;
use commitment_pairing::urs::URS;
use rand_core::OsRng;
use std::time::Instant;

// The following test verifies the consistency of the
// URS generation with the pairings of the URS exponents
#[test]
fn urs_test() {
    test::<Bn_382>();
}

fn test<E: PairingEngine>() {
    let depth = 30;
    let iterations = 1;
    let mut rng = &mut OsRng;

    // generate sample URS string for circuit depth of up to 'depth'
    println!("{}", "Generating the initial URS".green());
    let mut start = Instant::now();
    let mut urs = URS::<E>::create(depth, vec![3, 7], &mut rng);
    println!("{}{:?}", "Execution time: ".yellow(), start.elapsed());

    for i in 0..iterations {
        println!("{}{:?}", "Iteration: ", i);
        println!("{}", "Computing the update of the URS".green());

        // save necessary URS elements to verify next update
        let hx = urs.hx;

        start = Instant::now();
        // update sample URS string for circuit depth of up to 'depth'
        urs.update(&mut rng);
        println!("{}{:?}", "Execution time: ".yellow(), start.elapsed());

        println!("{}", "Verifying the update against its zk-proof".green());
        start = Instant::now();
        assert_eq!(urs.check(hx, &mut rng), true);
        println!("{}{:?}", "Execution time: ".yellow(), start.elapsed());
    }
}
