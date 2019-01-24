const SIZE: usize = 3;

fn main() {
    let mut system: [[i32; 4]; 3] = [
        [1, 3, 1, 9],
        [1, 1, -1, 1],
        [3, 11, 5, 35]
    ] ;

    for i in 0..SIZE-1 {
        for j in i..SIZE-1 {
            let factor = system[j + 1][i] as f32 / system[i][i] as f32;
            if system[i][i] == 0 {
                println!("Skipped!");
                continue;
            } else {
                println!("Factor: {}", factor);
                for k in i..SIZE+1 {
                    system[j + 1][k] -= (factor * system[i][k] as f32) as i32;
                    println!("{:?}", system);
                }
            }
        }
    }
    println!("\nFinal system:\n{:?}", system);
}

