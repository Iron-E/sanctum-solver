use std::{
	cmp::Ord,
	collections::{BTreeMap, HashMap, HashSet},
	hash::Hash,
};

pub trait Container<T>: Send + Sync
{
	/// # Summary
	///
	/// Return `true` if `some` value is contained in this [`Container`].
	fn contains(&self, some: &T) -> bool;
}

impl<K, V> Container<K> for BTreeMap<K, V>
where
	K: Hash + Ord + Send + Sync,
	V: Send + Sync,
{
	fn contains(&self, some: &K) -> bool
	{
		self.contains_key(some)
	}
}

impl<K, V> Container<K> for HashMap<K, V>
where
	K: Eq + Hash + Send + Sync,
	V: Send + Sync,
{
	fn contains(&self, some: &K) -> bool
	{
		self.contains_key(some)
	}
}

impl<T> Container<T> for HashSet<T>
where
	T: Eq + Hash + Send + Sync,
{
	fn contains(&self, some: &T) -> bool
	{
		HashSet::contains(self, some)
	}
}
