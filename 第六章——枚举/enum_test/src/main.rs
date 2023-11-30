//enum 枚举类型

//1.定义一个枚举
enum IpAddress
{
    IPv4,
    Ipv6,
}
//2.将数据依附到枚举中
enum IP_ADDR
{
    IPV4(u8,u8,u8,u8),
    IPV6(String)
}

//3. 我们也可以让结构来存储数据 将其依附到枚举中
#[derive(Debug)]
struct IP_V4
{
    ip:(u8,u8,u8,u8)
}
#[derive(Debug)]
struct IP_V6
{
    ip:String
}
#[derive(Debug)]
enum IP_ADD
{
    Ip_V4(IP_V4),
    Ip_v6(IP_V6)
}

fn main()
{
    //实例化一个enum 枚举
    let ipv4:IpAddress = IpAddress::IPv4;
    let ipv6:IpAddress = IpAddress::Ipv6;
    //枚举作为函数调用的参数
    test(ipv4);
    test(ipv6);
    test(IpAddress::IPv4);
    test(IpAddress::Ipv6);  //这三种以枚举类型作为函数参数传递的方式都是被允许的

    //实例化带参数的的枚举
    let v4:IP_ADDR = IP_ADDR::IPV4(127,0,0,1);
    let v6:IP_ADDR = IP_ADDR::IPV6(String::from("::123"));

    //实例化例3的枚举
    let ip_v4:IP_ADD = IP_ADD::Ip_V4(IP_V4 { ip:(127,0,0,1) });
    println!{"{:#?}",ip_v4};

}

fn test(Ip:IpAddress){}
