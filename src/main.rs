use std::fs;
use std::env;
use rand::Rng;

struct Shotgun {
    bullet: Vec<Vec<char>>,
    bullet_size: (usize, usize),
}

impl Shotgun {
    pub fn new() -> Self {
        Self {
            bullet: vec![vec![]],
            bullet_size: (0,0)
        }
    }
    pub fn load_bullet(&mut self, file: String) {
        let content = fs::read_to_string(file).expect("failed to read bullet");
        self.bullet_size = get_size(&content);
        self.bullet = content.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
    }

    fn check_bullet_fit(&self, lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
        let mut is_ok = true;
        let bu = self.bullet_size;
        for _ in 0..bu.0 {
            for _ in 0..bu.1 {
                match lines.get(pos.1+bu.1) {
                    None => is_ok = false,
                    Some(l) => if l.get(pos.0+bu.0) == None || l.get(pos.0+bu.0) == Some(&' ') { is_ok = false },
                }
            }
        }
        is_ok
    }

    pub fn fire(&self, lines: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<String> {
        let bu = self.bullet_size;
        if self.check_bullet_fit(lines, pos) {
            let mut lines_c = lines.clone();
            for i in 0..bu.0 {
                for j in 0..bu.1 {
                    if self.bullet[j][i] == '1' {
                        lines_c[pos.1+j][pos.0+i] = '*';
                    }
                }
            }
            Some(lines_c.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<_>>().join("\n"))
        } else {
            None
        }
    }
}
    

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("failed to read");
    let lines: Vec<_> = content.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();

    let mut gun = Shotgun::new();
    gun.load_bullet("b".to_string());
    let res = gun.fire(&lines, random_pos(get_size(&content))).unwrap_or("miss".to_string());
    println!("{}", res);
}



fn random_pos(size: (usize, usize)) -> (usize, usize) {
    dbg!(size);
    let mut rng = rand::thread_rng();
    dbg!((rng.gen_range(0..size.0), rng.gen_range(0..size.1)))
}

fn get_size(content: &String) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;
    for l in content.lines() {
        if l.len() > x {
            x = l.len();
        }
        y += 1;
    }
    (x,y)
}
