/*****************************************************************************************************************

This source file implements permutation constraint polynomials.

*****************************************************************************************************************/

use crate::constraints::ConstraintSystem;
use crate::polynomial::WitnessOverDomains;
use crate::scalars::{ProofEvaluations, RandomOracles};
use crate::wires::COLUMNS;
use algebra::{FftField, SquareRootField};
use ff_fft::{DenseOrSparsePolynomial, DensePolynomial, Evaluations, Radix2EvaluationDomain as D};
use oracle::{
    rndoracle::ProofError,
    utils::{EvalUtils, PolyUtils},
};
use rand::rngs::ThreadRng;

impl<F: FftField + SquareRootField> ConstraintSystem<F> {
    // permutation quotient poly contribution computation
    pub fn perm_quot(
        &self,
        lagrange: &WitnessOverDomains<F>,
        oracles: &RandomOracles<F>,
        z: &DensePolynomial<F>,
        alpha: &[F],
    ) -> Result<(Evaluations<F, D<F>>, DensePolynomial<F>), ProofError> {
        let l0 = &self.l08.scale(oracles.gamma1);

        let (bnd1, res) = DenseOrSparsePolynomial::divide_with_q_and_r(
            &(z - &DensePolynomial::from_coefficients_slice(&[F::one()])).into(),
            &DensePolynomial::from_coefficients_slice(&[-F::one(), F::one()]).into(),
        )
        .map_or(Err(ProofError::PolyDivision), |s| Ok(s))?;
        if res.is_zero() == false {
            return Err(ProofError::PolyDivision);
        }

        let (bnd2, res) = DenseOrSparsePolynomial::divide_with_q_and_r(
            &(z - &DensePolynomial::from_coefficients_slice(&[F::one()])).into(),
            &DensePolynomial::from_coefficients_slice(&[
                -self.sid[self.domain.d1.size as usize - 3],
                F::one(),
            ])
            .into(),
        )
        .map_or(Err(ProofError::PolyDivision), |s| Ok(s))?;
        if res.is_zero() == false {
            return Err(ProofError::PolyDivision);
        }

        Ok((
            &(&lagrange
                .d8
                .this
                .w
                .iter()
                .zip(self.shift.iter())
                .map(|(p, s)| p + &(l0 + &self.l18.scale(oracles.beta1 * s)))
                .fold(lagrange.d8.this.z.clone(), |x, y| &x * &y)
                - &lagrange
                    .d8
                    .this
                    .w
                    .iter()
                    .zip(self.sigmal8.iter())
                    .map(|(p, s)| p + &(l0 + &s.scale(oracles.beta1)))
                    .fold(lagrange.d8.next.z.clone(), |x, y| &x * &y))
                .scale(oracles.alpha)
                * &self.zkpl,
            &bnd1.scale(alpha[0]) + &bnd2.scale(alpha[1]),
        ))
    }

    // permutation linearization poly contribution computation
    pub fn perm_lnrz(
        &self,
        e: &Vec<ProofEvaluations<F>>,
        oracles: &RandomOracles<F>,
    ) -> DensePolynomial<F> {
        self.sigmam[COLUMNS - 1].scale(Self::perm_scalars(
            e,
            oracles,
            self.zkpm.evaluate(oracles.zeta),
        ))
    }

    pub fn perm_scalars(e: &Vec<ProofEvaluations<F>>, oracles: &RandomOracles<F>, z: F) -> F {
        -e[0]
            .w
            .iter()
            .zip(e[0].s.iter())
            .map(|(w, s)| oracles.gamma1 + &(oracles.beta1 * s) + w)
            .fold(e[1].z * &oracles.beta1 * &oracles.alpha * &z, |x, y| x * y)
    }

    // permutation aggregation polynomial computation
    pub fn perm_aggreg(
        &self,
        witness: &[Vec<F>; COLUMNS],
        oracles: &RandomOracles<F>,
        rng: &mut ThreadRng,
    ) -> Result<DensePolynomial<F>, ProofError> {
        let n = self.domain.d1.size as usize;
        let mut z = vec![F::one(); n];
        (0..n - 3).for_each(|j| {
            z[j + 1] = witness
                .iter()
                .zip(self.sigmal1.iter())
                .map(|(w, s)| w[j] + &(s[j] * &oracles.beta1) + &oracles.gamma1)
                .fold(F::one(), |x, y| x * y)
        });
        algebra::fields::batch_inversion::<F>(&mut z[1..=n - 3]);
        (0..n - 3).for_each(|j| {
            let x = z[j];
            z[j + 1] *= witness
                .iter()
                .zip(self.shift.iter())
                .map(|(w, s)| w[j] + &(self.sid[j] * &oracles.beta1 * s) + &oracles.gamma1)
                .fold(x, |z, y| z * y)
        });

        if z[n - 3] != F::one() {
            return Err(ProofError::ProofCreation);
        };
        z[n - 2] = F::rand(rng);
        z[n - 1] = F::rand(rng);
        Ok(Evaluations::<F, D<F>>::from_vec_and_domain(z, self.domain.d1).interpolate())
    }
}
