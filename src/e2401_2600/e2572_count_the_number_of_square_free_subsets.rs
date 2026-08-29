const MOD: i64 = 1_000_000_007;

fn dfs(
    num: usize,
    hashmap: &[i64],
    bits: &[i32],
    visited: &mut i32,
) -> i64 {
    let mut res = hashmap[num];
    res = (res + hashmap[num] * hashmap[1]) % MOD;

    let mut sum = res;
    for x in num+1..31 {
        if hashmap[x] != 0 && *visited & bits[x] == 0 {
            *visited ^= bits[x];
            let val = hashmap[num] * dfs(x, hashmap, bits, visited) % MOD;
            sum = (sum + val) % MOD;
            *visited ^= bits[x];
        }
    }

    sum
}

fn square_free_subsets(nums: Vec<i32>) -> i32 {
    let mut bits = [0;31];
    bits[2]  = 0b10;
    bits[3]  = 0b100;
    bits[6]  = 0b110;
    bits[5]  = 0b10000;
    bits[10] = 0b10010;
    bits[15] = 0b10100;
    bits[30] = 0b10110;
    bits[7]  = 0b1000000;
    bits[21] = 0b1000100;
    bits[14] = 0b1000010;
    bits[11] = 0b10000000000;
    bits[22] = 0b10000000010;
    bits[13] = 0b1000000000000;
    bits[26] = 0b1000000000010;
    bits[17] = 0b10000000000000000;
    bits[19] = 0b1000000000000000000;
    bits[23] = 0b10000000000000000000000;
    bits[29] = 0b10000000000000000000000000000;

    let mut hashmap = [0;31];
    for num in nums {
        if num == 1 {
            hashmap[num as usize] = hashmap[num as usize] * 2 + 1;
        } else if bits[num as usize] != 0 {
            hashmap[num as usize] += 1;
        }
    }

    let mut result = 0;
    for num in 2..31 {
        if hashmap[num] != 0 {
            result = (result + dfs(num, &hashmap, &bits, &mut bits[num].clone())) % MOD;
        }
    }
    ((result + hashmap[1]) % MOD) as i32

}

pub fn main() {
    let nums = [1,1,1,2].to_vec();
    println!("{:?}", square_free_subsets(nums));
}

