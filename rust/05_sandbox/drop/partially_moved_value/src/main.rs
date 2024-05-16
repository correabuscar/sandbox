#[allow(unused_imports)]
use std::cell::RefCell;

#[derive(Debug)]
#[allow(dead_code)]
struct StuffAboutLocation {
    counter: i32,
}

#[allow(dead_code)]
impl StuffAboutLocation {
    fn new() -> Self {
        StuffAboutLocation {
            counter: 1,
        }
    }
}

fn main() {
}

#[test]
fn test1() {
    let refcell = RefCell::new(Some(StuffAboutLocation::new()));

    let mut res_borrow = refcell.try_borrow_mut();
    //let i:i32=res_borrow;//found enum `Result<RefMut<'_, Option<StuffAboutLocation>>, BorrowMutError>`
    if let Ok(ref mut ref_mut_location) = res_borrow {
        //let i:i32=ref_mut_location;//found mutable reference `&mut RefMut<'_, Option<StuffAboutLocation>>`
        if let Some(lwc) = ref_mut_location.as_mut() {
            //let i:i32=lwc;//found `&mut StuffAboutLocation`
            if lwc.counter > 0 {
                lwc.counter -= 1;
            }
        }
    }
    //XXX: can:
    println!("{:?}",res_borrow);
    drop(res_borrow);
}

#[test]
fn test2() {
    let refcell = RefCell::new(Some(StuffAboutLocation::new()));

    let res_borrow = refcell.try_borrow_mut();
    //let i:i32=res_borrow;//found enum `Result<RefMut<'_, Option<StuffAboutLocation>>, BorrowMutError>`
    if let Ok(mut ref_mut_location) = res_borrow {
        //let i:i32=ref_mut_location;//found struct `RefMut<'_, Option<StuffAboutLocation>>`
        if let Some(lwc) = ref_mut_location.as_mut() {
            //let i:i32=lwc;//found `&mut StuffAboutLocation`
            if lwc.counter > 0 {
                lwc.counter -= 1;
            }
        }
    }
    //XXX: can't:
    //println!("{:?}",res_borrow);//E0382: borrow of partially moved value: `res_borrow`
    //drop(res_borrow); // error: E0382: use of partially moved value: `res_borrow`
}

