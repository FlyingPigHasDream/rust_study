
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let _y = 3;


    let (a, mut b) : (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    let e = 3;
    let e = e * 2;

    {
        let e = e * 2;
        println!("e = {}",e);
    }

    println!("e2 = {}", e);

    let mut s = "    ";
    let s = s.len();
    println!("s = {}", s);

    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }
}