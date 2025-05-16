extern crate rand;
use rand::Rng;

fn main() {
    let new_key: Keys = generate_rsa_keys(); 
    println!("The keys are, n = {}, e = {}, d = {}", new_key.n, new_key.e, new_key.d);


    let msg = String::from("meow");
    let ct = rsa_encrypt(new_key.n, new_key.e, msg);

    let decrypted_msg = rsa_decrypt(new_key.n, new_key.d, ct);
    print!("Decrypted message: {}", decrypted_msg);
}

// Eucleadian Algo
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); //terminates if n or m is 0
    while m != 0 {
        if m < n {
            let t = m; 
            m = n;
            n = t;
        }
        m = m % n;
    }
    n 
}

//Simple Trial division for smaller num (RSA uses Probabilistic tests like Miller-Rabin)
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false; 
    }
    // Is num divisible by any number from 2 to sqrt(num)
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false; // divisible
        }
    }   
    true 
}


// (num^exp) % modulus, using Right to left binary exponentiation
fn modular_exponent(mut num:u64 ,mut exp:u64 , modulus:u64) -> u64{
    let mut ans = 1;
    if exp <= 0 { return 1; }
    loop {
        if exp == 1 { return (ans * num) % modulus; }
        if exp & 1 == 0 { //checks if exp is even
            num = (num * num) % modulus;
            exp >>= 1; //right shift and assign. exp=exp>>1
            continue;
        }
        else {
            ans = (ans * num) % modulus;
            exp -= 1;
        }
    }
}

// Fermat's little theorem: (n^(p-2)) mod p (p is prime)
// fn mod_inverse_prime (num:u64, modulus:u64) -> u64{
//     if modulus <= 1 || gcd(num, modulus)>1 {
//         return 0; //no mod inv exists
//     }
//     return modular_exponent(num, modulus-2, modulus);
// }


// Extended Euclidean ALgorithm 
fn mod_inverse(a: u64, m: u64) -> u64 {
    let (mut t, mut new_t) = (0_i64, 1_i64); //t, t'
    let (mut r, mut new_r) = (m as i64, a as i64); //r, r'

    while new_r != 0 {
        let q = r / new_r;
        let temp_t = t - q * new_t;
        let temp_r = r - q * new_r;
        t = new_t;
        new_t = temp_t;
        r = new_r;
        new_r = temp_r;
    }

    if r > 1 { return 0 }; // No inverse
    if t < 0 { t += m as i64 };

    t as u64
}


//safely convert u64 to u8
fn convert_u64_to_u8(x: u64) -> u8 {
    if x <= u8::MAX as u64 { //is it bigger than u8?
        x as u8
    } else {
        panic!("u64 to u8 failed");
    }
}

struct Keys {
    n: u64,
    e: u64,
    d: u64,
}


fn generate_rsa_keys() -> Keys {
    let mut rng = rand::thread_rng();
    let mut p: u64;
    let mut q: u64;
    loop { //biger than msg
        p = rng.gen_range(300..10000);
        if is_prime(p) {
            break;
        }
    }
    loop {
        q = rng.gen_range(300..10000);
        if is_prime(q) {
            break;
        }
    }
    let n: u64 = p * q;
    let r: u64 = (p - 1) * (q - 1);
    let mut e: u64 = 5;
    //pick a prime val e, or:
    for i in 3..r {
        if gcd(i, r) == 1 {
            e = i;
            break;
        }
    }
    // Used a modinverse crate below, which didn't work
    // let d = match modinverse(e, r) { //returns Option type
    //     Some(val) => val,
    //     None => panic!("Mod Inverse don't exist!"),
    // };
    let d = mod_inverse(e, r); //e^-1 mod r

    Keys { n: (n), e: (e), d: (d) }
}

fn rsa_encrypt(n: u64, e: u64, txt: String) -> Vec<u64> {
    let mut ct = Vec::new();
    for l in txt.chars() { 
        ct.push(modular_exponent((l as u8) as u64, e, n));
    }
    println!("Encrypted message: {:?}", ct);
    ct
}

fn rsa_decrypt(n: u64, d: u64, ct: Vec<u64>) -> String {
    let mut msg: Vec<char> = Vec::new();
    for i in ct {
        let ascii = modular_exponent(i, d, n);
        msg.push(convert_u64_to_u8(ascii) as char);
    }
    msg.iter().collect()
}

