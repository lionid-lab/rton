use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};

#[derive(Copy, Clone)]
pub struct PublicKey(CompressedEdwardsY, EdwardsPoint);

impl PublicKey {
    #[inline(always)]
    pub fn from_bytes(bytes: [u8; 32]) -> Option<Self> {
        let compressed = CompressedEdwardsY(bytes);
        let point = compressed.decompress()?;
        Some(PublicKey(compressed, -point))
    }

    #[inline(always)]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0 .0
    }

    #[inline(always)]
    pub fn as_bytes(&'_ self) -> &'_ [u8; 32] {
        &self.0 .0
    }
}
