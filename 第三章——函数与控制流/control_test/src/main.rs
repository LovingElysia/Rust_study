fn main() {
   

    //循环控制语句
    //1.loop  loop循环是一个无限循环，直到我们手到停止它，相当于while true 可以用break 等跳出循环
    let mut number :i32=1;
    loop{
        number+=1;
        println!("{}",number);
        if number>5
        {
            break;
        }
    }
    //while 循环，同其他语言一样
    while number!=10
    {
        number+=1;
        println!("{}",number);
    }

    let arr:[i32;5]=[11,12,13,14,15];
    for it in arr.iter()
    {
        println!("{}",it);  //it是一个引用类型，使用for循环遍历集合不会将集合中的值赋值出来，而是直接使用引用
    }
    
}