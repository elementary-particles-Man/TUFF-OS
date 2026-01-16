pub struct KeyManager;

impl KeyManager {
    pub fn new() -> Self { Self }
    pub fn load_key(&self, _data: &[u8]) -> bool { true }
}
