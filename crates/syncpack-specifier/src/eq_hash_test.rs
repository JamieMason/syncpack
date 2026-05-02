use {
  crate::Specifier,
  std::{
    collections::{HashMap, HashSet},
    rc::Rc,
  },
};

#[test]
fn specifier_eq_by_raw_string() {
  let a = Specifier::new("^1.2.3");
  let b = Specifier::new("^1.2.3");
  assert_eq!(a, b);
  assert!(Rc::ptr_eq(&a, &b));
}

#[test]
fn specifier_neq_when_raw_differs() {
  let a = Specifier::new("^1.2.3");
  let b = Specifier::new("^1.2.4");
  assert_ne!(a, b);
}

#[test]
fn specifier_hash_consistent_with_eq() {
  let a = Specifier::new("^1.2.3");
  let b = Specifier::new("^1.2.3");
  let mut map: HashMap<Rc<Specifier>, &str> = HashMap::new();
  map.insert(a, "first");
  assert_eq!(map.get(&b), Some(&"first"));
}

#[test]
fn specifier_in_hashset() {
  let mut set: HashSet<Rc<Specifier>> = HashSet::new();
  set.insert(Specifier::new("1.0.0"));
  set.insert(Specifier::new("1.0.0"));
  set.insert(Specifier::new("2.0.0"));
  assert_eq!(set.len(), 2);
}

#[test]
fn specifier_none_hashes() {
  let a = Specifier::new("");
  let b = Specifier::new("");
  let mut set: HashSet<Rc<Specifier>> = HashSet::new();
  set.insert(a);
  assert!(set.contains(&b));
  assert!(matches!(*b, Specifier::None));
}
