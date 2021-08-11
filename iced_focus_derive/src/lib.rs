extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Focus, attributes(focus))]
pub fn focus_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_focus(&ast)
}

fn impl_focus(ast: &syn::DeriveInput) -> TokenStream {
    //println!("ast: {:#?}", ast);

    let ident = &ast.ident;

    match ast.data {
        syn::Data::Struct(ref s) => impl_focus_struct(ident, s),
        syn::Data::Enum(ref e) => impl_focus_enum(ident, e),
        syn::Data::Union(ref _u) => unimplemented!("Unions are currently not supported."),
    }
}

fn impl_focus_struct(ident: &syn::Ident, s: &syn::DataStruct) -> TokenStream {
    //println!("struct: {:#?}", s);

    let fields = match s.fields {
        syn::Fields::Named(ref named) => FocusField::collect_fields_named(named),
        //syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields are currently not supported."),
        syn::Fields::Unnamed(ref unnamed) => FocusField::collect_fields_unnamed(unnamed),
        syn::Fields::Unit => unimplemented!("Unit structs are currently not supported."),
    };

    //println!("fields: {:#?}", fields);

    build_focus_trait_for_struct(ident, &fields)
}

fn build_focus_trait_for_struct<'a>(ident: &syn::Ident, fields: &[FocusField<'a>]) -> TokenStream {
    let vector_name = quote! {fields};
    let focus_method_body = build_focus_method_body(0, &vector_name, fields, true);
    let has_focus_method_body = build_has_focus_method_body(fields, true);

    let result = quote! {
        impl iced_focus::Focus for #ident {
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

fn impl_focus_enum(ident: &syn::Ident, e: &syn::DataEnum) -> TokenStream {
    //println!("enum: {:#?}", e);

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
                .map(|(field_index, attribute)| match attribute {
                    FocusAttribute::Enable(_) => None,
                    FocusAttribute::EnableWith(_, _) => {
                        Some(attribute.to_boolean_expression(field_index, Some(index)))
                    }
                })
                .flatten();

            quote! {
                #(#booleans)*
            }
        })
        .collect();

    let result = quote! {
        impl iced_focus::Focus for #ident {
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

fn impl_focus_enum_variant(
    index: usize,
    variant: &syn::Variant,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ident = &variant.ident;
    let vector_name = quote! {fields};

    let fields = match variant.fields {
        syn::Fields::Named(ref named) => FocusField::collect_fields_named(named),
        syn::Fields::Unnamed(ref unnamed) => FocusField::collect_fields_unnamed(unnamed),
        syn::Fields::Unit => Vec::new(),
    };

    let field_idents: Vec<proc_macro2::TokenStream> =
        fields.iter().map(|field| field.ident(false)).collect();
    let focus_method_body = build_focus_method_body(index, &vector_name, &fields, false);
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

fn build_focus_method_body<'a>(
    index: usize,
    vector_name: &proc_macro2::TokenStream,
    fields: &[FocusField<'a>],
    with_self: bool,
) -> proc_macro2::TokenStream {
    let capacity = fields.len();
    let field_to_vector: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            if with_self {
                field.add_struct_field_to_vec(vector_name)
            } else {
                field.add_enum_field_to_vec(index, vector_name)
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

    quote! {
        let mut #vector_name: std::vec::Vec<&mut dyn iced_focus::Focus> = std::vec::Vec::with_capacity(#capacity);
        let mut #vector_name: std::vec::Vec<Box<&mut dyn iced_focus::Focus>> = std::vec::Vec::with_capacity(#capacity);

        #(#booleans)*

        #(#field_to_vector)*

        #vector_name.focus(direction)
    }
}

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

#[derive(Debug)]
struct FocusField<'a> {
    ident: proc_macro2::TokenStream,
    index: usize,
    unnamed: bool,
    attribute: FocusAttribute<'a>,
}

impl<'a> FocusField<'a> {
    fn collect_fields_named(fields_named: &'a syn::FieldsNamed) -> Vec<Self> {
        fields_named
            .named
            .iter()
            .enumerate()
            .map(|(index, field)| FocusField::from_field_if_annotated(field, index))
            .flatten()
            .collect()
    }

    fn collect_fields_unnamed(fields_unnamed: &'a syn::FieldsUnnamed) -> Vec<Self> {
        fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| FocusField::from_field_if_annotated(field, index))
            .flatten()
            .collect()
    }

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

    fn add_struct_field_to_vec(
        &self,
        vector_name: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let ident = self.ident(true);
        match self.attribute {
            FocusAttribute::Enable(_) => quote! {
                #vector_name.push(Box::new(&mut self.#ident));
            },
            FocusAttribute::EnableWith(_, _) => {
                let boolean =
                    syn::Ident::new(&format!("b_{}", self.index), proc_macro2::Span::call_site());
                quote! {
                    if #boolean {
                        #vector_name.push(Box::new(&mut self.#ident));
                    }
                }
            }
        }
    }

    fn add_enum_field_to_vec(
        &self,
        index: usize,
        vector_name: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let ident = self.ident(false);
        //quote! {
        //    #vector_name.push(#ident);
        //}
        match self.attribute {
            FocusAttribute::Enable(_) => quote! {
                #vector_name.push(Box::new(#ident));
            },
            FocusAttribute::EnableWith(_, _) => {
                let boolean = syn::Ident::new(
                    &format!("b_{}_{}", index, self.index),
                    proc_macro2::Span::call_site(),
                );
                quote! {
                    if #boolean {
                        #vector_name.push(Box::new(#ident));
                    }
                }
            }
        }
    }

    fn ident(&self, with_self: bool) -> proc_macro2::TokenStream {
        if !with_self && self.unnamed {
            // TODO: clean up
            let tmp = syn::Ident::new(&format!("t_{}", self.ident), proc_macro2::Span::call_site());
            quote! {#tmp}
        } else {
            self.ident.to_owned()
        }
    }
}

#[derive(Debug)]
enum FocusAttribute<'a> {
    Enable(&'a syn::Ident),
    EnableWith(&'a syn::Ident, proc_macro2::TokenStream),
    //Disable(&'a syn::Ident),
}

impl<'a> FocusAttribute<'a> {
    fn extract_focus_attribute(attrs: &'a [syn::Attribute]) -> Option<Self> {
        let attr: Option<(&syn::PathSegment, syn::MetaList)> = attrs
            .iter()
            .map(|attr| {
                let meta = match attr.parse_meta() {
                    Ok(syn::Meta::List(meta)) => meta,
                    Ok(_) | Err(_) => panic!(
                        "Expected a meta list like `focus(enable ...)` for the focus attribute."
                    ),
                };

                (&attr.path, meta)
            })
            .map(|(path, meta)| (&path.segments, meta))
            .flat_map(|(path, meta)| {
                path.iter()
                    .find(|s| s.ident == "focus")
                    .map(|path| (path, meta))
            })
            .next();

        attr.map(|(path, mut meta)| {
            if meta.nested.len() != 1 {
                panic!("Expected the focus attribute to be not empty.");
            }

            match meta.nested.pop().unwrap().into_value() {
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    nv.path
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
                    p.get_ident()
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
