// This is how rust manages memory. Lexically. A value is dropped when it goes
// out of scope.

fn memory() {
    {
        let vec = vec![1, 2, 3];  // Memory allocated here.
    } // Memory is dropped here.


    fn f(){
        let my_box = Box::new(5); // Explicit heap memory
    } // dropped here

    struct Person { name: String, birth: i32}

    let p = Person{name: "omar".to_string(), birth: 1993};
    let mut vec = Vec::new();
    vec.push(p);
    // let p2 = p; // p has been consumed by push.
}

fn simple_move() {
    let vec = vec![1, 2, 3];
    println!("vec: {:?}", vec);

    // value moved.
    // Clone instead.
    // let vec2 = vec;
    println!("vec: {:?}", vec); // use of moved value: `vec`
}

fn copy_trait() {
    let x = 3;
    println!("x: {:?}", x);

    let y = 4;
    println!("x: {:?}", x); // use of moved value: `vec`
}

// Values are moved when they are taken into a function.
fn f(v: Vec<i32>) {
    unimplemented!();
}

fn function_move(){
    let mut vec = vec![1, 2, 3];
    f(vec);
    // vec.push(3);
}

// Use reference if you don't want to consume the value!

// Values are moved when returned from a function:
fn move_around(v: Vec<i32>) -> Vec<i32> {
    v
}

fn use_move_around() {
    let mut vec = vec![1, 2, 3];
    vec = move_around(vec);
}

// Dangerous c++ code:
// vec<string> v = {"bar", "cat", "foo"};
// auto& r = &v[2];

fn ref_move() {
    let mut vec = vec![1, 2, 3];
    let r = &vec[2]; // borrowing
    // vec = vec![100; 0];
}

fn for_move(vec: Vec<i32>) {
    for v in vec {
        unimplemented!();
    }

    // println!("{:?}", vec);
}

#[derive(Copy, Clone)]
struct Label {number: i32}

// String does not implement Copy!

// #[derive(Copy, Clone)]
// struct LabelString {number: String}


// How would python move a vector?
// l = [1, 2, 3]
// l2 = l
// l3 = l

// Rc and Arc
fn rc_example() {
    use std::rc::Rc;
    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t = s.clone();
    let u = s.clone();
}
