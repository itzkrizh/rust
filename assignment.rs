fn main() {
    // Define the number for which you want to print the table
    let num = 9;

    // Loop to print the table
    for i in 1..=10 {
        let result = num * i;
        println!("{} x {} = {}", num, i, result);
    }
}