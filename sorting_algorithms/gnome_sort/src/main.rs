fn main() {
    // Gnome Sort (similar to bubble, looks ahead and behind)

    // collect a list of numbers from the user
    let mut numbers: Vec<i32> = Vec::new();
    let mut input: String = String::new();
    println!("Enter a list of integers (space delimited): ");
    std::io::stdin().read_line(&mut input).unwrap();
    println!("");

    // parse to a vec
    for num in input.trim().split(" ") {
        numbers.push(num.parse().unwrap());
    }

    // print original array
    print!("You entered: ");
    for val in numbers.iter() {
        print!("{} ", val);
    }
    println!("\n");

    // sort the algo
    gnome_sort(&mut numbers);

    // print sorted array
    print!("Sorted: ");
    for val in numbers.iter() {
        print!("{} ", val);
    }
    println!("\n");
}

fn gnome_sort(numbers: &mut Vec<i32>) {
    // length of array
    let length = numbers.len();
    let mut i: usize = 1;
    let mut j: usize = 2;

    // sort
    while i < length {
        // is prev less? step forward.
        if numbers[i - 1] <= numbers[i] {
            i = j;
            j += 1;
        }
        // otherwise, swap and step back
        else {
            numbers.swap(i - 1, i);
            i -= 1;

            // if at head of list, go forward
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
}
