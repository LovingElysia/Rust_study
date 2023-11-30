use core::num;

fn main()
{
    //引用 借用 以及引用的部分规则

    //引用 是一种指针 它指向某个已存在的值 但并不获取其所有权
    let s1:String = String::from("hello");
    let s2:&String = &s1;
    println!("{}",s1);//输出hello s1可正常访问堆区数据，那是因为s2获得是s1的引用， 只会获得这个堆区数据的使用权，而不会获得其所有权
    println!("{}",s2);//也可正常访问 ,实际上s2获得的是一个指向s1的指针,而s1的指针指向堆区数据

    //借用
    {
        let s1:String = String::from("Elysia");
        test(s1);
        //println!("S1={}",s1);  //报错 因为s1在进行函数调用时 将所有权传递给了形参s ,形参在打印值后便会释放这块内存
    }
    {
        let s1:String = String::from("kevin");
        test_1(&s1);           
        println!("s1={}",s1); //正常访问，因为s1只传递了自己的引用，而不会失去堆区数据的所有权
        //像这样 把数据以引用的方式传递出去 或作为函数的参数 使得某段时间某个变量拥有这个数据的使用权而无需获得使用权
        //就叫做借用
    }

    //可变引用
    {
        let mut s1 = String::from("Elysia");
        println!("s1 = {}",s1);  //输出Elysia
        test_2(&mut s1);
        println!("s1 = {}",s1);  //输出Elysia kiana
    }
    //不可变引用
    {
        let s1:String = String::from("kevin");
        let s2:&String = &s1;
        //s2.push_str("world");  //报错 引用默认是无法修改的 不可变引用无法对堆区数据进行修改
    }
    //可变引用和不可变引用在某个有效范围内无法同时存在
    {
        let mut s:String = String::from("Elysia");
        let immutable = &s;
        //let mutable= &mut s;  //报错 在任何给定作用域内 若同时出现一个可变和不可变引用 并且可变引用被使用 将会报错
        //println!("{}",immutable);  //这两段打印语句删除其中任何一个就不会报错
        //println!("{}",mutable);//因为 Rust 编译器在这种情况下采取了一种称为“非捕获”的优化措施。在这种情况下，编译器发现
                            //其中一个引用在作用域失效前未进行任何使用 编译器会自动删掉该引用
    }
    //只能存在一个可变引用
    {
        let mut s:String = String::from("kevin");
        let mut s1 = &mut s;
        let mut s2 = &mut s;  //报错 rust为了防止数据竞争 在任意作用域内只允许出现一个可变引用
        println!("{}",s1);
        println!("{}",s2);
    }
    //在任意作用域内  允许同时出现 多个不可变引用

    //悬空引用
    let s:&String = test_3();
    //像这样 在某个即将失效的作用域内创建数据并通过引用的方式返回
    //当s获得该引用时 原本的内存已经被释放 如果s再访问这块数据就会造成报错

}
fn test(s:String)
{
    println!("s={}",s);
}
fn test_1(s:&String)
{
    println!("s={}",s); //不会释放内存 因为没有这片内存的所有权
}
fn test_2(s:& mut String)
{
    s.push_str(" kiana");
}
fn test_3()->&String
{
    let s:String = String::from("kevin");
    return &s;
}