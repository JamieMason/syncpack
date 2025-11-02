use {crate::specifier::Specifier, std::rc::Rc};

#[derive(Debug, PartialEq)]
pub struct ComplexSemver {
  /// "1.3.0 || <1.0.0 >2.0.0"
  /// "<1.0.0 >2.0.0"
  /// "<1.0.0 >=2.0.0"
  /// "<1.5.0 || >=1.6.0"
  /// "<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2"
  /// "<=1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2"
  /// ">1.0.0 <1.0.0"
  /// ">1.0.0 <=2.0.0"
  /// ">=2.3.4 || <=1.2.3"
  pub raw: String,
  /// Used when checking if specifiers satisfy each other
  pub node_range: Rc<node_semver::Range>,
}

impl ComplexSemver {
  pub fn create(raw: &str) -> Specifier {
    match Specifier::new_node_range(raw) {
      Some(node_range) => Specifier::ComplexSemver(Self {
        raw: raw.to_string(),
        node_range,
      }),
      None => Specifier::Unsupported(raw.to_string()),
    }
  }
}
