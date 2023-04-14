use proc_macro::TokenStream;
use quote::quote;
use venial::{parse_declaration, Declaration};

#[proc_macro_derive(ObjectPartialEq)]
pub fn object_partial_eq_drive(input: TokenStream) -> TokenStream {
    if let Ok(Declaration::Struct(target)) = parse_declaration(input.into()) {
        let target_name = target.name;
        quote! {
            impl ObjectPartialEq for #target_name {
                fn eq(&self, other: Rc<dyn Object>) -> bool {
                    if self.class() == other.class() {
                        let other = &other as &dyn std::any::Any;
                        if let Some(other) = other.downcast_ref::<Self>() {
                            PartialEq::eq(self, other)
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            }
        }
        .into()
    } else {
        quote! {
            compile_error!("Must be a struct");
        }
        .into()
    }
}

#[proc_macro_derive(ObjectPartialOrd)]
pub fn object_partial_ord_derive(input: TokenStream) -> TokenStream {
    if let Ok(Declaration::Struct(target)) = parse_declaration(input.into()) {
        let target_name = target.name;
        quote! {
            impl ObjectPartialOrd for #target_name {
                fn partial_cmp(&self, other: Rc<dyn Object>) -> Option<std::cmp::Ordering> {
                    if self.class() == other.class() {
                        let other = &other as &dyn std::any::Any;
                        if let Some(other) = other.downcast_ref::<Self>() {
                            PartialOrd::partial_cmp(self, other)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }
        }
        .into()
    } else {
        quote! {
            compile_error!("Must be a struct");
        }
        .into()
    }
}

#[proc_macro_derive(ObjectEq)]
pub fn object_eq_derive(input: TokenStream) -> TokenStream {
    if let Ok(Declaration::Struct(target)) = parse_declaration(input.into()) {
        let target_name = target.name;
        quote! {
            impl ObjectEq for #target_name { }
        }
        .into()
    } else {
        quote! {
            compile_error!("Must be a struct");
        }
        .into()
    }
}

#[proc_macro_derive(ObjectOrd)]
pub fn object_ord_derive(input: TokenStream) -> TokenStream {
    if let Ok(Declaration::Struct(target)) = parse_declaration(input.into()) {
        let target_name = target.name;
        quote! {
            impl ObjectOrd for #target_name {
                fn cmp(&self, other: Rc<dyn Object>) -> std::cmp::Ordering {
                    ObjectPartialOrd::partial_cmp(self, other).unwrap()
                }
            }
        }
        .into()
    } else {
        quote! {
            compile_error!("Must be a struct");
        }
        .into()
    }
}
