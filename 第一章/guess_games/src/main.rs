use std::io;
//预导入模块 当你引用的模块不在当前模块的作用域，必须显示的导入
fn main()
{
    println!("猜数游戏");
    println!("猜一个数");
    
    /* 
    let foo =1;
    let num =2;
    foo = num;
    在rust中 默认变量的值是不可变的 如果你想要可变 必须声明为mut
    */
    //声明一个可变的字符串
    let mut guess = String::new();
    //stdin() 是一个标准输入函数，他会返回一个stdin的实例，该实例会被当做句柄
    //read_line是一个stdin的成员函数，read_line函数会把用户输入保存在一个字符串中
    //read_line参数一个可变字符串的引用，注意在rust中引用也是默认不变的 &为取地址符号
    //read_line会返回一个io::result 在rust中会有很多result，既有通用的，也有这种特定类型的
    //result 是一个枚举类型 而IO::result有两个枚举值 OK和err
    //在ok中会附带结果值 ，在err中会附带失败原因
    //expect是IO::result的成员函数 它会判断result的返回值 若为err，他会返回字符串信息
    //若为ok，则会将结果值返回给用户
    io::stdin().read_line(&mut guess).expect("该数无法读取");

    //{}是一个占位符
    println!("你猜测的数字为{}",guess);
}