use std::ops::BitXor;

const ALPHA: u32 = 7;
const BETA: u32 = 2;
const ROUNDS: usize = 22;

fn encrypt_round(x: &mut u16, y: &mut u16, k: u16) {
    *x = x.rotate_right(ALPHA).wrapping_add(*y).bitxor(k);
    *y = y.rotate_left(BETA).bitxor(*x);
}

fn decrypt_round(x: &mut u16, y: &mut u16, k: u16) {
    *y = y.bitxor(*x).rotate_right(BETA);
    *x = x.bitxor(k).wrapping_sub(*y).rotate_left(ALPHA);
}

fn expand_key(key: [u16; 4]) -> [u16; ROUNDS] {
    let mut round_keys = [0u16; ROUNDS];

    let mut l2 = key[0];
    let mut l1 = key[1];
    let mut l0 = key[2];
    let mut k0 = key[3];

    let mut i: usize = 0;
    while i < ROUNDS - 1 {
        round_keys[i] = k0;
        encrypt_round(&mut l0, &mut k0, i as u16);
        i += 1;

        round_keys[i] = k0;
        encrypt_round(&mut l1, &mut k0, i as u16);
        i += 1;

        round_keys[i] = k0;
        encrypt_round(&mut l2, &mut k0, i as u16);
        i += 1;
    }

    round_keys[i] = k0;

    round_keys
}

fn encrypt_block(pt: [u16; 2], key: [u16; 4]) -> [u16; 2] {
    let round_keys = expand_key(key);

    let mut x = pt[0];
    let mut y = pt[1];

    for &k in &round_keys {
        encrypt_round(&mut x, &mut y, k);
    }

    [x, y]
}

fn decrypt_block(pt: [u16; 2], key: [u16; 4]) -> [u16; 2] {
    let round_keys = expand_key(key);

    let mut x = pt[0];
    let mut y = pt[1];

    for &k in round_keys.iter().rev() {
        decrypt_round(&mut x, &mut y, k);
    }

    [x, y]
}

fn main() {
    // Key:       1918 1110 0908 0100
    // Plaintext: 6574 694c
    // Ciphertext:a868 42f2

    let key = [0x1918, 0x1110, 0x0908, 0x0100];
    let pt = [0x6574, 0x694c];

    let ct = encrypt_block(pt, key);
    let pt = decrypt_block(ct, key);

    println!("ciphertext = {:04x} {:04x}", ct[0], ct[1]);
    println!("plaintext = {:04x} {:04x}", pt[0], pt[1]);
}
