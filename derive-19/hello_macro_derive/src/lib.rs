extern crate proc_macro;

// 
/*
    借助 proc_macro 提供的编译器接口从而在代码中读取和操作 Rust 代码
    而由于它已经被内置在 Rust 里了，所以不需要添加它到 Cargo.toml 中的依赖
 */
use crate::proc_macro::TokenStream;

/*
    将 syn 产生的数据结构重新转化为 Rust 代码
 */
use quote::quote;

/*
    syn 用于把 Rust 代码从字符串转化为可供我们操作的数据结构
 */
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

}