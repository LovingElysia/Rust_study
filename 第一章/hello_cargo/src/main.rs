//使用cargo创建项目时
//1. toml文件，里面记录了rust的配置格式
//[package]
//name 表示项目名称
//version 项目版本
//edition rust版本

//[dependencies]  在这个下记录了该项目的依赖
//在rust中 库被称为creat
//cargo build会生成可执行文件 会在target debug下生成可执行文件
//使用cargo run会先进行项目编译，然后运行该项目
//使用cargo check会检查程序是否能通过编译 但不会生成可执行文件
//cargo build--release在编译代码时会对项目进行优化 会在target release下生成可执行文件
fn main() {
    println!("Hello, world!");
}
