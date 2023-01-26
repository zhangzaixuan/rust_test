use std::time::{Duration,Instant};

pub fn while_count(){

    let mut count=0;
    let time_limit=Duration::new(1,0);
    let start=Instant::now();

    // println!("{},{}",start.elapsed().as_nanos(),time_limit.as_nanos());
    println!("{},{}",start.elapsed().as_nanos(),time_limit.as_nanos());

    while (Instant::now()-start)<time_limit {

        count+=1;

    }

    println!("{}",count)

}