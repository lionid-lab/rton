// use std::borrow::Borrow;
// use std::convert::TryFrom;
//
// use crate::keys::ed25519;
// use rand::Rng;
//
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub struct NodeIdFull(ed25519::PublicKey);
//
// impl NodeIdFull {
//     pub const fn new(public_key: ed25519::PublicKey) -> Self {
//         Self(public_key)
//     }
//
//     #[inline(always)]
//     pub const fn public_key(&self) -> &ed25519::PublicKey {
//         &self.0
//     }
//
//     pub fn compute_short_id(&self) -> NodeIdShort {
//         NodeIdShort::new(tl_proto::hash(self.0.as_tl()))
//     }
// }
//
// impl From<ed25519::PublicKey> for NodeIdFull {
//     fn from(key: ed25519::PublicKey) -> Self {
//         Self::new(key)
//     }
// }
//
