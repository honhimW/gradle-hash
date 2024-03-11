use md5::{Md5, Digest};

fn main() {
    let param = read_param();
    let mut hasher = Md5::new();
    hasher.update(param.get(1).unwrap());
    let result = hasher.finalize();
    let digest = to_base36(result.as_slice());
    println!("{}", digest);
}

fn read_param() -> Vec<String> {
    let args = std::env::args().collect();
//    println!("{:?}", args);
    args
}

fn to_base36(bytes: &[u8]) -> String {
    let mut num = 0u128;
    for &b in bytes.iter() {
        num = (num << 8) | b as u128;
    }

    let mut base36 = String::new();
    let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
    while num > 0 {
        base36.push(chars[(num % 36) as usize]);
        num /= 36;
    }
    base36.chars().rev().collect()
}
