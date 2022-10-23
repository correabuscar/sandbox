#!/usr/bin/pypy3 -bb
import fibo
import fibo as renamed
#This does not enter the names of the functions defined in fibo directly in the current symbol table; it only enters the module name fibo there. Using the module name you can access the functions:
fibo.fib(1000)
#0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987
print(fibo.fib2(100))
#[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]
print(fibo.__name__)
#'fibo'
print(renamed.__name__)
#'fibo'


#If you intend to use a function often you can assign it to a local name:
fib = fibo.fib
fib(500)
#0 1 1 2 3 5 8 13 21 34 55 89 144 233 377

#There is a variant of the import statement that imports names from a module directly into the importing module’s symbol table. For example:

from fibo import fib, fib2
print(fib2(200))
#[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
#This does not introduce the module name from which the imports are taken in the local symbol table (so in the example, fibo is not defined).

