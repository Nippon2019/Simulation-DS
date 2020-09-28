use rand::Rng;
use std::num;

fn main() {

    // pi estimating
    let mut number = 0.0;
    let mut num = 0.0;
    let mut i = 0.0;
    for n in 1..1000001 {
        i+=1.0;
        let mut rng = rand::thread_rng();
        let n1 = rng.gen_range(0.0,1.0);
        let n2 = rng.gen_range(0.0,1.0);
        if n1*n1 + n2*n2 < 1.0{
            number += 1.0;
        }
        num += number;
        if n == 1 || n == 10 || n == 100 || n==1000 || n==10000 || n== 100000 || n== 1000000{
            println!("estimated pi value after {} sets of simulations: {}",n,num/(i)*4.0);
        }
        number = 0.0;
    }

    println!("..\n");

    // golden ratio estimating
    let mut rng = rand::thread_rng();
    let mut gr = rng.gen_range(1.0,2.0);
    for n in 1..1000001 {
        let mut rng = rand::thread_rng();
        let n1 = rng.gen_range(-0.1,0.1);
        let mut g1 = 0.0;
        let mut g2 = 0.0;
        if gr - (1.0 + 1.0/gr) >=0.0{
            g1 = gr - (1.0 + 1.0/gr);
        }else{
            g1 = (1.0 + 1.0/gr) - gr;
        }
        if (gr+n1) - (1.0 + 1.0/(gr+n1)) >= 0.0{
            g2 = (gr+n1) - (1.0 + 1.0/(gr+n1));
        }else{
            g2 =  (1.0 + 1.0/(gr+n1)) - (gr+n1);
        }
        if g1> g2{
            gr += n1;
            println!("estimated golden ratio after {} simulations: {}",n,gr);
        }
    }

    let pi = std::f32::consts::PI;
    let golden = ((5 as f64).sqrt() + 1.0 )/2.0;
    println!("..\n");
    println!("error of pi is {} %",((pi-(num/(i)*4.0))/pi*100.0).abs());
    println!("error of golden ratio is {} %",((golden-gr)/golden*100.0).abs());
}
