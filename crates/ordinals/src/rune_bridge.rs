use {super::*, message::Message, tag::Tag};

mod message;
mod tag;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RuneBridge {
  pub block: u64,
}


impl RuneBridge {
  pub const MAGIC_NUMBER: opcodes::All = opcodes::all::OP_PUSHNUM_14;

  pub fn decipher(transaction: &Transaction) -> Option<u64> {
    let payload = match RuneBridge::payload(transaction) {
      Some(payload) => payload,
      None => return None,
    };

    let Message {
      block,
    } = Message::from_payload(transaction, &payload);

    if !block.is_some() {
      return None;
    }
    let mut block_bytes = [0u8; 8];
    block_bytes.copy_from_slice(&block.unwrap()[..]);
    Some(u64::from_le_bytes(block_bytes))
  }

  pub fn encipher(&self) -> ScriptBuf {
    let mut payload = Vec::new();

    varint::encode_to_vec(Tag::Protocol.into(), &mut payload);
    varint::encode_to_vec(Tag::RuneBridge.into(), &mut payload);
    varint::encode_to_vec(Tag::Block.into(), &mut payload);
    let block_bytes = (self.block as u128).to_le_bytes();
    for byte in block_bytes {
      varint::encode_to_vec(byte as u128, &mut payload);
    }


    let mut builder = script::Builder::new()
      .push_opcode(opcodes::all::OP_RETURN)
      .push_opcode(RuneBridge::MAGIC_NUMBER);

    for chunk in payload.chunks(MAX_SCRIPT_ELEMENT_SIZE) {
      let push: &script::PushBytes = chunk.try_into().unwrap();
      builder = builder.push_slice(push);
    }

    builder.into_script()
  }

  fn payload(transaction: &Transaction) -> Option<Vec<u8>> {
    // search transaction outputs for payload
    for output in &transaction.output {
      let mut instructions = output.script_pubkey.instructions();

      // payload starts with OP_RETURN
      if instructions.next() != Some(Ok(Instruction::Op(opcodes::all::OP_RETURN))) {
        continue;
      }

      // followed by the protocol identifier, ignoring errors, since OP_RETURN
      // scripts may be invalid
      if instructions.next() != Some(Ok(Instruction::Op(RuneBridge::MAGIC_NUMBER))) {
        continue;
      }

      // construct the payload by concatenating remaining data pushes
      let mut payload = Vec::new();

      for result in instructions {
        match result {
          Ok(Instruction::PushBytes(push)) => {
            payload.extend_from_slice(push.as_bytes());
          }
          Ok(Instruction::Op(_)) => return None,
          Err(_) => return None,
        }
      }

      return Some(payload);
    }

    None
  }
}
