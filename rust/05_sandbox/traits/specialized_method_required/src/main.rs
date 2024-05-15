trait UnvisitTrait {
    fn unvisit(&self);
}

struct RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
 {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Drop for RecursionDetectionZoneGuard<T>
where
    RecursionDetectionZoneGuard<T>: UnvisitTrait,
{
    fn drop(&mut self) {
        self.unvisit();
    }
}

struct LocationInSourceCode;
struct NoHeapAllocationsThreadLocalForHere;

impl UnvisitTrait for RecursionDetectionZoneGuard<LocationInSourceCode> {
    fn unvisit(&self) {
        // Specific implementation for LocationInSourceCode
        println!("Unvisit for LocationInSourceCode");
    }
}

impl UnvisitTrait for RecursionDetectionZoneGuard<NoHeapAllocationsThreadLocalForHere> {
    fn unvisit(&self) {
        // Specific implementation for NoHeapAllocationsThreadLocalForHere
        println!("Unvisit for NoHeapAllocationsThreadLocalForHere");
    }
}

fn main() {
    let _specific_guard = RecursionDetectionZoneGuard::<LocationInSourceCode> {
        _marker: std::marker::PhantomData,
    };
    //specific_guard.unvisit(); // Calls the specific implementation

    let _another_guard = RecursionDetectionZoneGuard::<NoHeapAllocationsThreadLocalForHere> {
        _marker: std::marker::PhantomData,
    };
    //another_guard.unvisit(); // Calls the specific implementation

    // XXX: This will cause a compile-time error because i32 does not implement UnvisitTrait
    //let invalid_guard = RecursionDetectionZoneGuard::<i32> { _marker: std::marker::PhantomData };
    // invalid_guard.unvisit();
}

