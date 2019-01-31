const SIZE: usize = 3;

fn main() {
    let mut system: [[f32; SIZE + 1]; SIZE] = [
        // A , A   , A    , B
        [1.0 , 3.0 , 4.0  , 3.0]  ,
        [2.0 , 7.0 , 3.0 , -7.0]  ,
        [2.0 , 8.0 , 6.0  , -4.0]
    ] ;

    println!("Original system: ");
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

    println!("\n{}", "-".repeat(20));
    println!("Echelon form:");
    print_2d(system);

    for i in (1..SIZE).rev() {
        if system[i][i] == 0.0 {
            continue;
        } else {
            for j in (1..i+1).rev() {
                let factor = system[j - 1][i] as f32 / system[i][i] as f32;
                for k in (0..SIZE+1).rev() {
                    system[j - 1][k] -= factor * system[i][k] as f32;
                }
            }
        }
    }

    println!("\n{}", "-".repeat(20));
    println!("Gaussian eliminated:");
    print_2d(system);
}

fn print_2d( system: [[f32; SIZE + 1]; SIZE] ) {
    println!();
    for i in 0..SIZE {
        for j in 0..SIZE+1 {
            if j == SIZE {
                print!("| ");
            }
            print!("{:02} ", system[i][j]);
        }
        println!();
    }
}
