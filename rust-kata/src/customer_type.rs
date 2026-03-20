#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CustomerType {
    Vip,
    Premium,
    Employee,
    Regular,
    New,
    Unknown,
}

impl CustomerType {
    pub(crate) fn from_str(s: &str) -> Self {
        match s.trim() {
            "vip" => Self::Vip,
            "premium" => Self::Premium,
            "employee" => Self::Employee,
            "regular" => Self::Regular,
            "new" => Self::New,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_known_variants() {
        assert_eq!(CustomerType::from_str("vip"), CustomerType::Vip);
        assert_eq!(CustomerType::from_str("premium"), CustomerType::Premium);
        assert_eq!(CustomerType::from_str("employee"), CustomerType::Employee);
        assert_eq!(CustomerType::from_str("regular"), CustomerType::Regular);
        assert_eq!(CustomerType::from_str("new"), CustomerType::New);
    }

    #[test]
    fn unknown_strings_map_to_unknown() {
        assert_eq!(CustomerType::from_str("corporate"), CustomerType::Unknown);
        assert_eq!(CustomerType::from_str("wholesale"), CustomerType::Unknown);
        assert_eq!(CustomerType::from_str(""), CustomerType::Unknown);
    }

    #[test]
    fn leading_and_trailing_whitespace_is_trimmed() {
        assert_eq!(CustomerType::from_str(" vip "), CustomerType::Vip);
        assert_eq!(CustomerType::from_str("  premium  "), CustomerType::Premium);
        assert_eq!(CustomerType::from_str("\tnew\t"), CustomerType::New);
    }

    #[test]
    fn case_is_not_normalised_uppercase_maps_to_unknown() {
        // safe() only trims whitespace; it does not lowercase the input,
        // so "VIP" and "Premium" must not accidentally match.
        assert_eq!(CustomerType::from_str("VIP"), CustomerType::Unknown);
        assert_eq!(CustomerType::from_str("Premium"), CustomerType::Unknown);
        assert_eq!(CustomerType::from_str("EMPLOYEE"), CustomerType::Unknown);
    }
}
