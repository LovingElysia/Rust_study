//struct数据类型
//1.struct的声明
struct User{
    name:String,
    age:i32,
    gender:String
}
fn main() {
    //2.实例化struct  字段初始化
    let user1=User
    {
        name:String::from("Elysia"),
        age:17,
        gender:String::from("female")
    };
    //3.访问struct的数据
    println!("user name is {},user age is {},user gender is {}",user1.name,user1.age,user1.gender);

    //4
    let user2:User = user_factory(String::from("kevin"), 18, String::from("male"));
    println!("user2 name is {},user2 age is {},user2 gender is {}",user2.name,user2.age,user2.gender);

    //struct 更新语法
    //当你想基于某个struct实例来创建一个新的struct实例时 可以使用struct更新语法
    let user3:User=User
    {
        name:String::from("Mei"),
        ..user2  //像这样 剩下的字段就会自动匹配user2中对应的字段
    };
    println!("user3 name is {},user3 age is {},user3 gender is {}",user3.name,user3.age,user3.gender);

    //tuple struct 
    //当你想给整个tuple起名，且想让它不同于其他tuple，而且又不需要给每个元素起名
    struct color(i32,i32,i32);
    struct point(i32,i32,i32);  
    let block = color(0,0,0);
    let origin = point(0,0,0); //block和point是两个完全不同的类型 是不同tuple struct的实例
}

//4.字段初始化简写
fn user_factory(name:String,age:i32,gender:String)->User//结构可以作为返回值返回
{    
    //我们发现形参作为字段值 且与User结构的字段同名
    //像这样 作为字段值的变量与作为字段的变量同名 可直接省略 :字段值  简写为以下形式
    User{
        name,
        age,
        gender
    }
}