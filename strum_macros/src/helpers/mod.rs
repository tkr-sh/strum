pub use self::case_style::snakify;
pub use self::inner_variant_props::HasInnerVariantProperties;
pub use self::type_props::HasTypeProperties;
pub use self::variant_props::HasStrumVariantProperties;

// This basically means:
// ```
// [convert_case] | [convert_case, heck] => "convert_case_style.rs",
// [heck] => "convert_case_style.rs",
// ````
#[cfg_attr(
    all(feature = "heck", not(feature = "convert_case")),
    path = "heck_case_style.rs"
)]
#[cfg_attr(feature = "convert_case", path = "convert_case_style.rs")]
pub mod case_style;

pub mod inner_variant_props;
mod metadata;
pub mod type_props;
pub mod variant_props;

use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;

pub fn missing_parse_err_attr_error() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "`parse_err_ty` and `parse_err_fn` attributes are both required.",
    )
}

pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

pub fn non_unit_variant_error() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "This macro only supports enums of strictly unit variants. Consider \
        using it in conjunction with [`EnumDiscriminants`]",
    )
}

pub fn non_single_field_variant_error(attr: &str) -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        format_args!(
            "The [`{}`] attribute only supports enum variants with a single field",
            attr
        ),
    )
}

pub fn strum_discriminants_passthrough_error(span: &impl Spanned) -> syn::Error {
    syn::Error::new(
        span.span(),
        "expected a pass-through attribute, e.g. #[strum_discriminants(serde(rename = \"var0\"))]",
    )
}

pub fn occurrence_error<T: ToTokens>(fst: T, snd: T, attr: &str) -> syn::Error {
    let mut e = syn::Error::new_spanned(
        snd,
        format!("Found multiple occurrences of strum({})", attr),
    );
    e.combine(syn::Error::new_spanned(fst, "first one here"));
    e
}
