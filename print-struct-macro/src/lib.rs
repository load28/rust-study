extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrintStruct)]
pub fn print_struct_macro(input: TokenStream) -> TokenStream {
    // 입력 토큰을 파싱
    let input = parse_macro_input!(input as DeriveInput);
    
    // 구조체 이름 가져오기
    let name = &input.ident;
    
    // 구조체의 필드 정보 추출
    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) = input.data {
        // Collect fields into a Vec to extend their lifetime
        fields.named.iter().map(|f| {
            let field_name = &f.ident;
            let field_type = &f.ty;
            quote! {
                println!("필드 이름: {:?}, 타입: {}", stringify!(#field_name), stringify!(#field_type));
            }
        }).collect::<Vec<_>>()
    } else {
        panic!("PrintStruct는 named fields를 가진 구조체에만 사용할 수 있습니다");
    };

    // 구현 생성
    let expanded = quote! {
        impl #name {
            fn print_struct_info() {
                println!("구조체 이름: {}", stringify!(#name));
                println!("구조체의 필드 정보:");
                #(#fields)*
            }
        }
    };

    TokenStream::from(expanded)
}
