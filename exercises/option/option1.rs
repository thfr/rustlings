// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn is_even(num: u16) -> bool {
    match num % 2 {
        0 => true,
        _ => false,
    }
}
fn print_number(maybe_number: Option<u16>) {
    match maybe_number {
        Some(x) => println!("found a value: {}", x),
        None => println!("found no value"),
    };
}

fn main() {
    print_number(Option::Some(13));
    print_number(Option::Some(99));
    print_number(Option::None);

    let mut numbers: [Option<u16>; 5] = [None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = { ((iter * 1235) + 2) / (4 * 16) };

        numbers[iter as usize] = Some(number_to_add);
        print_number(numbers[iter as usize]);
    }
}
