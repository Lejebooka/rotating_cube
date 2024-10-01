const SCREEN_HIGH: usize = 44;
const SCREEN_WIDTH: usize = 160;
const HALF_CUBE_SIZE: f64 = 20.0;
const K1: f64 = 40.0;
const INCREMENT_SPEED: f64 = 1.0;
const ROTATE_SPEED_X: f64 = 0.05;
const ROTATE_SPEED_Y: f64 = 0.05;
const ROTATE_SPEED_Z: f64 = 0.01;

#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn rotation_matrix_x(&self, angle: f64) -> Self {
        let (sin_a, cos_a) = angle.sin_cos();
        Vec3 {
            x: self.x,
            y: self.y * cos_a - self.z * sin_a,
            z: self.y * sin_a + self.z * cos_a,
        }
    }

    fn rotation_matrix_y(&self, angle: f64) -> Self {
        let (sin_a, cos_a) = angle.sin_cos();
        Vec3 {
            x: self.x * cos_a + self.z * sin_a,
            y: self.y,
            z: -self.x * sin_a + self.z * cos_a,
        }
    }

    fn rotation_matrix_z(&self, angle: f64) -> Self {
        let (sin_a, cos_a) = angle.sin_cos();
        Vec3 {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
            z: self.z,
        }
    }

    fn projection(&self, distance_from_eye: f64) -> (usize, usize, f64) {
        let ooz = 1.0 / (self.z + distance_from_eye);
        let xp = (SCREEN_WIDTH as f64 / 2.0 + K1 * ooz * self.x) as usize;
        let yp = (SCREEN_HIGH as f64 / 2.0 + K1 * ooz * self.y) as usize;
        (xp, yp, ooz)
    }
}

fn set_character_at_coordinate(
    ch: char,
    zbuffer: &mut [[f64; SCREEN_WIDTH]; SCREEN_HIGH],
    output: &mut [[char; SCREEN_WIDTH]; SCREEN_HIGH],
    x: usize,
    y: usize,
    ooz: f64,
) {
    if x < SCREEN_WIDTH && y < SCREEN_HIGH && ooz > zbuffer[y][x] {
        zbuffer[y][x] = ooz;
        output[y][x] = ch;
    }
}

fn main() {
    print!("\x1b[2J");
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;

    loop {
        let mut output = [[' '; SCREEN_WIDTH]; SCREEN_HIGH];
        let mut zbuffer = [[0.0; SCREEN_WIDTH]; SCREEN_HIGH];

        let mut cx = -HALF_CUBE_SIZE;
        while cx < HALF_CUBE_SIZE {
            let mut cy = -HALF_CUBE_SIZE;
            while cy < HALF_CUBE_SIZE {
                let points = [
                    Vec3 { x: cx, y: cy, z: -HALF_CUBE_SIZE },    
                    Vec3 { x: cx, y: cy, z: HALF_CUBE_SIZE },        
                    Vec3 { x: HALF_CUBE_SIZE, y: cx, z: cy },      
                    Vec3 { x: -HALF_CUBE_SIZE, y: cx, z: cy },    
                    Vec3 { x: cx, y: HALF_CUBE_SIZE, z: cy },           
                    Vec3 { x: cx, y: -HALF_CUBE_SIZE, z: cy },    
                ];

                let symbols = ['.', '#', '$', '~', ';', '+'];

                for (i, &point) in points.iter().enumerate() {
                    let rotated_point = point
                        .rotation_matrix_x(a)
                        .rotation_matrix_y(b)
                        .rotation_matrix_z(c);

                    let (xp, yp, ooz) = rotated_point.projection(100.0);
                    set_character_at_coordinate(symbols[i], &mut zbuffer, &mut output, xp, yp, ooz);
                }

                cy += INCREMENT_SPEED;     
            }
            cx += INCREMENT_SPEED;           
        }

        print!("\x1b[H");
        for row in &output {
            for &ch in row {
                print!("{}", ch);
            }
            println!();
        }

        a += ROTATE_SPEED_X;
        b += ROTATE_SPEED_Y;
        c += ROTATE_SPEED_Z;

        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}


