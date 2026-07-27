pub(crate) struct Namespace<'a> {
    pub kind: &'a str,
    pub name_field: &'a str,
    pub index_field: &'a str,
    pub lookup: &'a dyn Fn(&str) -> Option<u64>,
    pub check: Option<&'a dyn Fn(u64) -> Result<(), String>>,
}

pub(crate) fn resolve(
    owner: &str,
    namespace: &Namespace<'_>,
    name: Option<&str>,
    index: Option<u64>,
) -> Result<Option<u64>, String> {
    let resolved = match (name, index) {
        (Some(_), Some(_)) => {
            return Err(format!(
                "'{}' sets both '{}' and '{}'; use one or the other",
                owner, namespace.name_field, namespace.index_field
            ));
        }
        (Some(name), None) => Some((*namespace.lookup)(name).ok_or_else(|| {
            format!(
                "'{}' references {} '{}', which is not declared",
                owner, namespace.kind, name
            )
        })?),
        (None, index) => index,
    };
    if let (Some(resolved), Some(check)) = (resolved, namespace.check) {
        check(resolved).map_err(|reason| format!("'{owner}' {reason}"))?;
    }
    Ok(resolved)
}

pub(crate) fn require(
    owner: &str,
    namespace: &Namespace<'_>,
    name: Option<&str>,
    index: Option<u64>,
) -> Result<u64, String> {
    resolve(owner, namespace, name, index)?.ok_or_else(|| {
        format!(
            "'{}' needs a '{}' or '{}'",
            owner, namespace.name_field, namespace.index_field
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn namespace<'a>(
        lookup: &'a dyn Fn(&str) -> Option<u64>,
        check: Option<&'a dyn Fn(u64) -> Result<(), String>>,
    ) -> Namespace<'a> {
        Namespace {
            kind: "asset",
            name_field: "asset",
            index_field: "asset_id",
            lookup,
            check,
        }
    }

    #[test]
    fn resolves_a_name_to_its_index() {
        let lookup = |name: &str| (name == "Logo").then_some(3);
        let ns = namespace(&lookup, None);
        assert_eq!(resolve("Sprite", &ns, Some("Logo"), None), Ok(Some(3)));
    }

    #[test]
    fn passes_through_an_explicit_index() {
        let lookup = |_: &str| None;
        let ns = namespace(&lookup, None);
        assert_eq!(resolve("Sprite", &ns, None, Some(7)), Ok(Some(7)));
        assert_eq!(resolve("Sprite", &ns, None, None), Ok(None));
    }

    #[test]
    fn rejects_setting_both_forms() {
        let lookup = |_: &str| Some(0);
        let ns = namespace(&lookup, None);
        let error = resolve("Sprite", &ns, Some("Logo"), Some(1)).unwrap_err();
        assert!(
            error.contains("sets both 'asset' and 'asset_id'"),
            "{error}"
        );
    }

    #[test]
    fn rejects_an_unknown_name() {
        let lookup = |_: &str| None;
        let ns = namespace(&lookup, None);
        let error = resolve("Sprite", &ns, Some("Missing"), None).unwrap_err();
        assert!(error.contains("references asset 'Missing'"), "{error}");
    }

    #[test]
    fn applies_the_namespace_check_to_both_forms() {
        let lookup = |name: &str| (name == "Font").then_some(1);
        let check = |index: u64| {
            if index == 1 {
                Err("references a font_asset; it must be an image_asset".to_string())
            } else {
                Ok(())
            }
        };
        let ns = namespace(&lookup, Some(&check));
        let named = resolve("Sprite", &ns, Some("Font"), None).unwrap_err();
        let numeric = resolve("Sprite", &ns, None, Some(1)).unwrap_err();
        assert!(named.contains("must be an image_asset"), "{named}");
        assert_eq!(named, numeric);
    }
}
