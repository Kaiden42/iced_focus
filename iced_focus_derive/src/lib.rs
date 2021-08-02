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
        syn::Data::Enum(ref _e) => unimplemented!(),
        syn::Data::Union(ref _u) => unimplemented!(),
    }
}

fn impl_focus_struct(ident: &syn::Ident, s: &syn::DataStruct) -> TokenStream {
    //println!("struct: {:#?}", s);

    let fields = match s.fields {
        syn::Fields::Named(ref named) => FocusField::collect_fields_named(named),
        syn::Fields::Unnamed(_) => unimplemented!(),
        syn::Fields::Unit => unimplemented!(),
    };

    //println!("fields: {:#?}", fields);

    impl_focus_trait_for(ident, &fields)
}

fn impl_focus_trait_for<'a>(ident: &syn::Ident, fields: &[FocusField<'a>]) -> TokenStream {
    let vector_name = quote! {fields};
    let capacity = fields.len();

    let field_to_vector: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|field| {
            let ident = field.ident.unwrap(); // TODO
            quote! {
                #vector_name.push(&mut self.#ident);
            }
        })
        .collect();

    let field_idents: Vec<&syn::Ident> = fields
        .iter()
        .map(|field| field.ident.unwrap()) // TODO
        .collect();

    let result = quote! {
        impl iced_focus::Focus for #ident {
            fn focus(&mut self, direction: iced_focus::Direction) -> iced_focus::State {
                let mut #vector_name: std::vec::Vec<&mut dyn iced_focus::Focus> = std::vec::Vec::with_capacity(#capacity);

                #(#field_to_vector)*

                #vector_name.focus(direction)
            }

            fn has_focus(&self) -> bool {
                #(self.#field_idents.has_focus() ||)* false
            }
        }
    };
    result.into()
}

#[derive(Debug)]
struct FocusField<'a> {
    ident: Option<&'a syn::Ident>,
    attribute: FocusAttribute<'a>,
}

impl<'a> FocusField<'a> {
    fn collect_fields_named(fields_named: &'a syn::FieldsNamed) -> Vec<Self> {
        //println!("fields named: {:#?}", fields_named);

        fields_named
            .named
            .iter()
            .map(|f| FocusField::from_field_if_annotated(f))
            .flatten()
            .collect()
    }

    fn from_field_if_annotated(field: &'a syn::Field) -> Option<Self> {
        let attribute = FocusAttribute::extract_focus_attribute(&field.attrs);

        attribute.map(|attribute| Self {
            ident: field.ident.as_ref(),
            attribute,
        })
    }
}

#[derive(Debug)]
enum FocusAttribute<'a> {
    Enable(&'a syn::Ident),
    EnableWith(&'a syn::Ident, &'a str),
    Disable(&'a syn::Ident),
}

impl<'a> FocusAttribute<'a> {
    fn extract_focus_attribute(attrs: &'a [syn::Attribute]) -> Option<Self> {
        let attr: Option<(&syn::PathSegment, &proc_macro2::TokenStream)> = attrs
            .iter()
            .map(|attr| (&attr.path, &attr.tokens))
            .map(|(path, tokens)| (&path.segments, tokens))
            .flat_map(|(path, tokens)| {
                path.iter()
                    .find(|s| s.ident == "focus")
                    .map(|path| (path, tokens))
            })
            .next();

        attr.map(|(path, tokens)| {
            // TODO: other variants
            FocusAttribute::Enable(&path.ident)
        })
    }
}
