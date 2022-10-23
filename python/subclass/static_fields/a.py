#!/usr/bin/python3

class Base(object):
    overridable_field=4
    bits=overridable_field*8
    #bits=0

    def __init__(self):
        print(self.__class__)
        if self.__class__ != Base:
            print(f"recomputing for {self}")
            #recompute static fields for subclasses!
            self.bits=self.overridable_field*8

    def show(self):
        print(f"{self.overridable_field=} {self.bits=}")


class Sub(Base):
    overridable_field=8

if __name__ == "__main__":
    b=Base()
    b.show()
    s=Sub()
    s.show()
