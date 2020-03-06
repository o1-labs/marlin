use crate::poseidon::{ArithmeticSponge, ArithmeticSpongeParams, Sponge};
use algebra::{
    curves::{short_weierstrass_jacobian::GroupAffine, SWModelParameters},
    BigInteger, Field, FpParameters, PrimeField,
};

pub use crate::FqSponge;

pub const DIGEST_LENGTH_IN_LIMBS: usize = 4;
pub const CHALLENGE_LENGTH_IN_LIMBS: usize = 2;

const HIGH_ENTROPY_LIMBS: usize = 4;

#[derive(Clone)]
pub struct DefaultFqSponge<P: SWModelParameters> {
    pub params: ArithmeticSpongeParams<P::BaseField>,
    pub sponge: ArithmeticSponge<P::BaseField>,
    pub last_squeezed: Vec<u64>,
}

pub struct DefaultFrSponge<Fr: Field> {
    pub params: ArithmeticSpongeParams<Fr>,
    pub sponge: ArithmeticSponge<Fr>,
    pub last_squeezed: Vec<u64>,
}

fn pack<B: BigInteger>(limbs_lsb: &[u64]) -> B {
    let mut res: B = 0.into();
    for &x in limbs_lsb.iter().rev() {
        res.muln(64);
        res.add_nocarry(&x.into());
    }
    res
}

impl<Fr: PrimeField> DefaultFrSponge<Fr> {
    pub fn squeeze(&mut self, num_limbs: usize) -> Fr {
        if self.last_squeezed.len() >= num_limbs {
            let last_squeezed = self.last_squeezed.clone();
            let (limbs, remaining) = last_squeezed.split_at(num_limbs);
            self.last_squeezed = remaining.to_vec();
            Fr::from_repr(pack::<Fr::BigInt>(&limbs))
        } else {
            let x = self.sponge.squeeze(&self.params).into_repr();
            self.last_squeezed.extend(x.as_ref());
            self.squeeze(num_limbs)
        }
    }
}

impl<P: SWModelParameters> DefaultFqSponge<P>
where
    P::BaseField: PrimeField,
    <P::BaseField as PrimeField>::BigInt: Into<<P::ScalarField as PrimeField>::BigInt>,
{
    pub fn squeeze(&mut self, num_limbs: usize) -> P::ScalarField {
        if self.last_squeezed.len() >= num_limbs {
            let last_squeezed = self.last_squeezed.clone();
            let (limbs, remaining) = last_squeezed.split_at(num_limbs);
            self.last_squeezed = remaining.to_vec();
            P::ScalarField::from_repr(pack(&limbs))
        } else {
            let x = self.sponge.squeeze(&self.params).into_repr();
            self.last_squeezed
                .extend(&x.as_ref()[0..HIGH_ENTROPY_LIMBS]);
            self.squeeze(num_limbs)
        }
    }
}

impl<P: SWModelParameters> FqSponge<P::BaseField, GroupAffine<P>, P::ScalarField>
    for DefaultFqSponge<P>
where
    P::BaseField: PrimeField,
    <P::BaseField as PrimeField>::BigInt: Into<<P::ScalarField as PrimeField>::BigInt>,
{
    fn new(params: ArithmeticSpongeParams<P::BaseField>) -> DefaultFqSponge<P> {
        DefaultFqSponge {
            params,
            sponge: ArithmeticSponge::new(),
            last_squeezed: vec![],
        }
    }

    fn absorb_g(&mut self, g: &[GroupAffine<P>]) {
        self.last_squeezed = vec![];
        for g in g.iter()
        {
            if g.infinity {
                panic!("marlin sponge got zero curve point");
            } else {
                self.sponge.absorb(&self.params, &[g.x]);
                self.sponge.absorb(&self.params, &[g.y]);
            }
        }
    }

    fn absorb_fr(&mut self, x: &P::ScalarField) {
        self.last_squeezed = vec![];
        // Big endian
        let bits: Vec<bool> = x.into_repr().to_bits();

        if <P::ScalarField as PrimeField>::Params::MODULUS
            < <P::BaseField as PrimeField>::Params::MODULUS.into()
        {
            self.sponge.absorb(
                &self.params,
                &[P::BaseField::from_repr(<P::BaseField as PrimeField>::BigInt::from_bits(&bits))],
            );
        } else {
            let low_bits = &bits[1..];

            let high_bit = if bits[0] {
                P::BaseField::one()
            } else {
                P::BaseField::zero()
            };
            self.sponge.absorb(
                &self.params,
                &[P::BaseField::from_repr(<P::BaseField as PrimeField>::BigInt::from_bits(
                    &low_bits,
                ))],
            );
            self.sponge.absorb(&self.params, &[high_bit]);
        }
    }

    fn digest(mut self) -> P::ScalarField {
        self.squeeze(DIGEST_LENGTH_IN_LIMBS)
    }

    fn challenge(&mut self) -> P::ScalarField {
        self.squeeze(CHALLENGE_LENGTH_IN_LIMBS)
    }
}
