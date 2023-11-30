#[derive(Debug)] //因为rust具有打印调试信息的功能，但是我们必须为我们的struct显示的选择这一功能
                 //derive可以是我们的struct派生于Debug trait 这样打印struct时就能使用调试格式
struct Circle
{
    pi:f64,
    r:f64
}

fn main() {
   let circle:Circle = Circle
   {
    pi:3.14,
    r:5.0
   };
   println!("this circle area is {}",area(&circle));
   //打印struct
   //println!("circle = {}",circle); //报错 因为circle没有默认的格式化方法 它需要我们为其实现一个display trait 编译告诉我们可以使用"{:?}或"{:#?}"来代替
   println!("circlr = {:?}",circle); //报错 编译告诉我们没有实现 debug trait 可以加上一个#[derive(Debug)]  添加以后边正常打印了
   println!("circle = {:#?}",circle); //使用{:#?}更加详细
}

fn area(circle:&Circle)->f64
{
    0.5*circle.pi*circle.r*circle.r
}
