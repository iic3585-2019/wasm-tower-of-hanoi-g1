pub fn hanoi_recursion(n_disks:i32) {

    let mut stack_a: Vec<i32> = create_reverse_stack(n_disks);
    let mut stack_b: Vec<i32> = Vec::new();
    let mut stack_c: Vec<i32> = Vec::new();

    move_(n_disks, &mut stack_a, &mut stack_c, &mut stack_b);
}

fn create_reverse_stack(n: i32) -> Vec<i32> {
    let mut stack_a: Vec<i32> = Vec::new();
    for i in 1..(n+1) {stack_a.push(i);}
    stack_a.reverse();
    return stack_a;
}

fn move_ (n: i32, source: &mut Vec<i32>, target: &mut Vec<i32>, aux: &mut Vec<i32>) {
    if n > 0 {
        
        move_(n-1, source, aux, target);
        target.push(source.pop().unwrap());
        // println!("{:?}\n{:?}\n{:?}", source, aux, target);
        move_(n-1, aux, target, source);
    }
}
