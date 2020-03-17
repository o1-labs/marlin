/*****************************************************************************************************************

This source file tests polynomial commitments, batched openings and
verification of a batch of batched opening proofs of polynomial commitments

*****************************************************************************************************************/

use algebra::{curves::bn_382::g::{Affine, Bn_382GParameters}, fields::bn_382::fp::Fp, UniformRand, AffineCurve, ProjectiveCurve};
use commitment_dlog::{srs::SRS, commitment::{CommitmentCurve, OpeningProof}};

use oracle::FqSponge;
use oracle::marlin_sponge::{DefaultFqSponge};

use std::time::{Instant, Duration};
use ff_fft::DensePolynomial;
use colored::Colorize;
use rand_core::OsRng;
use rand::Rng;
use groupmap::GroupMap;

type Fr = <Affine as AffineCurve>::ScalarField;

#[test]
fn shamir_equivalence()
{
    let rng = &mut OsRng;

    let g1 : Affine = (Affine::prime_subgroup_generator().into_projective() * &Fr::rand(rng)).into_affine();
    let g2 : Affine = (Affine::prime_subgroup_generator().into_projective() * &Fr::rand(rng)).into_affine();

    let x1 = Fr::rand(rng);
    let x2 = Fr::rand(rng);

    assert_eq!(commitment_dlog::commitment::shamir_sum(x1, g1, x2, g2), commitment_dlog::commitment::window_shamir(x1, g1, x2, g2))
}

#[test]
fn batch_commitment_test()
where <Fp as std::str::FromStr>::Err : std::fmt::Debug
{
    let rng = &mut OsRng;
    let mut random = rand::thread_rng();

    let depth = 2000;
    let srs = SRS::<Affine>::create(depth, rng);

    let group_map = <Affine as CommitmentCurve>::Map::setup();

    for i in 0..2
    {
        println!("{}{:?}", "test # ".bright_cyan(), i);

        let mut proofs = Vec::
        <(
            DefaultFqSponge<Bn_382GParameters>,
            Vec<Fr>,
            Fr,
            Fr,
            Vec<(Affine, Vec<Fr>, Option<(Affine, usize)>)>,
            OpeningProof<Affine>,
        )>::new();

        let mut commit = Duration::new(0, 0);
        let mut open = Duration::new(0, 0);
        
        for _ in 0..5
        {
            let size = (0..7).map
            (
                |_|
                {
                    let len: usize = random.gen();
                    (len % (depth-2))+1
                }
            ).collect::<Vec<_>>();
            println!("{}{:?}", "sizes: ".bright_cyan(), size);

            let a = size.iter().map(|s| DensePolynomial::<Fr>::rand(s-1,rng)).collect::<Vec<_>>();

            let mut start = Instant::now();
            let comm = (0..a.len()).map
            (
                |i|
                {
                    if i%2==0 {srs.commit_with_degree_bound(&a[i].clone(), a[i].coeffs.len()).unwrap()}
                    else {(srs.commit_no_degree_bound(&a[i].clone()).unwrap(), Affine::zero())}
                }
            ).collect::<Vec<_>>();
            commit += start.elapsed();

            let x = (0..7).map(|_| Fr::rand(rng)).collect::<Vec<Fr>>();
            let polymask = Fr::rand(rng);
            let evalmask = Fr::rand(rng);

            start = Instant::now();
            let sponge = DefaultFqSponge::<Bn_382GParameters>::new(oracle::bn_382::fp::params());

            let proof = srs.open::<DefaultFqSponge<Bn_382GParameters>>
            (   &group_map,
                &(0..a.len()).map
                (
                    |i| (a[i].clone(), if i%2==0 {Some(a[i].coeffs.len())} else {None})
                ).collect(),
                &x.clone(),
                polymask,
                evalmask,
                sponge.clone(),
                rng,
            ).unwrap();
            open += start.elapsed();

            proofs.push
            ((
                sponge.clone(),
                x.clone(),
                polymask,
                evalmask,
                (0..a.len()).map(|i| (comm[i].0, x.iter().map(|x| a[i].evaluate(*x)).collect(), if i%2==0 {Some((comm[i].1, a[i].coeffs.len()))} else {None})).collect(),
                proof,
            ));
        }

        println!("{}{:?}", "commitment time: ".yellow(), commit);
        println!("{}{:?}", "open time: ".magenta(), open);

        let start = Instant::now();
        assert_eq!(srs.verify::<DefaultFqSponge<Bn_382GParameters>>
            (
                &group_map,
                proofs,
                rng
            ), true);
        println!("{}{:?}", "verification time: ".green(), start.elapsed());
    }
}
