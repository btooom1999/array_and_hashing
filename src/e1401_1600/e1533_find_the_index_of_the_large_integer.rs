// This is the ArrayReader's API interface.
// You should not implement it, or speculate about its implementation
struct ArrayReader {
    arr: Vec<i32>,
}

impl ArrayReader {
    fn new(arr: Vec<i32>) -> Self {
        ArrayReader { arr }
    }

    // Compares the sum of arr[l..r] with the sum of arr[x..y]
    // return 1 if sum(arr[l..r]) > sum(arr[x..y])
    // return 0 if sum(arr[l..r]) == sum(arr[x..y])
    // return -1 if sum(arr[l..r]) < sum(arr[x..y])
    fn compare_sub(&self, l: usize, r: usize, x: usize, y: usize) -> i32 {
        let sum1: i32 = self.arr[l..=r].iter().sum();
        let sum2: i32 = self.arr[x..=y].iter().sum();
        if sum1 > sum2 {
            1
        } else if sum1 == sum2 {
            0
        } else {
            -1
        }
    }

    // Returns the length of the array
    fn length(&self) -> usize {
        self.arr.len()
    }
}

fn get_index(reader: ArrayReader) -> i32 {
    let mut l = 0;
    let mut r = reader.length() - 1;

    while l < r {
        let m = (l + r) / 2;
        let flag = reader.compare_sub(l, m, m+(l+r)%2, r);
        if flag < 0 {
            l = m+1;
        } else {
            r = m;
        }
    }

    l as i32
}

pub fn main() {
    let arr = [7,7,7,10,7,7,7,7].to_vec();
    let array_reader = ArrayReader::new(arr);
    println!("{}", get_index(array_reader));
}

