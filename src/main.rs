const SIZE: usize = 3;

fn main() {
    let mut system: [[i32; SIZE + 1]; SIZE] = [
        // A , A  , A  , B
        [1   , 3  , 1  , 9]  ,
        [1   , 1  , -1 , 1]  ,
        [3   , 11 , 5  , 35]
    ] ;

    print_2d(system);
    for i in 0..SIZE-1 {
        for j in i..SIZE-1 {
            let factor = system[j + 1][i] as f32 / system[i][i] as f32;
            if system[i][i] == 0 {
                continue;
            } else {
                for k in i..SIZE+1 {
                    system[j + 1][k] -= (factor * system[i][k] as f32) as i32;
                }
            }
        }
    }
    print_2d(system);
}

fn print_2d( system: [[i32; SIZE + 1]; SIZE] ) {
    println!();
    for i in 0..3 {
        for j in 0..4 {
            print!("{:02} ", system[i][j]);
        }
        println!();
    }
}
