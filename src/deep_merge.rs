/// Trait for all the types supporting deep merging.
pub trait DeepMerge {
	/// Merge `other` into `self`
	fn merge_from(&mut self, other: Self) where Self: Sized {
		*self = other;
	}
}

impl DeepMerge for bool {}
impl DeepMerge for i32 {}
impl DeepMerge for i64 {}
impl DeepMerge for f64 {}
impl DeepMerge for String {}
impl DeepMerge for crate::ByteString {}
impl<Tz> DeepMerge for chrono::DateTime<Tz> where Tz: chrono::TimeZone {}

impl DeepMerge for serde_json::Value {
	// JSON merge (RFC 7396).
	fn merge_from(&mut self, other: Self) where Self: Sized {
		if let serde_json::Value::Object(this) = self {
			if let serde_json::Value::Object(other) = other {
				for (k, v) in other {
					if v.is_null() {
						this.remove(&k);
					}
					else {
						this.entry(k).or_insert(serde_json::Value::Null).merge_from(v);
					}
				}

				return;
			}
		}

		*self = other;
	}
}

impl<T> DeepMerge for Box<T> where T: DeepMerge {
	fn merge_from(&mut self, other: Self) where Self: Sized {
		(**self).merge_from(*other)
	}
}

impl<K, V> DeepMerge for std::collections::BTreeMap<K, V> where K: Ord, V: DeepMerge {
	fn merge_from(&mut self, other: Self) {
		for (k, v) in other.into_iter() {
			match self.entry(k) {
				std::collections::btree_map::Entry::Vacant(e) => { e.insert(v); },
				std::collections::btree_map::Entry::Occupied(e) => e.into_mut().merge_from(v),
			}
		}
	}
}

impl<T> DeepMerge for Option<T> where T: DeepMerge {
	fn merge_from(&mut self, other: Self) {
		if let Some(other) = other {
			if let Some(s) = self {
				s.merge_from(other);
			} else {
				*self = Some(other);
			}
		}
	}
}

impl<T> DeepMerge for Vec<T> {
	fn merge_from(&mut self, mut other: Self) {
		self.append(&mut other);
	}
}
