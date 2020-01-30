fn main() {
    let a = [1, 2, 3];
    let b = &a;
    println!("b address: {:p}", b);

    let mut c = vec![1, 2, 3];
    c.push(4);
    println!("c address: {:p}", &c);
    println!("c: {:?}", c);
    let d = &mut c;
    d.push(4);
    println!("d: {:p}", &d);
    println!("d: {:?}", d);
    println!("c: {:?}", c);
    c.push(5);
//    println!("d: {:?}", d);
    println!("c: {:?}", c);

    let e = &42;
    println!("e: {}", e);
    println!("*e: {}", *e);
}
