pub trait CatalogueTrait<K, V> {
    fn add(&mut self, key: K, value: V);
    fn remove(&mut self, key: K, value: V);
    fn read(&self, key: K) -> Vec<V>;
}