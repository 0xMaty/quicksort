fn main() {
    // Sort integers.
    let mut integers = [10, -3, 530, -1, 2, 55, 54, 56, -55, -54, -56, 0];
    println!("Integers: {:?}", integers);
    let r = (integers.len() - 1) as isize;
    quicksort(&mut integers, 0, r);
    println!("Sorted Integers: {:?}", integers);

    // Sort strings.
    let mut strings = [
        "LeBron James",
        "Michael Jordan",
        "Kobe Bryant",
        "Shaquille O'Neal",
        "Dwyane Wade",
    ];
    println!("Strings: {:?}", strings);
    quicksort(&mut strings, 0, 4);
    println!("Sorted Strings: {:?}", strings);
}

// Time Complexity:
// - best case: O(nlog(n))
// - worst case: O(n^2)
fn quicksort<T: Ord>(array: &mut [T], p: isize, r: isize) {
    if p < r {
        let q = partition(array, p, r);
        quicksort(array, p, q - 1);
        quicksort(array, q + 1, r);
    }
}

fn partition<T: Ord>(array: &mut [T], p: isize, r: isize) -> isize {
    let mut i = p - 1;
    for j in p..=(r - 1) {
        if array[j as usize] < array[r as usize] {
            i += 1;
            array.swap(i as usize, j as usize);
        }
    }
    i += 1;
    array.swap(i as usize, r as usize);
    i
}
