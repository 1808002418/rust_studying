mod macros;

/**

Rust 有两种类型的宏：
声明式宏（Declarative macros）使得你能够写出类似 match 表达式的东西，来操作你所提供的 Rust 代码。它使用你提供的代码来生成用于替换宏调用的代码。
过程宏（Procedural macros）允许你操作给定 Rust 代码的抽象语法树（abstract syntax tree, AST）。过程宏是从一个（或者两个）TokenStream到另一个TokenStream的函数，用输出的结果来替换宏调用

宏展开执行命令:
    要先安装: rustup default nightly
    rustc  -Zunpretty=expanded ./src/main.rs

宏通过使用macro_rules!来声明
    https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html
    item ——一个项（item），像一个函数，结构体，模块等。
    block ——一个块 （block）（即一个语句块或一个表达式，由花括号所包围）
    stmt —— 一个语句（statement）
    pat ——一个模式（pattern）
    expr —— 一个表达式（expression）
    ty ——一个类型（type）
    ident—— 一个标识符（indentfier）
    path —— 一个路径（path）（例如，foo，::std::mem::replace，transmute::<_, int>，...）
    meta —— 一个元数据项；位于#[...]和#![...]属性
    tt ——一个词法树
    vis ——一个可能为空的Visibility限定词

量词: ($($element:expr),*)
    ?：表示最多一次重复，所以此时不能前跟分隔标记。
    *：表示零次或多次重复。
    +：表示一次或多次重复。

 */

fn main() {
    println!("{}", add!(1,2,3,4));

}