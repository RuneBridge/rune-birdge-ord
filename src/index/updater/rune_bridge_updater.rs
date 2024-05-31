use anyhow::Ok;
use ordinals::RuneBridge;

use super::*;

pub(super) struct RuneBridgeUpdater<'a, 'tx, 'client> {
  pub(super) block_time: u32,
  pub(super) client: &'client Client,
  pub(super) height: u32,
  pub(super) inscription_to_rune_bridge_block: &'a mut Table<'tx, InscriptionIdValue, u64>,
}

impl<'a, 'tx, 'client> RuneBridgeUpdater<'a, 'tx, 'client> {
  pub(super) fn index_rune_bridge(&mut self, tx_index: u32, tx: &Transaction, txid: Txid) -> Result<()> {
    let block = match RuneBridge::decipher(tx) {
      Some(block) => block,
      None => return Ok(())
    };
    if block > self.height as u64 {
      return Ok(());
    }
    let envelopes = ParsedEnvelope::from_transaction(tx);
    if envelopes.len() != 1 {
      return Ok(());
    }
    let inscription_id = InscriptionId{txid, index: envelopes[0].input};
    self.inscription_to_rune_bridge_block.insert(inscription_id.store(), block)?;

    Ok(())
  }
}
