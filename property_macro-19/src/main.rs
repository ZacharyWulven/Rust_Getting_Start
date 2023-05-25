// 类型属性宏
/*
    route 是用于做路由的
    这里如果方法是 GET，就会走到下边的 index 函数中
    而 route 就是过程宏定义的
 */
#[route(GET, "/")]
fn index() {

}

/*
    这就是 route 宏的函数签名
    attr 就对应 route(GET, "/") 的 GET 和路径 "/"
    item 就对应函数体，这里就是 index 函数
 */
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

}

fn main() {
    println!("Hello, world!");
}

// 函数宏
/*
    定义一个解析 SQL 语句的宏
 */
let sql = sql!(SELECT * FROM posts WHERE id=1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {

}