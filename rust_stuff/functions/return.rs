fn main() {
    println!("1 + 1 = {}", add_one(1));
    println!("if x = 2, than foo() returns: {}", foo(2));
    println!("if x = 9, than foo() returns: {}", foo(9));
}

fn add_one(x:int) -> int {
    x + 1
}

fn foo(bar: int) -> int {
    if bar < 5 { return bar; }

    // like ruby :^)
    bar + bar
}
