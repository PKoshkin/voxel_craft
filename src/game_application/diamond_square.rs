use game_application::rand::{Rng, thread_rng};

pub fn generate_hight_map(size: usize) -> Vec<Vec<usize>> {
    let mut rng = thread_rng();
    let mut result = Vec::with_capacity(size);
    for i in 0..size {
        result.push(Vec::with_capacity(size));
        for j in 0..size {
            result[i].push((size as f64) / 2.0);
        }
    }

    let mut square_size = size - 1;
    let roughness = 0.0012;
    while square_size > 1 {
        // square step
        let middle = square_size / 2;
        let mut x = 0;
        let mut y;
        while x + square_size < size {
            y = 0;
            while y + square_size < size {
                let random: f64 = rng.gen();
                result[x + middle][y + middle] = (
                    result[x][y] +
                    result[x + square_size][y] +
                    result[x + square_size][y + square_size] +
                    result[x][y + square_size]
                ) / 4.0 + (random * 2.0 - 1.0) * (square_size as f64) * (square_size as f64) * roughness;
                y += square_size;
            }
            x += square_size;
        }

        // diamond step
        x = middle;
        while x < size - middle {
            y = 0;
            while y < size - 1 {
                let random: f64 = rng.gen();
                let mut counter = 2;
                result[x][y] = result[x - middle][y] + result[x + middle][y];
                if y > 0 {
                    result[x][y] += result[x][y - middle];
                    counter += 1;
                }
                if y + middle < size {
                    result[x][y] += result[x][y + middle];
                    counter += 1;
                }
                result[x][y] = result[x][y] / (counter as f64);
                result[x][y] += (random * 2.0 - 1.0) * (square_size as f64) * (square_size as f64) * roughness;
                y += square_size;
            }
            x += square_size;
        }

        y = middle;
        while y < size - middle {
            x = 0;
            while x < size - 1 {
                let random: f64 = rng.gen();
                let mut counter = 2;
                result[x][y] = result[x][y - middle] + result[x][y + middle];
                if x > 0 {
                    result[x][y] += result[x - middle][y];
                    counter += 1;
                }
                if x + middle < size {
                    result[x][y] += result[x + middle][y];
                    counter += 1;
                }
                result[x][y] = result[x][y] / (counter as f64);
                result[x][y] += (random * 2.0 - 1.0) * (square_size as f64) * (square_size as f64) * roughness;
                x += square_size;
            }
            y += square_size;
        }
        square_size = middle;
    }

    let mut rounded_result = Vec::new();
    for i in 0..size {
        rounded_result.push(Vec::new());
        for j in 0..size {
            rounded_result[i].push((result[i][j]).round() as usize);
        }
    }
    rounded_result
}
