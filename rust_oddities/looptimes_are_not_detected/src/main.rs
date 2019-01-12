#![allow(unused_variables)]
//#![allow(dead_code)]

struct NewType(i32);

//see https://github.com/rust-lang/rust/issues/57555

fn main() {

    let mut v:Vec<NewType>=Vec::new();

    loop {
        println!("loop iteration");//hit once
        let a:NewType;
        change(&mut a); //true: "borrow of possibly uninitialized variable: `a`"
        v.push(a);
        break; // due to this break, the loop is detected as happening only once
    }

    loop {
        println!("loop iteration");//hit once
        let a:NewType;
		//see: confusing_overwritten_errors and forloop_innervar_shadowing for why the false error happen:
        change(&mut a); //false: "value borrowed here after move" see: https://github.com/rust-lang/rust/issues/57553
        v.push(a);// false: "value moved here, in previous iteration of loop"
        if 1==1 { // loop is no longer detected as happening only once due to this 'if'
          break;
        }
    }

    for i in 1..1 {
        println!("loop iteration");//never hit
        let a:NewType;
        change(&mut a); //false: "value borrowed here after move"
        v.push(a);// false: "value moved here, in previous iteration of loop"
        //break; //uncommenting this makes rust think the loop happens only once(even though it happens never) and thus reveals the true error: "borrow of possibly uninitialized variable: `a`"
    }

    for i in 1..=1 {
        println!("loop iteration");//hit once
        let a:NewType;
        change(&mut a); //false: "value borrowed here after move"
        v.push(a);// false: "value moved here, in previous iteration of loop"
    }

    for i in 1..=2 {
        println!("loop iteration");//hit twice
        let a:NewType;
        change(&mut a); //false: "value borrowed here after move"
        v.push(a);// false: "value moved here, in previous iteration of loop"
    }


}

fn change(i: &mut NewType) {
    //*i=NewType(1);
}

