//Rng是一个triat  是一个接口类型
 //cargo.lock 文件在会保留第一次构建项目时所使用的依赖
use rand::Rng;
//Ordering 是一个枚举类型 它有3个枚举值 Less Greater Equal 分别表示小于大于等于
use std::cmp::Ordering;
fn main()
{
    println!("猜数游戏");
    println!("请猜测一个数");
    //当不显示的声明数值类型时 一般默认该变量为i32类型(保证该变量存储的值范围不超过i32)
    let scret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();

    std::io::stdin().read_line(&mut guess).expect("无法读取该数");
    //:类型显示声明
    //rust允许同名变量重复声明，新声明的变量会隐藏之前的同名变量
    //这种一般被用在类型转换的场景
    //trim是String类型中一个方法，他会去除转义字符 '\n'，并返回一个新的字符串对象
    //parse也是string类型的方法，他会把字符串解析为一个数值类型
    //parse返回值是一个result
    let guess:u32 = guess.trim().parse().expect("无法解析");
    //match关键字是rust中的类型匹配
    //cmp是数值类型的方法，它的形参为另一个数值的引用类型，它会比较两个值的大小并返回一个bool
    match guess.cmp(&scret_number) {
        Ordering::Less => println!("太小了"),
        Ordering::Greater=>print!("太大了"),
        Ordering::Equal=>println!("猜对了")
    }
    println!("神秘数字是{}",scret_number);
}