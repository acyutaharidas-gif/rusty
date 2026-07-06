fn main() {
    println!("Hello, world!");
    a_function(32);

    let y = {
        let x = 3;
        x + 1 //no semi-colons >:3
    };

    println!("the value of x is {y}");

    // control flow
    if true {
        //cant do numbers like js here num =3 and if num would throw error. boolean strict
        println!("la la la");
    } else if false {
        println!("no la la :(");
    } else {
        println!("muhehehehehe");
    }

    // let anum = 5;
    // if anum {
    // println!("This WILL throw error or i will throw hands!");
    // }

    let bool_value = true;
    let _num = if bool_value { 67 } else { 76 };

    let mut i = 0;
    'loop_can_have_labels: loop {
        println!("we;re in a loop!");
        i += 1;

        let mut q = 2;

        loop {
            println!("a loop in a loop ! ");
            if i > 3 && bool_value {
                break 'loop_can_have_labels;
            }
            q -= 1;
            if q == 0 {
                break;
            }
        }
    }

    println!("i : {i}");

    let some_char = a_while_loop(5);
    println!("some_char: {some_char}");

    let arrrrrr = [10, 20, 30, 40, 50];
    a_for_loop(&arrrrrr); //this is slicing ? and borrowing? some ownership mumbo jumbo
    another_for_loop(arrrrrr);

    for number in (1..=4).rev() {
        println!("num: {number}");
    }
}

//functions uses snake_case_like_this
// & -> for return values
fn a_function(x: i32) -> i32 {
    println!("paramter value is: {x}");
    x //no semi-colon at the end (return) expression else it becomes a statement
      // tldr: no semi-colons just put the returning value.
}

fn a_while_loop(mut num: i8) -> char {
    while num != 0 {
        println!("muhehehehehe");
        num -= 1;
    }

    println!("WHILE EXITED");
    '🎉'
}

fn a_for_loop(arr: &[i32]) {
    for &a in arr {
        println!("current element: {a}");
    }
}

fn another_for_loop(arr: [i32; 5]) {
    for a in arr {
        println!("array element: {a}");
    }
}
