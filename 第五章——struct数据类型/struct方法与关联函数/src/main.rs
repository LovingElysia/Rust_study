struct Rectangle
{
    width:u32,
    length:u32
}

//struct方法
//使用impl关键字 与struct 名称  就能在对应的struct上下文中定义其方法
impl Rectangle{
    //对于一个struct方法 它的第一个参数必须时self 即自身的指针
    fn area(&self)->u32{    //这里既可以进行所有权转移 也能传一个借用  也可以是可变借用 不过原struct必须也是可变的
        self.width*self.length
    }

    //第一个参数也可以不是self 此时的函数便不再是方法 而是关联函数 它通常被用于构造器
    //构造一个正方形作为特殊长方形
    fn square(size:u32)->Rectangle
    {
        Rectangle { width: (size), length: (size) }
    }
}
fn is_square(rect:Rectangle)->bool{
    rect.width==rect.length
}
fn main() {
    let rect_1:Rectangle = Rectangle { width: (100), length: (200) };
    println!("rect_1 area is {}",rect_1.area()); //对于方法的调用，rust会自动解引用

    //关联函数的调用与方法不同 语法如下 object::func()
    let size:u32 = 8;
    let rect_2:Rectangle = Rectangle::square(size);
    if is_square(rect_2)
    {
        println!("是正方形")
    }
}
