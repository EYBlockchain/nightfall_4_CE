/// Trait for any structure that can provide a key. Agnostic to
/// how the key is stored.
pub trait KeyProvider<K> {
    fn get_key(key_id: &str) -> Option<K>;
    fn set_key(key_id: &str, key: K) -> Result<(), Box<dyn std::error::Error>>;
}
