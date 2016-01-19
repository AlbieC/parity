use hash::*;
use sha3::*;
use hashdb::*;
use rlp::*;
use super::triedbmut::*;
use super::trietraits::*;

/// A mutable `Trie` implementation which hashes keys and uses a generic `HashDB` backing database.
/// 
/// Use it as a `Trie` or `TrieMut` trait object. You can use `raw()` to get the backing TrieDBMut object.
pub struct SecTrieDBMut<'db> {
	raw: TrieDBMut<'db>
}

impl<'db> SecTrieDBMut<'db> {
	/// Create a new trie with the backing database `db` and empty `root`
	/// Initialise to the state entailed by the genesis block.
	/// This guarantees the trie is built correctly.
	pub fn new(db: &'db mut HashDB, root: &'db mut H256) -> Self { 
		SecTrieDBMut { raw: TrieDBMut::new(db, root) }
	}

	/// Create a new trie with the backing database `db` and `root`
	/// Panics, if `root` does not exist
	pub fn from_existing(db: &'db mut HashDB, root: &'db mut H256) -> Self {
		SecTrieDBMut { raw: TrieDBMut::from_existing(db, root) }
	}
}

impl<'db> Trie for SecTrieDBMut<'db> {
	fn root(&self) -> &H256 { self.raw.root() }

	fn contains(&self, key: &[u8]) -> bool {
		self.raw.contains(&key.sha3())
	}

	fn get<'a, 'key>(&'a self, key: &'key [u8]) -> Option<&'a [u8]> where 'a: 'key {
		self.raw.get(&key.sha3())
	}
}

impl<'db> TrieMut for SecTrieDBMut<'db> {
	fn insert(&mut self, key: &[u8], value: &[u8]) {
		self.raw.insert(&key.sha3(), value);
	}

	fn remove(&mut self, key: &[u8]) {
		self.raw.remove(&key.sha3());
	}
}

#[test]
fn sectrie_to_trie() {
	use memorydb::*;
	use super::triedb::*;

	let mut memdb = MemoryDB::new();
	let mut root = H256::new();
	{
		let mut t = SecTrieDBMut::new(&mut memdb, &mut root);
		t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]);
	}
	let t = TrieDB::new(&memdb, &root);
	assert_eq!(t.get(&(&[0x01u8, 0x23]).sha3()).unwrap(), &[0x01u8, 0x23]);
}