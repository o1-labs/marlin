/*****************************************************************************************************************

This source file implements Posedon constraint gate Plonk primitive.

Constraint vector format:

    [rc; SPONGE_WIDTH]: round constants

*****************************************************************************************************************/

use algebra::FftField;
use oracle::poseidon_5_wires::{PlonkSpongeConstants, sbox};
use crate::{wires::GateWires, wires::{COLUMNS, WIRES}, constraints::ConstraintSystem};
use crate::gate::{CircuitGate, GateType};
use array_init::array_init;

impl<F: FftField> CircuitGate<F>
{
    pub fn create_poseidon
    (
        row: usize,
        wires: GateWires,
        c: Vec<F>
    ) -> Self
    {
        CircuitGate
        {
            row,
            typ: GateType::Poseidon,
            wires,
            c
        }
    }

    pub fn verify_poseidon(&self, witness: &[Vec<F>; COLUMNS], cs: &ConstraintSystem<F>) -> bool
    {
        let this: [F; COLUMNS] = array_init(|i| sbox::<F, PlonkSpongeConstants>(witness[i][self.row]));
        let next: [F; COLUMNS] = array_init(|i| witness[i][self.row+1]);
        let rc = self.rc();

        let perm = cs.fr_sponge_params.mds.iter().enumerate().
            map(|(i, m)| rc[i] + &this.iter().zip(m.iter()).fold(F::zero(), |x, (s, &m)| m * s + x)).collect::<Vec<_>>();

        self.typ == GateType::Poseidon && perm.iter().zip(next.iter()).all(|(p, n)| p == n)
    }

    pub fn ps(&self) -> F {if self.typ == GateType::Poseidon {F::one()} else {F::zero()}}
    pub fn rc(&self) -> [F; COLUMNS]
    {
        array_init(|i| if self.typ == GateType::Poseidon {self.c[WIRES[i]]} else {F::zero()})
    }
}
