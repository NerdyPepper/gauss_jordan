const SIZE: usize = 3;

fn main() {
    let mut system: [[f32; SIZE + 1]; SIZE] = [
        // A , A    , A    , B
        [1.0 , 3.0  , 1.0  , 9.0]  ,
        [1.0 , 1.0  , -1.0 , 1.0]  ,
        [3.0 , 11.0 , 5.0  , 35.0]
    ] ;

    print_2d(system);
    for i in 0..SIZE-1 {
        for j in i..SIZE-1 {
            if system[i][i] == 0.0 {
                continue;
            } else {
                let factor = system[j + 1][i] as f32 / system[i][i] as f32;
                for k in i..SIZE+1 {
                    system[j + 1][k] -= factor * system[i][k] as f32;
                }
            }
        }
    }

    print_2d(system);

    for i in (1..SIZE).rev() {
        for j in (i..SIZE).rev() {
            if system[i][i] == 0.0 {
                continue;
            } else {
                let factor = system[j - 1][i] as f32 / system[i][i] as f32;
                println!("Factor: {}", factor);
                for k in (i..SIZE+1).rev() {
                    system[j - 1][k] -= factor * system[i][k] as f32;
                    print_2d(system);
                }
            }
        }
    }

    print_2d(system);
}

fn print_2d( system: [[f32; SIZE + 1]; SIZE] ) {
    println!();
    for i in 0..SIZE {
        for j in 0..SIZE+1 {
            print!("{:02} ", system[i][j]);
        }
        println!();
    }
}
