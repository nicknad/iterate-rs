use zeroize::Zeroizing;

pub struct ServiceKeys {
    pub content: Zeroizing<[u8; 32]>,
    pub meta: Zeroizing<[u8; 32]>,
}
