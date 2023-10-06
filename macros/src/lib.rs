use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Component)]
pub fn component_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {
            // Gets the component type ID. This is used to uniquely identify a component type.
            fn get_type_id() -> usize {
                // The static TYPE_ID is initialized the first time get_type_id() is called for
                // this Component type. Following calls will return the same ID.
                // It uses the NEXT_TYPE_ID AtomicU32 to generate a unique
                // ID for the Component type and declared in ecs/components.rs.

                static TYPE_ID: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
                TYPE_ID.get_or_init(rust_ecs::ecs::component::get_next_component_type_id);
                *TYPE_ID.get().unwrap()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(BaseSystem)]
pub fn system_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl rust_ecs::ecs::system::BaseSystem for #name {
            // Gets the system type ID. This is used to uniquely identify a system type.
            fn get_type_id() -> usize {
                // The static TYPE_ID is initialized the first time get_type_id() is called for
                // this System type. Following calls will return the same ID.
                // It uses the NEXT_TYPE_ID AtomicU32 to generate a unique
                // ID for the Component type and declared in ecs/components.rs.
                static TYPE_ID: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
                TYPE_ID.get_or_init(rust_ecs::ecs::system::get_next_system_type_id);
                *TYPE_ID.get().unwrap()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}
