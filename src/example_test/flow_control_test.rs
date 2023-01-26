pub fn fc_if_else(item: i32) {

    // rust中没有0/1数值这种替代真假的逻辑，条件判断就是true:false;
    if item == 1 {
        println!("{:?}", "cashback five dollars")
    } else if item == 2 {
        println!("{:?}", "cashback ten dollars")
    } else {
        println!("{:?}", "cashback one hundred dollars")
    }
}

pub fn is_even(n:i32)->bool{

    n%2==0
}