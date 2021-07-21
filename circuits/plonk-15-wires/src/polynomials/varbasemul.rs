/*****************************************************************************************************************

This source file implements short Weierstrass curve variable base scalar multiplication custom Plonk polynomials.

Acc := [2]T
for i = n-1 ... 0:
   Q := (r_i == 1) ? T : -T
   Acc := Acc + (Q + Acc)
return (d_0 == 0) ? Q - P : Q

One-bit round constraints:

S = (P + (b ? T : −T)) + P

VBSM gate constraints for THIS witness row
•	b1*(b1-1) = 0
•	b2*(b2-1) = 0
•	(xp - xt) * s1 = yp – (2b1-1)*yt
•	s1^2 - s2^2 = xt - xr
•	(2*xp + xt – s1^2) * (s1 + s2) = 2*yp
•	(xp – xr) * s2 = yr + yp
•	(xr - xt) * s3 = yr – (2b2-1)*yt
•	S3^2 – s4^2 = xt - xs
•	(2*xr + xt – s3^2) * (s3 + s4) = 2*yr
•	(xr – xs) * s4 = ys + yr
•	n = 32*n_n + 16*b2 + 8*b1 + 4*b3_n + 2*b2_n + b1_n

The constraints above are derived from the following EC Affine arithmetic equations:


    (xq1 - xp) * s1 = yq1 - yp
    s1^2 - s2^2 = xq1 - xr
    (2*xp + xq1 – s1^2) * (s1 + s2) = 2*yp
    (xp – xr) * s2 = yr + yp

    (xq2 - xr) * s3 = yq2 - yr
    s3^2 – s4^2 = xq2 - xs
    (2*xr + xq2 – s3^2) * (s3 + s4) = 2*yr
    (xr – xs) * s4 = ys + yr


VBSM gate constraints for NEXT witness row
•	b1*(b1-1) = 0
•	b2*(b2-1) = 0
•	b3*(b3-1) = 0
•	(xq - xp) * s1 = (2b1-1)*yt - yp
•	(2*xp – s1^2 + xq) * ((xp – xr) * s1 + yr + yp) = (xp – xr) * 2*yp
•	(yr + yp)^2 = (xp – xr)^2 * (s1^2 – xq + xr)
•	(xq - xr) * s3 = (2b2-1)*yt - yr
•	(2*xr – s3^2 + xq) * ((xr – xv) * s3 + yv + yr) = (xr – xv) * 2*yr
•	(yv + yr)^2 = (xr – xv)^2 * (s3^2 – xq + xv)
•	(xq - xv) * s5 = (2b3-1)*yt - yv
•	(2*xv – s5^2 + xq) * ((xv – xs) * s5 + ys + yv) = (xv – xs) * 2*yv
•	(ys + yv)^2 = (xv – xs)^2 * (s5^2 – xq + xs)

The constraints above are derived from the following EC Affine arithmetic equations:


    (xq1 - xp) * s1 = yq1 - yp
    s1^2 - s2^2 = xq1 - xr
    (2*xp + xq1 – s1^2) * (s1 + s2) = 2*yp
    (xp – xr) * s2 = yr + yp

    (xq2 - xr) * s3 = yq2 - yr
    s3^2 – s4^2 = xq2 - xv
    (2*xr + xq2 – s3^2) * (s3 + s4) = 2*yr
    (xr – xv) * s4 = yv + yr

    (xq3 - xv) * s5 = yq3 - yv
    s5^2 – s6^2 = xq3 - xs
    (2*xv + xq3 – s5^2) * (s5 + s6) = 2*yv
    (xv – xs) * s6 = ys + yv

=>

    (xq1 - xp) * s1 = yq1 - yp
    (2*xp – s1^2 + xq1) * ((xp – xr) * s1 + yr + yp) = (xp – xr) * 2*yp
    (yr + yp)^2 = (xp – xr)^2 * (s1^2 – xq1 + xr)

    (xq2 - xr) * s3 = yq2 - yr
    (2*xr – s3^2 + xq2) * ((xr – xv) * s3 + yv + yr) = (xr – xv) * 2*yr
    (yv + yr)^2 = (xr – xv)^2 * (s3^2 – xq2 + xv)

    (xq3 - xv) * s5 = yq3 - yv
    (2*xv – s5^2 + xq3) * ((xv – xs) * s5 + ys + yv) = (xv – xs) * 2*yv
    (ys + yv)^2 = (xv – xs)^2 * (s5^2 – xq3 + xs)


    Row	    0	1	2	3	4	5	6	7	8	9	10	11	12	13	14	Type

       i	xT	yT	xS	yS	xP	yP	n	xr	yr	s1	s2	b1	s3	s4	b2	VBSM
      i+1	s5	b3	xS	yS	xP	yP	n	xr	yr	xv	yv	s1	b1	s3	b2	ZERO

    i+100	xT	yT	xS	yS	xP	yP	n	xr	yr	s1	s2	b1	s3	s4	b2	VBSM
    i+101	s5	b3	xS	yS	xP	yP	n	xr	yr	xv	yv	s1	b1	s3	b2	ZERO


*****************************************************************************************************************/

use crate::nolookup::constraints::ConstraintSystem;
use crate::nolookup::scalars::ProofEvaluations;
use crate::polynomial::WitnessOverDomains;
use algebra::{FftField, SquareRootField};
use ff_fft::{DensePolynomial, Evaluations, Radix2EvaluationDomain as D};
use oracle::utils::{EvalUtils, PolyUtils};

impl<F: FftField + SquareRootField> ConstraintSystem<F> {
    // scalar multiplication constraint quotient poly contribution computation
    pub fn vbmul_quot(
        &self,
        polys: &WitnessOverDomains<F>,
        alpha: &[F],
    ) -> (Evaluations<F, D<F>>, Evaluations<F, D<F>>) {
        if self.mulm.is_zero() {
            return (self.zero4.clone(), self.zero8.clone());
        }

        let this4 = &polys.d4.this.w;
        let this8 = &polys.d8.this.w;
        let next4 = &polys.d4.next.w;
        let next8 = &polys.d8.next.w;

        let p4 = [
            // verify booleanity of the scalar bits
            &this4[11] - &this4[11].pow(2),
            &this4[14] - &this4[14].pow(2),
            &next4[12] - &next4[12].pow(2),
            &next4[14] - &next4[14].pow(2),
            &next4[1] - &next4[1].pow(2),
            // accumulate packing
            &(&(&(&(&(&next4[7].scale(F::from(2 as u64)) + &this4[11]).scale(F::from(2 as u64))
                + &this4[14])
                .scale(F::from(2 as u64))
                + &next4[12])
                .scale(F::from(2 as u64))
                + &next4[14])
                .scale(F::from(2 as u64))
                + &next4[2])
                - &this4[7],
            // (xp - xt) * s1 = yp – (2*b1-1)*yt
            &(&(&(&this4[4] - &this4[0]) * &this4[9]) - &this4[5])
                + &(&this4[1] * &(&this4[11].scale(F::from(2 as u64)) - &self.l04)),
            // s1^2 - s2^2 = xt - xr
            &(&(&this4[9].pow(2) - &this4[10].pow(2)) - &this4[0]) + &this4[7],
            // (2*xp + xt – s1^2) * (s1 + s2) = 2*yp
            &(&(&(&this4[4].scale(F::from(2 as u64)) + &this4[0]) - &this4[9].pow(2))
                * &(&this4[9] + &this4[10]))
                - &this4[5].scale(F::from(2 as u64)),
            // (xp – xr) * s2 = yr + yp
            &(&(&(&this4[4] - &this4[7]) * &this4[10]) - &this4[8]) - &this4[5],
            // (xr - xt) * s3 = yr – (2b2-1)*yt
            &(&(&(&this4[7] - &this4[0]) * &this4[12]) - &this4[8])
                + &(&this4[1] * &(&this4[14].scale(F::from(2 as u64)) - &self.l04)),
            // S3^2 – s4^2 = xt - xs
            &(&(&this4[12].pow(2) - &this4[13].pow(2)) - &this4[0]) + &this4[2],
            // (2*xr + xt – s3^2) * (s3 + s4) = 2*yr
            &(&(&(&this4[7].scale(F::from(2 as u64)) + &this4[0]) - &this4[12].pow(2))
                * &(&this4[12] + &this4[13]))
                - &this4[8].scale(F::from(2 as u64)),
            // (xr – xs) * s4 = ys + yr
            &(&(&(&this4[7] - &this4[2]) * &this4[13]) - &this4[3]) - &this4[8],
        ];

        let xpmxr = &(&next8[4] - &next8[7]);
        let xrmxv = &(&next8[7] - &next8[9]);
        let xvmxs = &(&next8[9] - &next8[2]);
        let p8 = [
            // (xt - xp) * s1 = (2b1-1)*yt - yp
            &(&(&(&this8[0] - &next8[4]) * &next8[11])
                - &(&(&next8[12].scale(F::from(2 as u64)) - &self.l04) * &this8[1]))
                + &next8[5],
            // (2*xp – s1^2 + xt) * ((xp – xr) * s1 + yr + yp) = (xp – xr) * 2*yp
            &(&(&(&next8[4].scale(F::from(2 as u64)) - &next8[11].pow(2)) + &this8[0])
                * &(&(&(xpmxr * &next8[11]) + &next8[8]) + &next8[5]))
                - &(xpmxr * &next8[5].scale(F::from(2 as u64))),
            // (yr + yp)^2 = (xp – xr)^2 * (s1^2 – xt + xr)
            &(&next8[8] + &next8[5]).pow(2)
                - &(&xpmxr.pow(2) * &(&(&next8[11].pow(2) - &this8[0]) + &next8[7])),
            // (xt - xr) * s3 = (2b2-1)*yt - yr
            &(&(&(&this8[0] - &next8[7]) * &next8[13])
                - &(&(&next8[14].scale(F::from(2 as u64)) - &self.l04) * &this8[1]))
                + &next8[8],
            // (2*xr – s3^2 + xt) * ((xr – xv) * s3 + yv + yr) = (xr – xv) * 2*yr
            &(&(&(&next8[7].scale(F::from(2 as u64)) - &next8[13].pow(2)) + &this8[0])
                * &(&(&(xrmxv * &next8[13]) + &next8[10]) + &next8[8]))
                - &(xrmxv * &next8[8].scale(F::from(2 as u64))),
            // (yv + yr)^2 = (xr – xv)^2 * (s3^2 – xt + xv)
            &(&next8[10] + &next8[8]).pow(2)
                - &(&xrmxv.pow(2) * &(&(&next8[13].pow(2) - &this8[0]) + &next8[9])),
            // (xt - xv) * s5 = (2b3-1)*yt - yv
            &(&(&(&this8[0] - &next8[9]) * &next8[0])
                - &(&(&next8[1].scale(F::from(2 as u64)) - &self.l04) * &this8[1]))
                + &next8[10],
            // (2*xv – s5^2 + xt) * ((xv – xs) * s5 + ys + yv) = (xv – xs) * 2*yv
            &(&(&(&next8[9].scale(F::from(2 as u64)) - &next8[0].pow(2)) + &this8[0])
                * &(&(&(xvmxs * &next8[0]) + &next8[3]) + &next8[10]))
                - &(xvmxs * &next8[10].scale(F::from(2 as u64))),
            // (ys + yv)^2 = (xv – xs)^2 * (s5^2 – xt + xs)
            &(&next8[3] + &next8[10]).pow(2)
                - &(&xvmxs.pow(2) * &(&(&next8[0].pow(2) - &this8[0]) + &next8[2])),
        ];
        (
            &p4.iter()
                .skip(1)
                .zip(alpha.iter().skip(1))
                .map(|(p, a)| p.scale(*a))
                .fold(p4[0].scale(alpha[0]), |x, y| &x + &y)
                * &self.mull4,
            &p8.iter()
                .skip(1)
                .zip(alpha[p4.len() + 1..].iter())
                .map(|(p, a)| p.scale(*a))
                .fold(p8[0].scale(alpha[p4.len()]), |x, y| &x + &y)
                * &self.mull8,
        )
    }

    pub fn vbmul_scalars(evals: &Vec<ProofEvaluations<F>>, alpha: &[F]) -> F {
        let this = &evals[0].w;
        let next = &evals[1].w;
        let xpmxr = next[4] - &next[7];
        let xrmxv = next[7] - &next[9];
        let xvmxs = next[9] - &next[2];

        let s = [
            // verify booleanity of the scalar bits
            this[11] - &this[11].square(),
            this[14] - &this[14].square(),
            next[12] - &next[12].square(),
            next[14] - &next[14].square(),
            next[1] - &next[1].square(),
            // accumulate packing
            (((((next[7].double() + &this[11]).double() + &this[14]).double() + &next[12])
                .double()
                + &next[14])
                .double()
                + &next[2])
                - &this[7],
            // (xp - xt) * s1 = yp – (2*b1-1)*yt
            (((this[4] - &this[0]) * &this[9]) - &this[5])
                + &(this[1] * &(this[11].double() - &F::one())),
            // s1^2 - s2^2 = xt - xr
            ((this[9].square() - &this[10].square()) - &this[0]) + &this[7],
            // (2*xp + xt – s1^2) * (s1 + s2) = 2*yp
            (((this[4].double() + &this[0]) - &this[9].square()) * &(this[9] + &this[10]))
                - &this[5].double(),
            // (xp – xr) * s2 = yr + yp
            (((this[4] - &this[7]) * &this[10]) - &this[8]) - &this[5],
            // (xr - xt) * s3 = yr – (2b2-1)*yt
            (((this[7] - &this[0]) * &this[12]) - &this[8])
                + &(this[1] * &(this[14].double() - &F::one())),
            // S3^2 – s4^2 = xt - xs
            ((this[12].square() - &this[13].square()) - &this[0]) + &this[2],
            // (2*xr + xt – s3^2) * (s3 + s4) = 2*yr
            (((this[7].double() + &this[0]) - &this[12].square()) * &(this[12] + &this[13]))
                - &this[8].double(),
            // (xr – xs) * s4 = ys + yr
            (((this[7] - &this[2]) * &this[13]) - &this[3]) - &this[8],
            // (xt - xp) * s1 = (2b1-1)*yt - yp
            (((this[0] - &next[4]) * &next[11]) - &((next[12].double() - &F::one()) * &this[1]))
                + &next[5],
            // (2*xp – s1^2 + xt) * ((xp – xr) * s1 + yr + yp) = (xp – xr) * 2*yp
            (((next[4].double() - &next[11].square()) + &this[0])
                * &(((xpmxr * &next[11]) + &next[8]) + &next[5]))
                - &(xpmxr * &next[5].double()),
            // (yr + yp)^2 = (xp – xr)^2 * (s1^2 – xt + xr)
            (next[8] + &next[5]).square()
                - &(xpmxr.square() * &((next[11].square() - &this[0]) + &next[7])),
            // (xt - xr) * s3 = (2b2-1)*yt - yr
            (((this[0] - &next[7]) * &next[13]) - &((next[14].double() - &F::one()) * &this[1]))
                + &next[8],
            // (2*xr – s3^2 + xt) * ((xr – xv) * s3 + yv + yr) = (xr – xv) * 2*yr
            (((next[7].double() - &next[13].square()) + &this[0])
                * &(((xrmxv * &next[13]) + &next[10]) + &next[8]))
                - &(xrmxv * &next[8].double()),
            // (yv + yr)^2 = (xr – xv)^2 * (s3^2 – xt + xv)
            (next[10] + &next[8]).square()
                - &(xrmxv.square() * &((next[13].square() - &this[0]) + &next[9])),
            // (xt - xv) * s5 = (2b3-1)*yt - yv
            (((this[0] - &next[9]) * &next[0]) - &((next[1].double() - &F::one()) * &this[1]))
                + &next[10],
            // (2*xv – s5^2 + xt) * ((xv – xs) * s5 + ys + yv) = (xv – xs) * 2*yv
            (((next[9].double() - &next[0].square()) + &this[0])
                * &(((xvmxs * &next[0]) + &next[3]) + &next[10]))
                - &(xvmxs * &next[10].double()),
            // (ys + yv)^2 = (xv – xs)^2 * (s5^2 – xt + xs)
            (next[3] + &next[10]).square()
                - &(xvmxs.square() * &((next[0].square() - &this[0]) + &next[2])),
        ];

        s.iter()
            .zip(alpha.iter())
            .map(|(p, a)| *p * a)
            .fold(F::zero(), |x, y| x + y)
    }

    // scalar multiplication constraint linearization poly contribution computation
    pub fn vbmul_lnrz(&self, evals: &Vec<ProofEvaluations<F>>, alpha: &[F]) -> DensePolynomial<F> {
        self.mulm.scale(Self::vbmul_scalars(evals, alpha))
    }
}
