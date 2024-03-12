#!/usr/bin/env python3

# src: chatgpt generated

class WrappedString:
    def __init__(self, value: str):
        self.value = value

    def __str__(self) -> str:
        return self.value

    def __repr__(self) -> str:
        return f'WrappedString({self.value})'

    def upper(self) -> 'WrappedString':
        return WrappedString(self.value.upper())

    def lower(self) -> 'WrappedString':
        return WrappedString(self.value.lower())

    def capitalize(self) -> 'WrappedString':
        return WrappedString(self.value.capitalize())

    # Define other methods as needed
class ReversedString(WrappedString):
    def __str__(self) -> str:
        return self.value[::-1]

class TitleString(WrappedString):
    def __str__(self) -> str:
        return self.value.title()

class Animal:
    def make_sound(self) -> WrappedString:
        return WrappedString("Generic animal sound")

class Dog(Animal):
    def make_sound(self) -> ReversedString:
        return ReversedString("Woof!")

class Cat(Animal):
    def make_sound(self) -> TitleString:
        return TitleString("Meow!")

class Bird(Animal):
    def make_sound(self) -> WrappedString:
        return WrappedString("Tweet!")

def print_sound(animal: Animal) -> None:
    print(animal.make_sound())

dog = Dog()
cat = Cat()
bird = Bird()

#so by being able to pass Dog/Cat/Bird to print_sound which expects an Animal(aka base class type) we're doing contravariance
#and covariance is that make_sound() is expected to return a WrappedString (from Animal) type, but we can have it return other subtypes of WrappedString.
print_sound(dog)   # Covariance: Dog's make_sound returns ReversedString
print_sound(cat)   # Covariance: Cat's make_sound returns TitleString
print_sound(bird)  # Covariance: Bird's make_sound returns WrappedString

