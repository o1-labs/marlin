/*****************************************************************************************************************

This source file has been copied from
https://gist.githubusercontent.com/imeckler/590adbed68c22288136d2d7987ac364c/raw/915840119a0fdd545b707621026478edc99a3195/poseidon.rs

It implements Poseidon Hash Function primitive

*****************************************************************************************************************/

use algebra::Field;

pub const ROUNDS_FULL: usize = 63;
pub const SPONGE_CAPACITY: usize = 1;
pub const SPONGE_RATE: usize = 2;
pub const SPONGE_BOX: usize = 5;
pub const SPONGE_WIDTH: usize = SPONGE_CAPACITY + SPONGE_RATE;

pub trait Sponge<Input, Digest> {
    type Params;
    fn new() -> Self;
    fn absorb(&mut self, params: &Self::Params, x: &[Input]);
    fn squeeze(&mut self, params: &Self::Params) -> Digest;
}

pub fn sbox<F: Field>(x: F) -> F {
    x.pow([SPONGE_BOX as u64])
}

#[derive(Clone)]
enum SpongeState {
    Absorbed(usize),
    Squeezed(usize),
}

#[derive(Clone)]
pub struct ArithmeticSpongeParams<F: Field> {
    pub round_constants: Vec<Vec<F>>,
    pub mds: Vec<Vec<F>>,
}

#[derive(Clone)]
pub struct ArithmeticSponge<F: Field> {
    sponge_state: SpongeState,
    rate: usize,
    pub state: Vec<F>,
}

impl<F: Field> ArithmeticSponge<F> {
    fn apply_mds_matrix(&mut self, params: &ArithmeticSpongeParams<F>) {
        self.state = params.mds.iter().
            map(|m| self.state.iter().zip(m.iter()).fold(F::zero(), |x, (s, &m)| m * s + x)).collect();
    }

    pub fn full_round(&mut self, r: usize, params: &ArithmeticSpongeParams<F>) {
        for i in 0..self.state.len() {
            self.state[i] = sbox(self.state[i]);
        }
        self.apply_mds_matrix(params);
        for (i, x) in params.round_constants[r].iter().enumerate() {
            self.state[i].add_assign(x);
        }
    }

    fn poseidon_block_cipher(&mut self, params: &ArithmeticSpongeParams<F>) {
        for r in 0..ROUNDS_FULL {
            self.full_round(r, params);
        }
    }
}

impl<F: Field> Sponge<F, F> for ArithmeticSponge<F> {
    type Params = ArithmeticSpongeParams<F>;

    fn new() -> ArithmeticSponge<F> {
        let capacity = SPONGE_CAPACITY;
        let rate = SPONGE_RATE;

        let mut state = Vec::with_capacity(capacity + rate);

        for _ in 0..(capacity + rate) {
            state.push(F::zero());
        }

        ArithmeticSponge {
            state,
            rate,
            sponge_state: SpongeState::Absorbed(0),
        }
    }

    fn absorb(&mut self, params: &ArithmeticSpongeParams<F>, x: &[F]) {
        for x in x.iter()
        {
            match self.sponge_state {
                SpongeState::Absorbed(n) => {
                    if n == self.rate {
                        self.poseidon_block_cipher(params);
                        self.sponge_state = SpongeState::Absorbed(1);
                        self.state[0].add_assign(x);
                    } else {
                        self.sponge_state = SpongeState::Absorbed(n + 1);
                        self.state[n].add_assign(x);
                    }
                }
                SpongeState::Squeezed(_n) => {
                    self.state[0].add_assign(x);
                    self.sponge_state = SpongeState::Absorbed(1);
                }
            }
        }
    }

    fn squeeze(&mut self, params: &ArithmeticSpongeParams<F>) -> F {
        match self.sponge_state {
            SpongeState::Squeezed(n) => {
                if n == self.rate {
                    self.poseidon_block_cipher(params);
                    self.sponge_state = SpongeState::Squeezed(1);
                    self.state[0]
                } else {
                    self.sponge_state = SpongeState::Squeezed(n + 1);
                    self.state[n]
                }
            }
            SpongeState::Absorbed(_n) => {
                self.poseidon_block_cipher(params);
                self.sponge_state = SpongeState::Squeezed(1);
                self.state[0]
            }
        }
    }
}
