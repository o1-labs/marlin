/********************************************************************************************

This source file implements zk-proof batch verifier functionality.

*********************************************************************************************/

use rand_core::RngCore;
use algebra::PairingEngine;
use circuits::index::Index;
use oracle::rndoracle::ProofError;
pub use super::prover::{ProverProof, RandomOracles};
pub use super::batch_prover::BatchProof;

impl<E: PairingEngine> BatchProof<E>
{
    // This function verifies the batch of zk-proofs
    //     index: Index
    //     rng: randomness source context
    //     RETURN: verification status
    pub fn verify
    (
        &self,
        index: &Index<E>,
        rng: &mut dyn RngCore
    ) -> Result<bool, ProofError>
    {
        let mut batch = vec![Vec::new(), Vec::new(), Vec::new()];
        for proof in self.batch.iter()
        {
            let proof = proof.clone()?;
            let oracles = proof.oracles(index)?;

            // first, verify the sumcheck argument values
            if !proof.sumcheck_1_verify (index, &oracles) || !proof.sumcheck_2_verify (index, &oracles) || !proof.sumcheck_3_verify (index, &oracles)
            {
                return Err(ProofError::ProofVerification)
            }

            batch[0].push
            ((
                oracles.beta[0],
                oracles.batch,
                vec!
                [
                    (proof.za_comm, proof.za_eval, index.h_group.size()),
                    (proof.zb_comm, proof.zb_eval, index.h_group.size()),
                    (proof.w_comm,  proof.w_eval,  index.x_group.size()),
                    (proof.h1_comm, proof.h1_eval, index.h_group.size()*2-2),
                    (proof.g1_comm, proof.g1_eval, index.h_group.size()-1),
                ],
                proof.proof1
            ));
            batch[1].push
            ((
                oracles.beta[1],
                oracles.batch,
                vec!
                [
                    (proof.h2_comm, proof.h2_eval, index.h_group.size()-1),
                    (proof.g2_comm, proof.g2_eval, index.h_group.size()-1),
                ],
                proof.proof2
            ));
            batch[2].push
            ((
                oracles.beta[2],
                oracles.batch,
                vec!
                [
                    (proof.h3_comm, proof.h3_eval, index.k_group.size()*6-6),
                    (proof.g3_comm, proof.g3_eval, index.k_group.size()-1),
                    (index.compiled[0].row_comm, proof.row_eval[0], index.k_group.size()),
                    (index.compiled[1].row_comm, proof.row_eval[1], index.k_group.size()),
                    (index.compiled[2].row_comm, proof.row_eval[2], index.k_group.size()),
                    (index.compiled[0].col_comm, proof.col_eval[0], index.k_group.size()),
                    (index.compiled[1].col_comm, proof.col_eval[1], index.k_group.size()),
                    (index.compiled[2].col_comm, proof.col_eval[2], index.k_group.size()),
                    (index.compiled[0].val_comm, proof.val_eval[0], index.k_group.size()),
                    (index.compiled[1].val_comm, proof.val_eval[1], index.k_group.size()),
                    (index.compiled[2].val_comm, proof.val_eval[2], index.k_group.size()),
                ],
                proof.proof3
            ));
        }
        // second, verify the commitment opening proofs
        match index.urs.verify(&batch, rng)
        {
            false => Err(ProofError::OpenProof),
            true => Ok(true)
        }
    }
}
