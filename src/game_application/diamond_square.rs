use game_application::rand::{Rng, thread_rng};

pub fn generate_hight_map(size: usize) -> Vec<Vec<usize>> {
    let mut rng = thread_rng();
    let mut result = Vec::new();
    result.resize(size, Vec::new());
    for i in 0..size {
        result[i].resize(size, 0);
    }

    let mut square_size = size - 1;
    let mut random_bound = 8;
    while square_size > 0 {
        // square step
        let middle = square_size / 2;
        let mut x = 0;
        let mut y;
        while x < size - 1 {
            y = 0;
            while y < size - 1 {
                result[x + middle][y + middle] = (
                    result[x][y] +
                    result[x + square_size][y] +
                    result[x + square_size][y + square_size] +
                    result[x][y + square_size]
                ) / 4;
                if random_bound > 1 {
                    result[x + middle][y + middle] += rng.gen_range(0, random_bound);
                }
                y += square_size;
            }
            x += square_size;
        }

        // diamond step
        x = middle;
        while x < size - middle {
            y = 0;
            while y < size - 1 {
                // square step
                let mut counter = 2;
                result[x][y] = result[x - middle][y] + result[x + middle][y];
                if y > 0 {
                    result[x][y] += result[x][y - middle];
                    counter += 1;
                }
                if y < size - 1 {
                    result[x][y] += result[x][y + middle];
                    counter += 1;
                }
                result[x][y] = result[x][y] / counter;
                if random_bound > 1 {
                    result[x][y] += rng.gen_range(0, random_bound);
                }
                y += square_size;
            }
            x += square_size;
        }


        y = middle;
        while y < size - middle {
            x = 0;
            while x < size - 1 {
                // square step
                let mut counter = 2;
                result[x][y] = result[x][y - middle] + result[x][y + middle];
                if x > 0 {
                    result[x][y] += result[x - middle][y];
                    counter += 1;
                }
                if x < size - 1 {
                    result[x][y] += result[x + middle][y];
                    counter += 1;
                }
                result[x][y] = result[x][y] / counter;
                if random_bound > 1 {
                    result[x][y] += rng.gen_range(0, random_bound);
                }
                x += square_size;
            }
            y += square_size;
        }

        if random_bound > 1 {
            random_bound -= 1;
        }
        square_size = middle;
    }

    result
}
