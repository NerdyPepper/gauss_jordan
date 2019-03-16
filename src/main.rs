const SIZE: usize = 4;

// TODO write tests
fn main() {
    let mut system: [[f32; SIZE + 1]; SIZE] = [
        [1.0 , 2.0 , 1.0  , -1.0 , -2.0] ,
        [2.0 , 3.0 , -1.0 , 2.0  , 7.0]  ,
        [1.0 , 1.0 , 3.0  , -2.0 , -6.0] ,
        [1.0 , 1.0 , 1.0  , 1.0  , 2.0]
    ];

    println!("Original system: ");
    print_2d(system);

    for i in 0..SIZE-1 {
        for j in i..SIZE-1 {
            if system[i][i] == 0f32 {
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
        if system[i][i] == 0f32 {
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

    println!("\n{}", "-".repeat(20));

    for i in 0..SIZE {
        if system[i][i] == 0f32 {
            println!("Infnitely many solutions");
        }
        else {
            system[i][SIZE] /= system[i][i] as f32;
            system[i][i] = 1f32;
            println!("X{} = {}", i + 1, system[i][SIZE]);
        }
    }
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
