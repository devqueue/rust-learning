fn is_even(x:u32) -> bool{
    x % 2 == 0
}


pub fn run(){
    let limit = 500;
    let mut sum = 0;

    for i in 0..{
        let isq = i*i;

        if isq > limit{
            break;
        }else if is_even(isq){
            sum += isq;
        }

    }
    println!("loop sum {}", sum);

    // All of this can be done in 1 line
    let sum2 = (0..).map(|x| x*x)
                    .take_while(|&x| x < limit)
                    .filter(|x| is_even(*x))
                    .fold(0, |sum, x| sum+x);
    
    println!("higher order function sum {}", sum2);
}