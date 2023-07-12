/*
 * The Keccak sponge function, designed by Guido Bertoni, Joan Daemen,
 * Michaël Peeters and Gilles Van Assche. For more information, feedback or
 * questions, please refer to our website: http://keccak.noekeon.org/
 * Implementation by the designers,
 * hereby denoted as "the implementer".
 * To the extent possible under law, the implementer has waived all copyright
 * and related or neighboring rights to the source code in this file.
 * http://creativecommons.org/publicdomain/zero/1.0/
 *
 * This implementation is ported to more idiomatic rust compared to the other one.
 */
// tag::function[]
const KECCAK_ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001u64,
    0x0000000000008082u64,
    0x800000000000808au64,
    0x8000000080008000u64,
    0x000000000000808bu64,
    0x0000000080000001u64,
    0x8000000080008081u64,
    0x8000000000008009u64,
    0x000000000000008au64,
    0x0000000000000088u64,
    0x0000000080008009u64,
    0x000000008000000au64,
    0x000000008000808bu64,
    0x800000000000008bu64,
    0x8000000000008089u64,
    0x8000000000008003u64,
    0x8000000000008002u64,
    0x8000000000000080u64,
    0x000000000000800au64,
    0x800000008000000au64,
    0x8000000080008081u64,
    0x8000000000008080u64,
    0x0000000080000001u64,
    0x8000000080008008u64,
];

const KECCAK_RHO_OFFSETS: [u8; 25] = [
    0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8,
    18, 2, 61, 56, 14,
];

macro_rules! index {
    ($x:expr, $y:expr) => {
        (($x) % 5) + 5 * (($y) % 5)
    };
}

macro_rules! rol64 {
    ($a:expr, $offset:expr) => {
        (if ($offset != 0) {
            (((u64::from($a)) << $offset) ^ ((u64::from($a)) >> (64 - $offset)))
        } else {
            $a
        })
    };
}

fn theta(a: &mut [u64; 25]) -> () {
    let mut c: [u64; 5] = [0; 5];
    let mut d: [u64; 5] = [0; 5];

    for x in 0..5 {
        for y in 0..5 {
            c[x] ^= a[index!(x, y)];
        }
    }
    for x in 0..5 {
        d[x] = rol64!(c[(x + 1) % 5], 1) ^ c[(x + 4) % 5];
    }
    for x in 0..5 {
        for y in 0..5 {
            a[index!(x, y)] ^= d[x];
        }
    }
}

fn rho(a: &mut [u64; 25]) -> () {
    for x in 0..5 {
        for y in 0..5 {
            a[index!(x, y)] =
                rol64!(a[index!(x, y)], KECCAK_RHO_OFFSETS[index!(x, y)]);
        }
    }
}

fn pi(a: &mut [u64; 25]) -> () {
    let mut temp_a: [u64; 25] = [0; 25];

    for x in 0..5 {
        for y in 0..5 {
            temp_a[index!(x, y)] = a[index!(x, y)];
        }
    }

    for x in 0..5 {
        for y in 0..5 {
            a[index!(0 * x + 1 * y, 2 * x + 3 * y)] = temp_a[index!(x, y)];
        }
    }
}

fn chi(a: &mut [u64; 25]) -> () {
    let mut c: [u64; 5] = [0; 5];

    for y in 0..5 {
        for x in 0..5 {
            c[x] = a[index!(x, y)]
                ^ ((!a[index!(x + 1, y)]) & a[index!(x + 2, y)]);
        }
        for x in 0..5 {
            a[index!(x, y)] = c[x];
        }
    }
}

fn iota(a: &mut [u64; 25], index_round: usize) -> () {
    a[0] ^= KECCAK_ROUND_CONSTANTS[index_round];
}

pub unsafe extern "C" fn keccak(a: *mut u64) -> () {
    let a: &mut [u64; 25] = std::mem::transmute(a);
    for i in 0..24 {
        theta(a);
        rho(a);
        pi(a);
        chi(a);
        iota(a, i);
    }
}
// end::function[]

#[cfg(test)]
mod tests {

    use super::keccak;

    #[test]
    fn hashing_zeroes_creates_expected_result() {
        let mut input = [0u64; 25];
        let expected_result: [u64; 25] = [
            0xF1258F7940E1DDE7,
            0x84D5CCF933C0478A,
            0xD598261EA65AA9EE,
            0xBD1547306F80494D,
            0x8B284E056253D057,
            0xFF97A42D7F8E6FD4,
            0x90FEE5A0A44647C4,
            0x8C5BDA0CD6192E76,
            0xAD30A6F71B19059C,
            0x30935AB7D08FFC64,
            0xEB5AA93F2317D635,
            0xA9A6E6260D712103,
            0x81A57C16DBCF555F,
            0x43B831CD0347C826,
            0x01F22F1A11A5569F,
            0x05E5635A21D9AE61,
            0x64BEFEF28CC970F2,
            0x613670957BC46611,
            0xB87C5A554FD00ECB,
            0x8C3EE88A1CCF32C8,
            0x940C7922AE3A2614,
            0x1841F924A2C509E4,
            0x16F53526E70465C2,
            0x75F644E97F30A13B,
            0xEAF1FF7B5CECA249,
        ];
        unsafe {
            let input_pointer: *mut u64 = std::mem::transmute(&mut input);
            keccak(input_pointer);
            assert_eq!(input, expected_result)
        }
    }
}
