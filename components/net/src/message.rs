// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use protobuf;

use error::Result;

bitflags! {
    flags TransactionFlags: u32 {
        const TXN_RESPONSE = 0 << 31,
        const TXN_PARTIAL = 0 << 30,
        const TXN_ID = 0x3FFFFFFF,
    }
}

pub struct Message<T> {
    pub body: T,
    flags: TransactionFlags,
}

impl<T: protobuf::Message> Message<T> {
    pub fn new(body: T, txn: Option<u32>) -> Self {
        let flags = if txn.is_some() {
            match TransactionFlags::from_bits(txn.unwrap()) {
                Some(bits) => bits,
                None => panic!("invalid transaction id: {}", txn.unwrap()),
            }
        } else {
            TransactionFlags::empty()
        };
        Message {
            body: body,
            flags: flags,
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let bytes = try!(self.body.write_to_bytes());
        Ok(bytes)
    }

    pub fn txn_id(&self) -> Option<u32> {
        let id = self.flags & TXN_ID;
        if id.bits > 0 {
            Some(id.bits)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_txn_id() {
        let no_transaction = Message::new("body", None);
        assert_eq!(no_transaction.txn_id(), None);

        let transactional = Message::new("body", Some(1));
        assert_eq!(transactional.txn_id(), Some(1));

        let highbound = Message::new("body", Some(1073741823));
        assert_eq!(highbound.txn_id(), Some(1073741823));
    }

    #[should_panic]
    fn txn_id_overflow() {
        let msg = Message::new("body", Some(1073741824));
    }

    #[test]
    fn txn_is_response() {}

    #[test]
    fn txn_is_partial() {}
}
