fn main() {
    println!("Hello, world!");
}

//如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期。
fn longest_1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

//当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
//如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。
fn longest_2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
//最好的解决方案是返回一个有所有权的数据类型而不是一个引用，这样函数调用者就需要负责清理这个值了。

//生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
//一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作
//并阻止会产生悬垂指针亦或是违反内存安全的行为。