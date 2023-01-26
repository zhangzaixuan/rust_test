pub fn lifetime_test_fn() {
    let name = "ruixing_coffee";
    const price: f64 = 41.00;
    let goods = Goods { name: name, price: &price };
    println!("{:?}",goods)
}

/*
tips:
  当引用类型用于另一个引用类型声明时，必须标注引用的生命周期；
    -- 01 goods 声明要通过，可以为Goods中的所有引用赋静态生命周期，比较粗暴，不建议；
    -- 02 为结构体提供一个生命周期
 */
// debug implement `Debug` 需要实现debug接口。


// #[derive(Debug)]
// struct Goods {
//     name: &'static str,
//     price: &'static f64,
// }


// reference url  https://mp.weixin.qq.com/s/jjy0oIHgAwd_tQbJdugjqQ


#[derive(Debug)]
struct Goods<'a> {
    // 结构体泛型上加生命周期'a,那结构体每个引用值上都使用泛型指定的生命周期；
    name: &'a str,
    price: &'a f64,
}