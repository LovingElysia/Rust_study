use std::string;


fn main()
{
    //所有权  heap  stack  MOVE  copy_trait drop_trait
    //copy_trait
    let x:i32 = 5;   //像整数这种标量类型，它们的长度从一开始就被确定 它们被简单的存放在栈上 并且实现了copy_trait
    let y:i32 = x;   //被实现了copy_trait 的数据类型 像这样进行赋值或传参操作的时候 会自动进行拷贝
    println!("x={}",x);  

    //drop_tarit
    let mut s1:String = String::from("hello"); //"hello"是字符串字面量，被存放在栈上，因为它的长度在编译时就被确定，并且不会改变的
    s1.push_str("，world"); //对于String类型,它会被动态的分配内存用于存放程序运行时产生的未知的文本大小
                                    //对于这种在编译期无法确定长度，并且需要动态分配内存的复杂数据类型，必须存放在堆上

    //堆上的数据会返回一个指针，访问堆上的数据是根据指针引导进行间接访问
    //对于一个string 类型的构成  name(存放value)  ptr(指向堆区的指针) len(元素长度) capacity(堆实际分配的内存大小)
    //对于堆区的内存往往需要手动释放 而rust为String类型实现了 drop_tarit,当该数据类型的实例对象离开其有效作用域时
    //便会自动释放内存
    //例1:
    {
        let s2:String = String::from("hello");
    }
    //println!("{}",s2) 报错，因为s2已离开作用域被释放

    //我们知道 浅拷贝问题 假如当s2只把指针复制给某个s3变量 而不把堆区数据复制给s3，而s2随机释放，若s3后续离开作用
    //域也被释放，机会导致同一块内存被重复释放
    //对于rust 每种数据类型 只能实现copy_trait 与 drop_trait其中一种 那么String该如何避免浅拷贝问题？

    //MOVE 与 所有权
    let s3:String = String::from("hello");
    let s4:String = s3;  //当实现drop_trait的数据类型进行传递时 会把指向堆区的指针进行copy,并且使原来的指针失效 
   // println!("{}",s3)  //你可能认为这依然是一个浅拷贝 但它会使原来的指针失效，实际上它应该被称之为MOVE(移动)
                       //例如像这样把s3复制给s4,s3同样把堆区的所有权交给s4 ,而我们再次试图通过s3访问堆区时，就会报错

    //函数间的所有权移动       
    let s5 = String::from("hello world");
    println!("{}",s5);  //还可以访问s5,所有权未转移
    let s6 = return_string(s5); //函数调用 s5的所有权首先被移动给形参str,随后通过返回值的形式，又将所有权返回给了s6
    //println!("{}",s5);  //此时s5已经已经失去了所有权，无法再被访问
    println!("{}",s6);

}
fn return_string(str:String)->String{
    str
}