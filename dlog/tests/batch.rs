/*****************************************************************************************************************

This source file tests polynomial commitments, batched openings and
verification of a batch of batched opening proofs of polynomial commitments

*****************************************************************************************************************/

use algebra::{tweedle::{dee::{Affine, TweedledeeParameters}, Fp}, UniformRand};
use commitment_dlog::{srs::SRS, commitment::{CommitmentCurve}};
use oracle::utils::PolyUtils;

use oracle::FqSponge;
use oracle::sponge::{DefaultFqSponge};
use oracle::poseidon::{PlonkSpongeConstants as SC};

use std::time::{Instant, Duration};
use ff_fft::DensePolynomial;
use colored::Colorize;
use rand_core::OsRng;
use rand::Rng;
use groupmap::GroupMap;

#[test]
fn dlog_commitment_test()
where <Fp as std::str::FromStr>::Err : std::fmt::Debug
{
    let rng = &mut OsRng;
    let mut random = rand::thread_rng();

    let size = 1 << 7;
    let polysize = 500;
    let srs = SRS::<Affine>::create(size);

    let group_map = <Affine as CommitmentCurve>::Map::setup();
    let sponge = DefaultFqSponge::<TweedledeeParameters, SC>::new(oracle::tweedle::fq::params());

    let mut commit = Duration::new(0, 0);
    let mut open = Duration::new(0, 0);
    
    let prfs = (0..7).map
    (
        |_|
        {
            let length = (0..11).map
            (
                |_|
                {
                    let len: usize = random.gen();
                    (len % polysize)+1
                }
            ).collect::<Vec<_>>();
            println!("{}{:?}", "sizes: ".bright_cyan(), length);

            let a = length.iter().map(|s| DensePolynomial::<Fp>::rand(s-1,rng)).collect::<Vec<_>>();

            let x = (0..7).map(|_| Fp::rand(rng)).collect::<Vec<Fp>>();
            let polymask = Fp::rand(rng);
            let evalmask = Fp::rand(rng);

            let mut start = Instant::now();
            let proof = srs.open::<DefaultFqSponge<TweedledeeParameters, SC>>
            (
                &group_map,
                (0..a.len()).map
                (
                    |i| (&a[i], if i%2==0 {Some(a[i].coeffs.len())} else {None})
                ).collect::<Vec<_>>(),
                &x.clone(),
                polymask,
                evalmask,
                sponge.clone(),
                rng
            );
            open += start.elapsed();

            start = Instant::now();
            let comm =
            (
                sponge.clone(),
                x.clone(),
                polymask,
                evalmask,
                (0..a.len()).map
                (
                    |i|
                    (
                        if i%2==0 {srs.commit(&a[i].clone(), Some(a[i].coeffs.len()))} else {srs.commit(&a[i].clone(), None)},
                        x.iter().map(|xx| a[i].eval(*xx, size)).collect::<Vec<_>>(),
                        if i%2==0 {Some(a[i].coeffs.len())} else {None}
                    )
                ).collect::<Vec<_>>(),
                proof
            );
            commit += start.elapsed();
            comm
        }
    ).collect::<Vec<_>>();

    let mut proofs = prfs.iter().map
    (
        |proof|
        {
            (
                proof.0.clone(),
                proof.1.clone(),
                proof.2,
                proof.3,
                proof.4.iter().map
                (
                    |poly|
                    (
                        &poly.0,
                        poly.1.iter().map(|vector| vector).collect::<Vec<_>>(),
                        poly.2
                    )
                ).collect::<Vec<_>>(),
                &proof.5
            )
        }
    ).collect::<Vec<_>>();

    println!("{}{:?}", "commitment time: ".yellow(), commit);
    println!("{}{:?}", "open time: ".magenta(), open);

    let start = Instant::now();
    assert!(srs.verify::<DefaultFqSponge<TweedledeeParameters, SC>>(&group_map, &mut proofs, rng));
    println!("{}{:?}", "verification time: ".green(), start.elapsed());
}
