fn main()
{
    //在传统语言中，我们在将某个数据置空时 如
    //string a = null;
    //stirng b = "Elysia";
    //若把a这个null值当作非空值使用 例如：
    // a = a+b;
    //就有可能产生nullpointerexception的报错
    //rust 提供了OPtion<T>枚举来解决这样的问题
    let a:i32 = 128;
    let b:Option<i32> = None;
    //a= a+b; 而实际上option<i32>与i32并非同一类型 所以遇到这种情况必须强制转换才能相加 这样就避免了null值的使用
}