const SIZE: usize = 4;

fn main() {
    let mut system = [
        [1f32, 1f32, 1f32, 1f32],
        [2f32, 3f32, 4f32, 5f32],
        [5f32 ,6f32, 7f32, 8f32],
        [8f32 ,9f32, 10f32, 11f32]
    ];

    // make echelon form
    for i in 0..SIZE-1 {
        for j in i..SIZE-1 {
        let factor = system[j + 1][i] as f32 / system[i][i] as f32;
        println!("Factor: {}", factor);
            for k in i..SIZE {
                system[j + 1][k] -= factor * system[i][k];
                println!("{:?}", system);
            }
        }
    }
    println!("\nFinal system:\n{:?}", system);
}
