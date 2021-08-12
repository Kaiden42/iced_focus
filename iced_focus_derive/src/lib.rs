//! A proc-macro to derive a focus chain for Iced applications
//! Take a look at the readme for more informations.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,

    // Restriction lints
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::exit,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::let_underscore_must_use,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::todo,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
)]
#![allow(
    clippy::suboptimal_flops,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions,
    clippy::missing_panics_doc
)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// The starting point of the procedural macro.
#[proc_macro_derive(Focus, attributes(focus))]
pub fn focus_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_focus(&ast)
}

/// Implement the `Focus` trait for the given AST.
fn impl_focus(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;

    match ast.data {
        syn::Data::Struct(ref s) => impl_focus_struct(ident, generics, s),
        syn::Data::Enum(ref e) => impl_focus_enum(ident, generics, e),
        syn::Data::Union(ref _u) => unimplemented!("Unions are currently not supported."),
    }
}

/// Implement the `Focus` trait for a struct.
fn impl_focus_struct(
    ident: &syn::Ident,
    generics: &syn::Generics,
    s: &syn::DataStruct,
) -> TokenStream {
    let (fields, len) = match s.fields {
        syn::Fields::Named(ref named) => {
            (FocusField::collect_fields_named(named), named.named.len())
        }
        syn::Fields::Unnamed(ref unnamed) => (
            FocusField::collect_fields_unnamed(unnamed),
            unnamed.unnamed.len(),
        ),
        syn::Fields::Unit => unimplemented!("Unit structs are currently not supported."),
    };

    build_focus_trait_for_struct(ident, generics, &fields, len)
}

/// Build the token stream of the trait implementation for a struct.
fn build_focus_trait_for_struct<'a>(
    ident: &syn::Ident,
    generics: &syn::Generics,
    fields: &[FocusField<'a>],
    len: usize,
) -> TokenStream {
    let array_name = quote! {fields};
    let focus_method_body = build_focus_method_body(0, &array_name, fields, len, true);
    let has_focus_method_body = build_has_focus_method_body(fields, true);

    let generic_idents = generic_idents(generics);

    let result = quote! {
        impl#generics iced_focus::Focus for #ident#generic_idents {
            fn focus(&mut self, direction: iced_focus::Direction) -> iced_focus::State {
                #focus_method_body
            }

            fn has_focus(&self) -> bool {
                #has_focus_method_body
            }
        }
    };
    result.into()
}

/// Implement the `Focus` trait for an enum.
fn impl_focus_enum(ident: &syn::Ident, generics: &syn::Generics, e: &syn::DataEnum) -> TokenStream {
    let variants = &e.variants;

    let method_bodies: Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)> = variants
        .iter()
        .enumerate()
        .map(|(index, variant)| impl_focus_enum_variant(index, variant))
        .collect();

    let focus_bodies = method_bodies.iter().map(|(focus, _)| focus);
    let has_focus_bodies = method_bodies.iter().map(|(_, has_focus)| has_focus);

    // TODO: clean this up.
    let booleans: Vec<proc_macro2::TokenStream> = variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let fields = match variant.fields {
                syn::Fields::Named(ref named) => FocusField::collect_fields_named(named),
                syn::Fields::Unnamed(ref unnamed) => FocusField::collect_fields_unnamed(unnamed),
                syn::Fields::Unit => Vec::new(),
            };

            let booleans = fields
                .iter()
                .map(|field| (field.index, &field.attribute))
                .filter_map(|(field_index, attribute)| match attribute {
                    FocusAttribute::Enable(_) => None,
                    FocusAttribute::EnableWith(_, _) => {
                        Some(attribute.to_boolean_expression(field_index, Some(index)))
                    }
                });

            quote! {
                #(#booleans)*
            }
        })
        .collect();

    let generic_idents = generic_idents(generics);

    let result = quote! {
        impl#generics iced_focus::Focus for #ident#generic_idents {
            fn focus(&mut self, direction: iced_focus::Direction) -> iced_focus::State {
                #(#booleans)*

                match self {
                    #(#focus_bodies)*
                }
            }

            fn has_focus(&self) -> bool {
                match self {
                    #(#has_focus_bodies)*
                }
            }
        }
    };
    result.into()
}

/// Implement the `Focus` trait for a variant of an enum.
fn impl_focus_enum_variant(
    index: usize,
    variant: &syn::Variant,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ident = &variant.ident;
    let array_name = quote! {fields};

    let (fields, len) = match variant.fields {
        syn::Fields::Named(ref named) => {
            (FocusField::collect_fields_named(named), named.named.len())
        }
        syn::Fields::Unnamed(ref unnamed) => (
            FocusField::collect_fields_unnamed(unnamed),
            unnamed.unnamed.len(),
        ),
        syn::Fields::Unit => (Vec::new(), 0),
    };

    let field_idents: Vec<proc_macro2::TokenStream> =
        fields.iter().map(|field| field.ident(false)).collect();
    let focus_method_body = build_focus_method_body(index, &array_name, &fields, len, false);
    let has_focus_method_body = build_has_focus_method_body(&fields, false);

    let variant_fields = match variant.fields {
        syn::Fields::Named(_) => quote! { {#(#field_idents,)* ..} },
        syn::Fields::Unnamed(ref unnamed) => {
            let idents = (0..unnamed.unnamed.len())
                .into_iter()
                .map(|i| syn::Ident::new(&format!("t_{}", i), proc_macro2::Span::call_site()));
            quote! { (#(#idents,)*) }
        }
        syn::Fields::Unit => quote! {},
    };

    let focus_method_body = quote! {
        Self::#ident #variant_fields => {
            #focus_method_body
        }
    };

    let has_focus_method_body = quote! {
        Self::#ident #variant_fields => {
            #has_focus_method_body
        }
    };

    (focus_method_body, has_focus_method_body)
}

/// Build the `focus(&mut self, iced_focus::Direction) -> iced_focus::State` method of the `Focus` trait.
fn build_focus_method_body<'a>(
    index: usize,
    array_name: &proc_macro2::TokenStream,
    fields: &[FocusField<'a>],
    len: usize,
    with_self: bool,
) -> proc_macro2::TokenStream {
    //let _capacity = fields.len();
    let field_to_vector: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            if with_self {
                field.add_struct_field_to_array(array_name)
            } else {
                field.add_enum_field_to_array(index, array_name)
            }
        })
        .collect();
    let booleans: Vec<proc_macro2::TokenStream> = if with_self {
        fields
            .iter()
            .map(|field| field.attribute.to_boolean_expression(field.index, None))
            .collect()
    } else {
        Vec::new()
    };

    let array_init = std::iter::repeat(quote! { None }).take(len);

    quote! {
        let mut #array_name: [Option<&mut dyn iced_focus::Focus>; #len] = [#(#array_init,)*];

        #(#booleans)*

        #(#field_to_vector)*

        #array_name.focus(direction)
    }
}

/// Build the `has_focus(&self) -> bool` method of the `Focus` trait.
fn build_has_focus_method_body(
    fields: &[FocusField<'_>],
    with_self: bool,
) -> proc_macro2::TokenStream {
    let field_idents: Vec<proc_macro2::TokenStream> =
        fields.iter().map(|field| field.ident(with_self)).collect();

    let booleans: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| match field.attribute {
            FocusAttribute::Enable(_) => quote! {},
            FocusAttribute::EnableWith(_, ref path) => quote! {#path() &&},
        })
        .collect();

    let with_self = if with_self {
        quote! {self.}
    } else {
        quote! {}
    };

    quote! {
        #(#booleans #with_self#field_idents.has_focus() ||)* false
    }
}

/// Extracts the idents of the annotated generics.
fn generic_idents(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let generics: Vec<proc_macro2::TokenStream> = generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Type(ty) => {
                let ident = &ty.ident;
                quote! { #ident }
            }
            syn::GenericParam::Lifetime(lifetime) => {
                let lifetime = &lifetime.lifetime;
                quote! { #lifetime }
            }
            syn::GenericParam::Const(con) => {
                let ident = &con.ident;
                quote! { #ident }
            }
        })
        .collect();

    if generics.is_empty() {
        quote! {}
    } else {
        quote! { <#(#generics,)*> }
    }
}

/// Represents a field annotated with `focus(enable...)`.
#[derive(Debug)]
struct FocusField<'a> {
    /// The ident of the field.
    ident: proc_macro2::TokenStream,
    /// The index of the field in the struct/enum.
    index: usize,
    /// If the field is unnamed.
    unnamed: bool,
    /// The annotated focus attribute of the field.
    attribute: FocusAttribute<'a>,
}

impl<'a> FocusField<'a> {
    /// Collect all fields annotated with `focus(enable...)` from a named fields list.
    fn collect_fields_named(fields_named: &'a syn::FieldsNamed) -> Vec<Self> {
        fields_named
            .named
            .iter()
            .enumerate()
            .filter_map(|(index, field)| FocusField::from_field_if_annotated(field, index))
            .collect()
    }

    /// Collect all fields annotated with `focus(enable...)` from an unnamed fields list.
    fn collect_fields_unnamed(fields_unnamed: &'a syn::FieldsUnnamed) -> Vec<Self> {
        fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .filter_map(|(index, field)| FocusField::from_field_if_annotated(field, index))
            .collect()
    }

    /// Returns a [`FocusField`](FocusField) representation of the given field if the field was annotated with `focus(enable...)`.
    fn from_field_if_annotated(field: &'a syn::Field, index: usize) -> Option<Self> {
        let attribute = FocusAttribute::extract_focus_attribute(&field.attrs);
        let index_literal = proc_macro2::Literal::usize_unsuffixed(index);

        attribute.map(|attribute| Self {
            ident: if let Some(ident) = field.ident.as_ref() {
                quote! {#ident}
            } else {
                quote! {#index_literal}
            },
            index,
            unnamed: field.ident.is_none(),
            attribute,
        })
    }

    /// TODO
    fn add_struct_field_to_array(
        &self,
        array_name: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let ident = self.ident(true);
        let index = self.index;
        match self.attribute {
            FocusAttribute::Enable(_) => quote! {
                #array_name[#index] = Some(&mut self.#ident);
            },
            FocusAttribute::EnableWith(_, _) => {
                let boolean =
                    syn::Ident::new(&format!("b_{}", self.index), proc_macro2::Span::call_site());
                quote! {
                    if #boolean {
                        #array_name[#index] = Some(&mut self.#ident);
                    }
                }
            }
        }
    }

    /// TODO
    fn add_enum_field_to_array(
        &self,
        index: usize,
        array_name: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let ident = self.ident(false);
        let field_index = self.index;

        match self.attribute {
            FocusAttribute::Enable(_) => quote! {
                #array_name[#field_index] = Some(#ident);
            },
            FocusAttribute::EnableWith(_, _) => {
                let boolean = syn::Ident::new(
                    &format!("b_{}_{}", index, self.index),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    if #boolean {
                        #array_name[#field_index] = Some(#ident);
                    }
                }
            }
        }
    }

    /// Return the ident of this field.
    fn ident(&self, with_self: bool) -> proc_macro2::TokenStream {
        if !with_self && self.unnamed {
            // TODO: clean up
            let tmp = syn::Ident::new(&format!("t_{}", self.ident), proc_macro2::Span::call_site());
            quote! {#tmp}
        } else {
            self.ident.clone()
        }
    }
}

/// The representation of the `focus(enable...)` attribute.
#[derive(Debug)]
enum FocusAttribute<'a> {
    /// The `focus(enable)` annotation.
    Enable(&'a syn::Ident),
    /// The `focus(enable = WITH)` annotation.
    EnableWith(&'a syn::Ident, proc_macro2::TokenStream),
    //Disable(&'a syn::Ident),
}

impl<'a> FocusAttribute<'a> {
    /// Extract the [`FocusAttribute`](FocusAttribute) from the given slice of attributes if present.
    fn extract_focus_attribute(attrs: &'a [syn::Attribute]) -> Option<Self> {
        let attr: Option<(&syn::PathSegment, syn::MetaList)> = attrs
            .iter()
            .filter_map(|attr| {
                match attr.parse_meta() {
                    Ok(syn::Meta::List(meta)) => Some((&attr.path.segments, meta)),
                    Ok(_) | Err(_) => None,
                }
            })
            .find_map(|(path, meta)| {
                path.iter()
                    .find(|s| s.ident == "focus")
                    .map(|path| (path, meta))
            });

        attr.map(|(path, mut meta)| {
            if meta.nested.len() != 1 {
                panic!("Expected the focus attribute to be not empty.");
            }

            match meta.nested.pop().unwrap().into_value() {
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    let _ = nv
                        .path
                        .get_ident()
                        .filter(|ident| *ident == "enable")
                        .expect("Expected the ident `enable` inside the focus attribute.");

                    match nv.lit {
                        syn::Lit::Str(s) => {
                            let p: proc_macro2::TokenStream = syn::parse_str(&s.value()).unwrap();

                            FocusAttribute::EnableWith(&path.ident, p)
                        }
                        _ => panic!(
                            "Expected the path of `focus(enable = PATH) to be a `str` literal."
                        ),
                    }
                }
                syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
                    let _ = p
                        .get_ident()
                        .filter(|ident| *ident == "enable")
                        .expect("Expected the ident `enable` inside the focus attribute.");

                    FocusAttribute::Enable(&path.ident)
                }
                _ => panic!(
                    "The nested meta of the focus attribute must be `enable` or `enable = PATH`."
                ),
            }
        })
    }

    /// Builds the boolean expression if this [`FocusAttribute`](FocusAttribute) contains a path to a boolean method.
    fn to_boolean_expression(
        &self,
        field_index: usize,
        variant_index: Option<usize>,
    ) -> proc_macro2::TokenStream {
        match self {
            FocusAttribute::Enable(_) => quote! {},
            FocusAttribute::EnableWith(_, ref path) => {
                let boolean = match variant_index {
                    Some(variant_index) => syn::Ident::new(
                        &format!("b_{}_{}", variant_index, field_index),
                        proc_macro2::Span::call_site(),
                    ),
                    None => syn::Ident::new(
                        &format!("b_{}", field_index),
                        proc_macro2::Span::call_site(),
                    ),
                };
                quote! {
                    let #boolean = #path();
                }
            }
        }
    }
}
