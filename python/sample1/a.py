#!/usr/bin/pypy3 -bb
#!/usr/bin/python -bb
#pypy3 ^ is almost 4 times faster than python3, but slower to print the output on time, possibly it's buffered for 1 sec?

#src: https://wiki.python.org/moin/SimplePrograms
import re,sys
for test_string in ['555-1212', 'ILL-EGAL']:
    if re.match(r'^\d{3}-\d{4}$', test_string):
        print (test_string, 'is a valid US local phone number')
    else:
        print (test_string, 'rejected')

prices = {'apple': 0.40, 'banana': 0.50}
my_purchase = {
    'apple': 1,
    'banana': 6}
grocery_bill = sum(prices[fruit] * my_purchase[fruit]
                   for fruit in my_purchase)
print ('I owe the grocer $%.2f' % grocery_bill) #$3.40

print("Passed args:",sys.argv[1:])
print("Full args:",sys.argv[:])

# This program adds up integers that have been passed as arguments in the command line
import sys
try:
    total = sum(int(arg) for arg in sys.argv[1:])
    print ('sum =', total)
except ValueError:
    print ('Please supply integer arguments')

# indent your Python code to put into an email
import glob
# glob supports Unix style pathname extensions
python_files = glob.glob('*.py')
for file_name in sorted(python_files):
    print ('    ------' + file_name)

    with open(file_name) as f:
        for line in f:
            print ('    ' + line.rstrip())

    print()


#src: https://stackoverflow.com/questions/2866380/how-can-i-time-a-code-segment-for-testing-performance-with-pythons-timeit/2866456#2866456
import time
from random import random
import types

def myfast():
    l = []
    for i in range(1000):
        l.append(int(random()*100 % 100))
    return l

n = 10000
t0 = time.time()
for i in range(n): myfast()
t1 = time.time()

total_n = t1-t0

print(total_n);

class BankAccount(object):  #hmm ok so 'object' is base class! ie. inheritance
    def __init__(self, initial_balance=0, somethingelse=12):
        self.balance = initial_balance
    def deposit(self, amount):
        self.balance += amount
    def withdraw(self, amount):
        self.balance -= amount
    def overdrawn(self):
        return self.balance < 0
my_account = BankAccount(15)
my_account.withdraw(50)
print (my_account.balance, my_account.overdrawn())
#print(my_account)


#otherwise vim will use tabs instead of spaces for indentation when pypy3 is interpreter
# vim: set ft=python
