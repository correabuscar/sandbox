
use no_heap_allocations_thread_local::NoHeapAllocThreadLocal;
use no_heap_allocations_thread_local::get_current_thread_id;

#[derive(Debug, Clone, PartialEq)]
struct MyType(usize);
//impl MyType {
//    fn inc(&mut self, i:usize) {
//        self.0+=i;
//    }
//}

const HOW_MANY: usize = 10;
static FOO: NoHeapAllocThreadLocal<HOW_MANY, MyType> = NoHeapAllocThreadLocal::new();

fn main() {
    println!("Hello thread local without any allocations on heap");
    println!("{:?}", FOO);

    let mut handles = Vec::new();
    for i in 1..=HOW_MANY*2 {
        let handle=std::thread::spawn(move || {
            let current_thread_id = get_current_thread_id();//FOO::get  std::thread::current().id().as_u64();
            //let set_to=MyType(current_thread_id.get() as usize * 10);
            let set_to=MyType(current_thread_id as usize * 10);
            if let (already_existed,Some(mut val)) = FOO.get_or_set(set_to.clone(),std::time::Duration::from_secs((i/2) as u64)) {
                println!(
                    "Slot allocated for thread {}, already existed? {}, val={:?} wanted to set to {:?}",
                    current_thread_id, already_existed, val, set_to
                );
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
                std::thread::sleep(std::time::Duration::from_millis(300*i as u64));
                FOO.unset();
            } else {
                println!("No available slots found for thread {}", current_thread_id);
            }
        }); //spawn
        handles.push(handle);
    }
     // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", FOO);
    println!("All threads have finished");
}
