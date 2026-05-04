/// Skip consecutive ASCII digits from `pos`, returning position after last digit.
/// Returns None if no digits at `pos`.
fn skip_digits(bytes: &[u8], pos: usize) -> Option<usize> {
  let start = pos;
  let mut i = pos;
  while i < bytes.len() && bytes[i].is_ascii_digit() {
    i += 1;
  }
  if i == start { None } else { Some(i) }
}

/// Check if bytes from `pos` match a tag suffix: -[a-z0-9._-]+$
fn has_tag_suffix(bytes: &[u8], pos: usize) -> bool {
  pos < bytes.len()
    && bytes[pos] == b'-'
    && bytes.len() > pos + 1
    && bytes[pos + 1..]
      .iter()
      .all(|&b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'_'))
}

/// Check if bytes from `pos` match digits.digits.digits (optionally with -tag)
fn is_semver_triple(bytes: &[u8], pos: usize) -> bool {
  let Some(p1) = skip_digits(bytes, pos) else { return false };
  if p1 >= bytes.len() || bytes[p1] != b'.' {
    return false;
  }
  let Some(p2) = skip_digits(bytes, p1 + 1) else { return false };
  if p2 >= bytes.len() || bytes[p2] != b'.' {
    return false;
  }
  let Some(p3) = skip_digits(bytes, p2 + 1) else { return false };
  p3 == bytes.len() || has_tag_suffix(bytes, p3)
}

/// Check if bytes from `pos` match digits.digits$
fn is_minor_version(bytes: &[u8], pos: usize) -> bool {
  let Some(p1) = skip_digits(bytes, pos) else { return false };
  if p1 >= bytes.len() || bytes[p1] != b'.' {
    return false;
  }
  let Some(p2) = skip_digits(bytes, p1 + 1) else { return false };
  p2 == bytes.len()
}

/// Check if bytes from `pos` match digits$
fn is_major_version(bytes: &[u8], pos: usize) -> bool {
  let Some(p1) = skip_digits(bytes, pos) else { return false };
  p1 == bytes.len()
}

/// Get the byte offset after a range prefix (^, ~, >, >=, <, <=).
/// Returns None if no range prefix found.
fn range_prefix_len(bytes: &[u8]) -> Option<usize> {
  match bytes.first() {
    Some(b'^' | b'~') => Some(1),
    Some(b'>' | b'<') => {
      if bytes.get(1) == Some(&b'=') {
        Some(2)
      } else {
        Some(1)
      }
    }
    _ => None,
  }
}

pub fn is_simple_semver(str: &str) -> bool {
  is_exact(str) || is_latest(str) || is_major(str) || is_minor(str) || is_range(str) || is_range_major(str) || is_range_minor(str)
}

/// "1.2.3", "1.2.3-beta.1", "=1.2.3", "=1.2.3-beta.1"
pub fn is_exact(s: &str) -> bool {
  let bytes = s.as_bytes();
  let start = if bytes.first() == Some(&b'=') { 1 } else { 0 };
  is_semver_triple(bytes, start)
}

pub fn is_latest(str: &str) -> bool {
  str == "*" || str == "latest" || str == "x"
}

/// "1"
pub fn is_major(s: &str) -> bool {
  is_major_version(s.as_bytes(), 0)
}

/// "1.2"
pub fn is_minor(s: &str) -> bool {
  is_minor_version(s.as_bytes(), 0)
}

/// "^1.2.3", "~1.2.3", ">1.2.3", ">=1.2.3", etc. (with optional -tag)
pub fn is_range(s: &str) -> bool {
  let bytes = s.as_bytes();
  let Some(start) = range_prefix_len(bytes) else { return false };
  is_semver_triple(bytes, start)
}

/// "^1", "~1", ">1", ">=1", etc.
pub fn is_range_major(s: &str) -> bool {
  let bytes = s.as_bytes();
  let Some(start) = range_prefix_len(bytes) else { return false };
  is_major_version(bytes, start)
}

/// "^1.2", "~1.2", ">1.2", ">=1.2", etc.
pub fn is_range_minor(s: &str) -> bool {
  let bytes = s.as_bytes();
  let Some(start) = range_prefix_len(bytes) else { return false };
  is_minor_version(bytes, start)
}

/// Is this a semver range containing multiple parts?
/// Such as OR (" || ") or AND (" ") or hyphen range (" - ")
pub fn is_complex_range(specifier: &str) -> bool {
  // Quick reject: must contain a space
  if !specifier.contains(' ') {
    return false;
  }
  // Split on " || " first (OR conditions), then each part is space-separated AND conditions
  let or_parts = specifier.split(" || ");
  let mut total_parts = 0;
  for or_part in or_parts {
    let tokens: Vec<_> = or_part.split([' ', '-']).map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    total_parts += tokens.len();
    if !tokens.iter().all(|s| is_simple_semver(s)) {
      return false;
    }
  }
  total_parts > 1
}

/// "alpha", "beta", etc.
pub fn is_tag(s: &str) -> bool {
  !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphabetic())
}

/// git+ssh://, git://, github:, git@, git+https://
pub fn is_git(s: &str) -> bool {
  s.starts_with("git://")
    || s.starts_with("github:")
    || s.starts_with("git@")
    || s.starts_with("git+https://")
    || s.starts_with("git+ssh://")
}

/// "link:../path"
pub fn is_link(s: &str) -> bool {
  s.starts_with("link:")
}
