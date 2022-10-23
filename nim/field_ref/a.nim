#type
#  Person = ref object of RootObj
#    name*: string
#    age: int
#    moo: int
#
#var
#  p: Person
#

#const
#  ExpBits* = 10'i32
const
  ExpBitsMask* = 1'i32 shl ExpBits
  ExpBits* = 10'i32
