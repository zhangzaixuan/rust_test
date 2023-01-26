pub fn nessting_loop(){

    'outer: for x in 0..{

        for y in 0..{

            for z in 0..{

                if x+y+z>1000{
                    println!("{},{},{}",x,y,z);
                    break 'outer;
                }

            }
        }

    }



}


pub fn zip_nessting_loop(){

    'outer: for ((x,y),z) in (0..).zip(0..).zip(0..){
                if x+y+z>1000{
                    println!("{},{},{}",x,y,z);
                    break 'outer;
                }
            }

}