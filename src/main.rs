use std::fs;
use std::env;
use rand::Rng;

const BULLET: &str = "00000\n00100\n01110\n00100\n00000";

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("failed to read");
    let lines: Vec<_> = content.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
    let bullet_bin: Vec<_> = BULLET.lines().map(|l| l.chars().collect::<Vec<_>>()).collect();
    let size: (usize, usize) = (lines.iter().map(|l| l.len()).max().unwrap(), lines.len());

    let pos = random_pos(size);
    if check_bullet_fit(&lines, pos, get_bullet_size()) {
        let res = shoot(&lines, &bullet_bin, pos, get_bullet_size());
        println!("{}", res);
    } else {
        println!("miss");
    }
}

fn get_bullet_size() -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;
    for l in BULLET.lines() {
        if l.len() > x {
            x = l.len();
        }
        y += 1;
    }
    (x, y)
}

fn check_bullet_fit(lines: &Vec<Vec<char>>, cor: (usize, usize), bu: (usize, usize)) -> bool {
    let mut is_ok = true;
    for _ in 0..bu.0 {
        for _ in 0..bu.1 {
            match lines.get(cor.1+bu.1) {
                None => is_ok = false,
                Some(l) => if l.get(cor.0+bu.0) == None { is_ok = false },
            }
        }
    }
    is_ok
}

fn shoot(lines: &Vec<Vec<char>>, bullet: &Vec<Vec<char>>, cor: (usize, usize), bu: (usize, usize)) -> String {
    let mut lines_c = lines.clone();
    for i in 0..bu.0 {
        for j in 0..bu.1 {
            if bullet[j][i] == '1' {
                lines_c[cor.1+j][cor.0+i] = '!';
            }
        }
    }
    lines_c.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
}

fn random_pos(size: (usize, usize)) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..size.0), rng.gen_range(0..size.1))
}
