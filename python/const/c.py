#!/usr/bin/python

#src: https://stackoverflow.com/questions/2682745/how-do-i-create-a-constant-in-python/43941956#43941956

class Constant:
  def __init__(self,value=None):
    self.value = value
  def __get__(self,instance,owner):
    return self.value
  def __set__(self,instance,value):
    raise ValueError("You can't change a constant")

class A:
  NULL = Constant()
  NUM = Constant(0xFF)

class B:
  NAME = Constant('bar')
  LISTA = Constant([0,1,'INFINITY'])

obj=A()
print(A.NUM)
print(obj.NUM)
#obj.NUM=2
#del obj.NUM
#obj.NUM=2

#XXX yah that worked and both are changed!
A.NUM=2
print(A.NUM)
print(obj.NUM)

