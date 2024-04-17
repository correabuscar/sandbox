use rand::Rng;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use std::thread;


////okTODO: since HashSet is a HashMap underneath(isn't it? chatgpt3.5 said) then why not use HashMap
////and store a counter as the value
//fn get_shared_data() -> &'static Arc<RwLock<HashSet<String>>> {
//    // static gets inited only once before main() and is scoped only to this function
//    static SHARED_DATA: OnceLock<Arc<RwLock<HashSet<String>>>> = OnceLock::new();
//    //the inner value (hashset) is inited only once on first call of this function
//    SHARED_DATA.get_or_init(|| Arc::new(RwLock::new(HashSet::new())))
//}
fn get_shared_data() -> &'static Arc<RwLock<HashMap<String,u64>>> {
    // static gets inited only once before main() and is scoped only to this function
    static SHARED_DATA: OnceLock<Arc<RwLock<HashMap<String,u64>>>> = OnceLock::new();
    //the inner value (hashset) is inited only once on first call of this function
    SHARED_DATA.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

fn main() {
    // Spawn multiple reader threads
    for i in 0..50 {
        let shared_data_clone = Arc::clone(get_shared_data());
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let wait_duration = std::time::Duration::from_millis(rng.gen_range(i*15..i*30));
            thread::sleep(wait_duration);
            // Acquire a read lock and access the data
            if let Ok(data) = shared_data_clone.read() {
                println!("Reader thread {} read data: {:?}", i, *data);
            }
        });
    }

    // Spawn a writer thread
    thread::spawn(|| {
            let mut rng = rand::thread_rng();
        for _i in 0..50 {
            let wait_duration = std::time::Duration::from_millis(rng.gen_range(10..20));
            thread::sleep(wait_duration);
            // Acquire a write lock and modify the data
            if let Ok(mut data) = get_shared_data().write() {
                let key=wait_duration.as_millis().to_string();
                let entry = data.entry(key.clone()).or_insert(0);
                *entry += 1;
                //if let Some(val)=data.get(&key) {
                //    data.insert(wait_duration.as_millis().to_string(), val+1);
                //}
                println!("Writer thread modified data");
            }
        }
    })
    .join()
    .unwrap(); // Wait for the writer thread to finish

    // Wait for all reader threads to finish
    thread::sleep(std::time::Duration::from_millis(1500));
}
