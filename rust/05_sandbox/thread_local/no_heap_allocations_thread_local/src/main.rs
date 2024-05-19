use std::sync::Arc;

use no_heap_allocations_thread_local::NoHeapAllocThreadLocal;
use no_heap_allocations_thread_local::get_current_thread_id;

#[derive(Debug, Clone, PartialEq)]
struct MyType(usize);
//impl MyType {
//    fn inc(&mut self, i:usize) {
//        self.0+=i;
//    }
//}
impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

const HOW_MANY: usize = 10;
//static FOO: NoHeapAllocThreadLocal<HOW_MANY, MyType> = NoHeapAllocThreadLocal::new();

fn main() {
    println!("Hello thread local without any allocations on heap");
    #[allow(non_snake_case)]
    let FOO: Arc<NoHeapAllocThreadLocal<HOW_MANY, MyType>> = Arc::new(NoHeapAllocThreadLocal::new());
    println!("{:?}", FOO);
    {
        let first_val=MyType(10101);
        if let (already_existed,Some(val)) = FOO.get_or_set(first_val.clone(),std::time::Duration::from_secs(1)) {
            assert_eq!(false, already_existed);
            assert_eq!(*val, Some(first_val.clone()));
        }
        let diff_val=MyType(20202);
        if let (already_existed,Some(val)) = FOO.get_or_set(diff_val.clone(),std::time::Duration::from_secs(1)) {
            assert_eq!(true, already_existed);
            assert_eq!(*val, Some(first_val.clone()));
        }
        if let (already_existed,Some(val)) = FOO.get_or_set(first_val.clone(),std::time::Duration::from_secs(1)) {
            assert_eq!(true, already_existed);
            assert_eq!(*val, Some(first_val));
        }
        if let (already_existed,Some(val)) = FOO.get_or_set(diff_val.clone(),std::time::Duration::from_secs(1)) {
            assert_eq!(true, already_existed);
            assert_ne!(*val, Some(diff_val));
        }
    }//block
    println!("{:?}", FOO);

    let mut handles = Vec::new();
    for i in 1..=HOW_MANY*2 {
        #[allow(non_snake_case)]
        let FOO=FOO.clone();
        let handle=std::thread::spawn(move || {
            let current_thread_id = get_current_thread_id();//FOO::get  std::thread::current().id().as_u64();
            //let set_to=MyType(current_thread_id.get() as usize * 10);
            let set_to=MyType(current_thread_id as usize * 10);
            if let (already_existed,Some(mut val)) = FOO.get_or_set(set_to.clone(),std::time::Duration::from_secs((i/2) as u64)) {
                println!(
                    "Slot allocated for thread {}, already existed? {}, val={:?} wanted to set to {:?}",
                    current_thread_id, already_existed, val, set_to
                );
                assert_eq!(already_existed, false);
                assert_eq!(*val, Some(set_to),"well, weird, coded wrongly then!");
                //val.unwrap().0+=100;
                //let mut i:MyType=val.unwrap();
                //let mut i:MyType=(*val).unwrap();
                //i.inc(100);
                //(*val).unwrap().inc(100);
                //*val=None;//works
                //works too:
                //let old:usize=val.clone().unwrap().0;
                //*val=Some(MyType(old+100));
                //let mut old:MyType=val.clone().unwrap();
                //old.inc(100);
                //*val=Some(old);
                //if let Some(inner_t) = val.as_mut() {
                //    inner_t.0+=100;
                //}
                val.as_mut().unwrap().0+=i;
                //i.0+=100;//val.unwrap().0+100;
                drop(val);
                //std::thread::sleep(std::time::Duration::from_millis(300*i as u64));
                //FOO.unset();
            } else {
                println!("No available slots found for thread {}, you're likely already having {} threads still using the noalloc-thread-local concurrently, consider using .unset() if you don't need the thread local anymore.", current_thread_id, HOW_MANY);
            }
        }); //spawn
        handles.push(handle);
    }
     // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", FOO);
    //let f=FOO.clone();
    drop(FOO);
    //Arc::drop(&mut FOO);//won't work, need &mut; and "explicit use of destructor method: explicit destructor calls not allowed"
    //unsafe { std::mem::ManuallyDrop::drop(&mut FOO) };
    //println!("{:?}", f);
    println!("All threads have finished. Main is done!");
}
