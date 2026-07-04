fn main() {
    let mut x = "hello";
    let ptr1 = &mut x;

    let x = 123;
    let ptr2 = &x;

    println!("addr of ptr1: {:p}", ptr1);
    println!("addr of ptr2: {:p}", ptr2);

    println!("dereferencing ptr1: {}", *ptr1);
    println!("dereferencing ptr2: {}", *ptr2);

    *ptr1 = "hello, World!";
    println!("dereferencing ptr1: {}", *ptr1);
    println!("{x}");

    // example usecase
    {
        let input = "8080 ";
        let input = input.trim();
        let input: i32 = input.parse().unwrap();

        println!("input is now trimmed and a number! {}", input);
    }

    let a = [3; 5]; //a = [3,3,3,3,3];
}
