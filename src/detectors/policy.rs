use std::path::Path;
use std::sync::{LazyLock, RwLock};

use crate::domain::config::PolicyConfig;

static POLICY: LazyLock<RwLock<PolicyConfig>> =
    LazyLock::new(|| RwLock::new(PolicyConfig::default()));

pub(crate) fn configure(policy: &PolicyConfig) {
    *POLICY.write().expect("policy lock poisoned") = policy.clone();
}

pub(crate) fn is_test_path(path: &Path) -> bool {
    let policy = POLICY.read().expect("policy lock poisoned");
    if !policy.skip_tests {
        return false;
    }

    path.components().any(|component| {
        let component = component.as_os_str().to_string_lossy();
        policy
            .test_path_markers
            .iter()
            .any(|marker| component == marker.as_str() || component.ends_with(marker))
    })
}

pub(crate) fn is_dto_template_or_config_struct(item: &syn::ItemStruct) -> bool {
    let policy = POLICY.read().expect("policy lock poisoned");
    policy.skip_template_structs && has_template_attr(item)
        || policy.skip_data_carrier_structs
            && is_data_carrier_name(&item.ident.to_string(), &policy)
}

fn has_template_attr(item: &syn::ItemStruct) -> bool {
    item.attrs.iter().any(|attr| {
        attr.path().is_ident("template")
            || attr.path().is_ident("Template")
            || attr.path().is_ident("derive")
                && attr
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::token::Comma>::parse_terminated,
                    )
                    .is_ok_and(|nested| nested.iter().any(|meta| meta.path().is_ident("Template")))
    })
}

fn is_data_carrier_name(name: &str, policy: &PolicyConfig) -> bool {
    policy
        .data_carrier_struct_suffixes
        .iter()
        .any(|suffix| name.ends_with(suffix))
}
