pub fn hanoi_recursion()  {
    let mut stack_a: Vec<i32> = vec![4,3,2,1];
    let mut stack_b: Vec<i32> = Vec::new();
    let mut stack_c: Vec<i32> = Vec::new();

    move_(4, &mut stack_a, &mut stack_c, &mut stack_b);

}

fn move_ (n: i32, source: &mut Vec<i32>, target: &mut Vec<i32>, aux: &mut Vec<i32>) {
    if n > 0 {
        
        move_(n-1, source, aux, target);
        
        target.push(source.pop().unwrap());
        // println!("{:?}\n{:?}\n{:?}", source, aux, target);
        
        move_(n-1, aux, target, source);
    }
}
