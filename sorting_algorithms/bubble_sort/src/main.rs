use rand::{thread_rng, Rng};

fn main() {
    // Bubble Sort

    // generate 100 random numbers
    println!("Random numbers:");
    let mut numbers: Vec<i32> = Vec::new();
    for _ in 0..100 {
        let mut rng = thread_rng();
        let random_number = rng.gen_range(0..100);
        numbers.push(random_number);
        print!("{} ", random_number);
    }
    print!("\n");

    // track changes and vec size
    let mut do_next: bool = true;
    let mut total_numbers = numbers.len() as i32;

    // sort the array
    while do_next {
        // the array is sorted if there is only one number to check
        if total_numbers <= 1 {
            break;
        }

        // reset the sentinel and decrement total to check
        do_next = false;
        total_numbers -= 1;

        // sort rest of array
        for pos in 0..total_numbers {
            let current = pos as usize;
            let next = (pos + 1) as usize;

            // check if a swap is needed
            if numbers[current] > numbers[next] {
                // get a temp value
                let temp = numbers[current];

                // perform the swap
                numbers[current] = numbers[next];
                numbers[next] = temp;
                do_next = true;
            }
        }
    }

    // print the sorted list
    println!("Sorted numbers:");
    for _num in numbers.iter() {
        print!("{} ", _num);
    }
    print!("\n");
}
