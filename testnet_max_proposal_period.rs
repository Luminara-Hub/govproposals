use namada_tx_prelude::*;

pub const MAX_PROPOSAL_PERIOD: u64 = 260;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {

    // Update proposal period parameter
    let proposal_period_key = gov_storage::keys::get_max_proposal_period_key();
    ctx.write(&proposal_period_key, MAX_PROPOSAL_PERIOD)?;

    Ok(())
}
