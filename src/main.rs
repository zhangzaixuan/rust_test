// main.rs idea中attch,todo 后续有时间看能不能用clion,有全家桶账户
mod example_test;

use example_test::user::User;
use example_test::while_test;
use example_test::nessting_loop;
use example_test::lifetime_test;
// tryinto trait的引入，类比oop中的抽象类和接口
use std::convert::TryInto;
use example_test::flow_control_test;
use crate::example_test::flow_control_test::is_even;

fn main() {
    /*let u1 = User::new_user(String::from("tom"), 5);
    println!("user name: {}", u1.name());
    println!("1+2: {}", example_test::user::add(1, 2));*/
    // idea/cargo 会自动补全，本来想测试后续声明类型；

    // 几种类型声明和类型推断
    /*let a = 10;
    let b = 20;
    let c = 30i32;
    let d = 30_i32;

    let e = add(add(a, b), add(c, d));
    // println!() 是一个宏，返回的是代码，而不是值。add是一个函数，该处宏的定义类比scala的柯里化（既可以传入函数，也可以返回函数）
    println!("(a+b)+(c+d)={}", e);*/
    // rust ""表示字符串，''表示单个字符。

    //     尽可能保证显式声明和转换数值类型，目前来看不能自动转换整数位长；
    /*let round_num = 6.18_f32.round();//round(6.18_f32) 不被接受  apply式方法声明
    println!("{}", round_num);
    let one_million: i64 = 1_000_000;//下划线只是为了可读性，emmm;
    println!("{}", one_million.pow(2));
    let thrity_six = [36.0, 36f32, 36.0_f32];
    println!("{:02}", thrity_six[0]);//小数位前保留，我有一个猜测
    println!("{:05}", thrity_six[0]);
    // 00000000000000000000000000000000000000000000000036
    println!("{:050}", thrity_six[0]);
    //                                                 36
    println!("{:50}", thrity_six[0]);*/

    /*let three = 0b11;
    let thrity = 0o36;
    let three_hundred = 0x12c;
    println!("base 10:{},{},{}", three, thrity, three_hundred);
    println!("bast 2:{:b} {:b} {:b}", three, thrity, three_hundred);
    println!("base 8:{:o},{:o},{:o}", three, thrity, three_hundred);
    println!("base16:{:x},{:x},{:x}", three, thrity, three_hundred);

    let five=0b101;
    let twenty_seven=0o33;
    let three_hundred_ninety_eight=0x18e;
    println!("base 10:{},{},{}", five, twenty_seven, three_hundred_ninety_eight);*/

    /* let a1: i32 = 10;
     let b1: u16 = 100;*/
    // mismatchtypes err
    /*if a1<b1{
        println!("a1<b1")
    }*/
    /*if a1<(b1 as i32){
        // u16 到i32的类型提升
        println!("a1<b1")
    }*/
    // exit code 101 浮点数使用还是慎重，这个获取就是decimal 类型的由来
    // assert!(0.1+0.2==0.3);

    /*let b1_ = b1.try_into().unwrap();
    // 此处如果类型转换异常，unwrap将会引发程序异常，这里估计要和go if err!=nil 异曲同工。
    // prelude 局部作用域隐式包含
    if a1 < b1_ {
        println!("a1<b1_")
    }

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc(f32)");
    println!("0.1+0.2:{:x}", (abc.0 + abc.1).to_bits());
    println!("0.3:{:x}", (abc.2).to_bits());
    println!();

    println!("xyz(f32)");
    println!("0.1+0.2:{:x}", (xyz.0 + xyz.1).to_bits());
    println!("0.3:{:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // why note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // assert!(xyz.0 + xyz.1 == xyz.2);

    let res: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_diff = (desired - res).abs();
    assert!(absolute_diff <= f32::EPSILON);

//     nan 特殊值，非数值，看样子也不能被定义成虚数根
//     let x=(-42.0_f32).sqrt();
    // assert_eq!(x,x)

    let x1: f32 = 1.0 / 0.0;
    // assert!(x1.is_finite());
    assert!(x1.is_infinite());
//     isnan 是不是非数值型，is_finite 是不是在有效值范围里，is_infinite 不在有效值范围里*/
    /*let collection=[1,2,3,4,5,6];
    let collection_1=[1,2,3,4,5,6,7];
    // mut mutable &mut 声明代表在使用过程中可修改集合子元素值，所以需要添加可变声明；
    let mut collection_2 =[1,2,3,4,5,6];*/

//     for item in collecton/for item in &collecton/for item in &mut collection
//     for item in intolterator::into_iter(collection) 拥有所有权，for item in collection.iter 只读/for item in collection.item_mut() 读写

    // 01 没有声明引用&，迭代容器container在方法作用域中默认使用后生命周期结束;02 声明&,可二次进入迭代container;03 如果中间要修改迭代元素，则需要使用mut关键字

    /*for item in collection{

        println!("{}",item);

    }

    for item in &collection_1{

        println!("{}",item)
    }

    for item in &mut collection_2{
        println!("{}", item);
    }

    for i in  0..collection_2.len(){
        // rust不建议像java,python通过下标取元素，但是能用
        println!("get i :{}", collection_2[i]);
        collection_2[i]+=1;
    }

    for item in &mut collection_2{
        println!("change ori-collection :{}", item);
    }

    for n in 0..10{

        if n%2==0{
            continue;
        }
        println!("{}",n)
    }*/

    /*    let mut samples=vec![];
        while samples.len()<10 {
            let sample=take_sample();
            if is_outlier(sample){

                continue;
            }
            samples.push(sample);
        }*/

    // while_test::while_count();

    // while true 死循环，这个都一样
    // loop 长时循环执行，break或者terminate终止；

    // zip  拉链操作
    /*for (x, y) in (0..).zip(0..) {
        println!("{},{}", x, y);
        if x + y > 100 {
            println!("{},{}", x, y);
            break;
        }
    }*/

    // function `crate::nessting_loop::nessting_loop` exists but is inaccessible,private public 概念，必须显式指定，默认函数私有；

    // nessting_loop::nessting_loop();
    // nessting_loop::zip_nessting_loop();


    // lifetime_test::lifetime_test_fn();
    // flow_control_test::fc_if_else(1000);

    // let even_resut=flow_control_test::is_even(8);
    let even_resut=flow_control_test::is_even(7);

    println!("{:?}",even_resut)


}


fn add(i: i32, j: i32) -> i32 {
    i + j
}
