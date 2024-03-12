// Define a wrapper trait for WrappedString that erases the size of Self
trait WrappedStringDyn {
    fn to_string_dyn(&self) -> String;
}

// Implement WrappedStringDyn for any type that implements WrappedString
impl<T: WrappedString> WrappedStringDyn for T {
    fn to_string_dyn(&self) -> String {
        self.to_string()
    }
}

// Define the original trait for wrapped strings
trait WrappedString {
    fn new(value: &str) -> Self;
    fn to_string(&self) -> String;
}

// Implement the trait for a simple string wrapper
struct SimpleString(String);

impl WrappedString for SimpleString {
    fn new(value: &str) -> Self {
        SimpleString(value.to_string())
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}

// Implement the trait for a reversed string wrapper
struct ReversedString(String);

impl WrappedString for ReversedString {
    fn new(value: &str) -> Self {
        ReversedString(value.chars().rev().collect())
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}

// Implement the trait for a title-cased string wrapper
struct TitleString(String);

impl WrappedString for TitleString {
    fn new(value: &str) -> Self {
        TitleString(value.to_string())
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}

// Define an enum to represent different types of animals
enum AnimalType {
    Dog,
    Cat,
    Bird,
}

// Define a trait for animals
trait Animal {
    fn make_sound(&self) -> Box<dyn WrappedStringDyn>;
}

// Implement the trait for each type of animal
struct Dog;
impl Animal for Dog {
    fn make_sound(&self) -> Box<dyn WrappedStringDyn> {
        Box::new(ReversedString::new("Woof!"))
    }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) -> Box<dyn WrappedStringDyn> {
        Box::new(TitleString::new("Meow!"))
    }
}

struct Bird;
impl Animal for Bird {
    fn make_sound(&self) -> Box<dyn WrappedStringDyn> {
        Box::new(SimpleString::new("Tweet!"))
    }
}

// Function to print the sound of an animal
fn print_sound(animal: &dyn Animal) {
    let sound = animal.make_sound();
    println!("{}", sound.to_string_dyn());
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    let bird = Bird;

    print_sound(&dog);   // Output: foow
    print_sound(&cat);   // Output: Meow!
    print_sound(&bird);  // Output: Tweet!
}

