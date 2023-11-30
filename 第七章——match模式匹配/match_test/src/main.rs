
//match 模式匹配

//1. 对于枚举类型的匹配
enum City {
    beijing,
    shanghai,
    chengdu,
    chongqing
}

//2.绑定值的匹配
#[derive(Debug)]
enum Elysia {
    zhichun,
    zhiai,
    zhigao,
    zhishan,
    zhimei
}
#[derive(Debug)]
enum Hero{
    kevin,
    pardofelis,
    Mei,
    Elysia(Elysia)
}
fn main() {
    let city:City = City::shanghai;
    println!("{}",match_test(city));
    
    let elysia:Hero = Hero::Elysia(Elysia::zhichun);
    match_test_2(elysia);

    //在使用match时必须穷举所有可能
    let v = 0u8; //u8类型的范围是从0~256 所以他有256种可能
    match v{
        1=>println!("1"),
        3=>println!("3"),
        7=>println!("7"),
        _=>()   //当我们无法枚举出所有可能 或者不想枚举所有可能
    }           //就可以使用通配符‘_’ 表示我们不关心的枚举 并且返回一个空元组 当然这个必须写在match的最后

}

fn match_test(city:City)->u8
{
    match city{
        City::beijing => 1,
        City::shanghai => 2,
        City::chengdu => 3,
        City::chongqing =>4
    }
}


fn match_test_2(hero:Hero)
{
    match hero{
        Hero::pardofelis =>{println!("猫猫可爱")},
        Hero::kevin =>{println!("救世之铭")},
        Hero::Mei =>{println!("God")}
        //由于Elysia本身也是一个枚举类型 是可以存一个枚举值的 在这里我们也可以为她传入一个state 使这个state绑定到这个值上面
        Hero::Elysia(state) =>{println!("{:#?}",state)}
    }
}

