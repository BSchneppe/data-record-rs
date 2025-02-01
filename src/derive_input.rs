use darling::{Error, FromDeriveInput};
use proc_macro2::Ident;
use std::convert::TryFrom;
use syn::spanned::Spanned;
use syn::{parse_quote_spanned, Attribute, Meta};

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(datarecord),
    forward_attrs(
        cfg,
        allow,
        datarecord_getter_attr,
        datarecord_getter_impl_attr,
        datarecord_const_attr,
        datarecord_const_impl_attr,
        datarecord_const_impl_method_attr
    )
)]
pub struct DataRecordOpts {
    pub ident: Ident,
    pub name: String,
    #[darling(default = "default_constructor_name")]
    pub constructor_name: String,
    #[darling(default)]
    pub impl_const: bool,
    #[darling(default)]
    pub impl_getter: bool,
    #[darling(with = TryFrom::try_from)]
    pub attrs: ForwardedAttrs,
}
#[derive(Debug, Clone, Default)]
pub struct ForwardedAttrs {
    pub getter_attrs: Vec<Attribute>,
    pub getter_impl_attrs: Vec<Attribute>,
    pub const_attrs: Vec<Attribute>,
    pub const_impl_attrs: Vec<Attribute>,
    pub const_impl_method_attrs: Vec<Attribute>,
}

impl TryFrom<Vec<Attribute>> for ForwardedAttrs {
    type Error = Error;
    fn try_from(value: Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut result = Self::default();
        distribute_and_unnest_attrs(
            value,
            &mut [
                ("datarecord_getter_attr", &mut result.getter_attrs),
                ("datarecord_getter_impl_attr", &mut result.getter_impl_attrs),
                ("datarecord_const_attr", &mut result.const_attrs),
                ("datarecord_const_impl_attr", &mut result.const_impl_attrs),
                (
                    "datarecord_const_impl_method_attr",
                    &mut result.const_impl_method_attrs,
                ),
            ],
        )?;
        Ok(result)
    }
}

fn default_constructor_name() -> String {
    "new".to_string()
}

// Everything below is copied from https://github.com/colin-kiegel/rust-derive-builder

/// Divide a list of attributes into multiple partially-overlapping output lists.
///
/// Some attributes from the macro input will be added to the output in multiple places;
/// for example, a `cfg` attribute must be replicated to both the struct and its impl block or
/// the resulting code will not compile.
///
/// Other attributes are scoped to a specific output by their path, e.g. `builder_field_attr`.
/// These attributes will only appear in one output list, but need that outer path removed.
///
/// For performance reasons, we want to do this in one pass through the list instead of
/// first distributing and then iterating through each of the output lists.
///
/// Each item in `outputs` contains the attribute name unique to that output, and the `Vec` where all attributes for that output should be inserted.
/// Attributes whose path matches any value in `outputs` will be added only to the first matching one, and will be "unnested".
/// Other attributes are not unnested, and simply copied for each decoratee.
fn distribute_and_unnest_attrs(
    mut input: Vec<Attribute>,
    outputs: &mut [(&'static str, &mut Vec<Attribute>)],
) -> darling::Result<()> {
    let mut errors = vec![];

    for (name, list) in &*outputs {
        assert!(list.is_empty(), "Output Vec for '{}' was not empty", name);
    }

    for attr in input.drain(..) {
        let destination = outputs
            .iter_mut()
            .find(|(ptattr, _)| attr.path().is_ident(ptattr));

        if let Some((_, destination)) = destination {
            match unnest_from_one_attribute(attr) {
                Ok(n) => destination.push(n),
                Err(e) => errors.push(e),
            }
        } else {
            for (_, output) in outputs.iter_mut() {
                output.push(attr.clone());
            }
        }
    }

    if !errors.is_empty() {
        return Err(darling::Error::multiple(errors));
    }

    Ok(())
}

fn unnest_from_one_attribute(attr: syn::Attribute) -> darling::Result<Attribute> {
    match &attr.style {
        syn::AttrStyle::Outer => (),
        syn::AttrStyle::Inner(bang) => {
            return Err(darling::Error::unsupported_format(&format!(
                "{} must be an outer attribute",
                attr.path()
                    .get_ident()
                    .map(Ident::to_string)
                    .unwrap_or_else(|| "Attribute".to_string())
            ))
            .with_span(bang));
        }
    };

    let original_span = attr.span();

    let pound = attr.pound_token;
    let meta = attr.meta;

    match meta {
        Meta::Path(_) => Err(Error::unsupported_format("word").with_span(&meta)),
        Meta::NameValue(_) => Err(Error::unsupported_format("name-value").with_span(&meta)),
        Meta::List(list) => {
            let inner = list.tokens;
            Ok(parse_quote_spanned!(original_span=> #pound [ #inner ]))
        }
    }
}
