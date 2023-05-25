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

/*
    当别人写 #[derive(HelloMacro)] 时，
    hello_macro_derive 函数就会自动被调用

    为什么会自动调用呢？就是因为 hello_macro_derive 上边
    标注了 #[proc_macro_derive(HelloMacro)]，我们指明了 HelloMacro trait

    而 hello_macro_derive 函数，首先会把 input 转化解释成我们可操作的数据结构


 */

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 本函数用于解析 TokenStream
    // 这个函数签名对于你创建其他过程宏其实基本是一样的
    // 不同的是下边的 impl_hello_macro 

    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // DeriveInput 代表了解析后的 Rust 代码
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    // 负责转化语法树，即最后生成 Rust 代码的地方
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // Ident 结构体，这里有被标注类型的名称
    let name = &ast.ident;
    /*
        quote! 宏，定义返回的那些 Rust 代码
        就返回 impl 的代码

        quote! 宏的结果是编译器无法直接理解的类型，
        所以需要把结果转化为 TokenStream
        通过调用 into() 方法转化为 TokenStream

        #name 会被替换为变量 name 的值
     */
    /*
        stringify! 宏，它是 Rust 内置的，
        它接收一个表达式，例如 1+2，并把其抓为字符串字面值 ("1+2")
     */
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()

}