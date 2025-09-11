#[cfg(test)]
#[path = "format_test.rs"]
mod format_test;

use {
  crate::{package_json::PackageJson, rcfile::Rcfile},
  regex::Regex,
  serde_json::{self, Map, Value},
  std::{cmp::Ordering, collections::HashSet},
};

/// Use a shorthand format for the bugs URL when possible
pub fn get_formatted_bugs(package: &PackageJson) -> Option<Value> {
  package.get_prop("/bugs/url")
}

/// Use a shorthand format for the repository URL when possible
pub fn get_formatted_repository(package: &PackageJson) -> Option<Value> {
  if !package.has_prop("/repository/directory") {
    package.get_prop("/repository/url").and_then(|url| {
      if let Value::String(url) = url {
        Regex::new(r#".+github\.com/"#)
          .ok()
          .map(|re| re.replace(url.as_str(), "").to_string())
          .map(Value::String)
      } else {
        None
      }
    })
  } else {
    None
  }
}

/// Get sorted conditional exports and conditional exports subpaths
pub fn get_sorted_exports(rcfile: &Rcfile, package: &PackageJson) -> Option<Value> {
  /// Recursively visit and sort nested objects of the exports object
  fn sort_nested_objects(sort_exports: &Vec<String>, value: &mut Value) {
    if let Value::Object(obj) = value {
      sort_keys_with_priority(sort_exports, false, obj);
      // Ensure that the key "default", if present, is always last.
      if let Some(default) = obj.remove("default") {
        obj.insert("default".to_string(), default);
      }
      for next_value in obj.values_mut() {
        sort_nested_objects(sort_exports, next_value);
      }
    }
  }
  let contents = package.contents.borrow();
  if let Some(exports) = contents.pointer("/exports") {
    let mut sorted_exports = exports.clone();
    sort_nested_objects(&rcfile.sort_exports, &mut sorted_exports);
    if !is_identical(exports, &sorted_exports) {
      std::mem::drop(contents);
      return Some(sorted_exports);
    }
  }
  std::mem::drop(contents);
  None
}

/// Get a sorted version of the given property from package.json
pub fn get_sorted_az(key: &str, package: &PackageJson) -> Option<Value> {
  let contents = package.contents.borrow();
  if let Some(value) = contents.pointer(format!("/{key}").as_str()) {
    let mut sorted = value.clone();
    sort_alphabetically(&mut sorted);
    if !is_identical(value, &sorted) {
      std::mem::drop(contents);
      return Some(sorted);
    }
  }
  std::mem::drop(contents);
  None
}

/// Get a new package.json with its keys sorted to match the rcfile
pub fn get_sorted_first(rcfile: &Rcfile, package: &PackageJson) -> Option<Value> {
  let contents = package.contents.borrow();
  if let Value::Object(value) = &*contents {
    let mut sorted = value.clone();
    sort_keys_with_priority(&rcfile.sort_first, rcfile.sort_packages, &mut sorted);
    if !has_same_key_order(value, &sorted) {
      std::mem::drop(contents);
      return Some(serde_json::Value::Object(sorted));
    }
  }
  std::mem::drop(contents);
  None
}

/// Do both of these objects have the same order keys?
fn has_same_key_order(a: &Map<String, Value>, b: &Map<String, Value>) -> bool {
  let a_keys = a.keys().collect::<Vec<_>>();
  let b_keys = b.keys().collect::<Vec<_>>();
  a_keys == b_keys
}

/// Are these two values identical including their order?
#[allow(clippy::cmp_owned)]
fn is_identical(a: &Value, b: &Value) -> bool {
  // @TODO: serde_json with feature = "preserve_order" ignores order when compared
  a.to_string() == b.to_string()
}

/// Sort the keys in a JSON object, with the given keys first
///
/// # Parameters
///
/// * `order`: The keys to sort first, in order.
/// * `obj`: The JSON object to sort.
/// * `sort_remaining_keys`: Whether to sort the remaining keys alphabetically.
fn sort_keys_with_priority(order: &[String], sort_remaining_keys: bool, obj: &mut Map<String, Value>) {
  let order_set: HashSet<_> = order.iter().collect();
  let mut sorted_obj: Map<String, Value> = Map::new();
  let mut remaining_keys: Vec<_> = obj.keys().filter(|k| !order_set.contains(*k)).cloned().collect();

  if sort_remaining_keys {
    let collator = get_locale_collator();
    remaining_keys.sort_by(|a, b| collator(a, b));
  }

  for key in order {
    if let Some(val) = obj.remove(key) {
      sorted_obj.insert(key.clone(), val);
    }
  }

  for key in remaining_keys {
    if let Some(val) = obj.remove(&key) {
      sorted_obj.insert(key, val);
    }
  }

  *obj = sorted_obj;
}

/// Sort an array or object alphabetically by EN locale
fn sort_alphabetically(value: &mut Value) {
  let collator = get_locale_collator();
  match value {
    Value::Object(obj) => {
      let mut entries: Vec<_> = obj.clone().into_iter().collect();
      entries.sort_by(|a, b| collator(&a.0, &b.0));
      *value = Value::Object(Map::from_iter(entries));
    }
    Value::Array(arr) => {
      arr.sort_by(|a, b| {
        if let (Some(a), Some(b)) = (a.as_str(), b.as_str()) {
          collator(a, b)
        } else {
          Ordering::Equal
        }
      });
    }
    _ => {}
  }
}

/// Get a collator that mimics JavaScript's localeCompare behavior
/// Expected order: symbols (@), then numbers (1), then letters (A)
fn get_locale_collator() -> impl Fn(&str, &str) -> Ordering {
  |a: &str, b: &str| {
    // Extract the first character to determine sorting priority
    let a_first = a.chars().next().unwrap_or('\0');
    let b_first = b.chars().next().unwrap_or('\0');

    // Classify characters into three categories
    let get_priority = |c: char| -> u8 {
      if c.is_ascii_alphabetic() {
        2 // Letters last
      } else if c.is_ascii_digit() {
        1 // Numbers in middle
      } else {
        0 // Symbols first (@, etc.)
      }
    };

    let a_priority = get_priority(a_first);
    let b_priority = get_priority(b_first);

    match a_priority.cmp(&b_priority) {
      Ordering::Equal => {
        // Same category, use case-insensitive comparison
        a.to_lowercase().cmp(&b.to_lowercase())
      }
      other => other,
    }
  }
}
