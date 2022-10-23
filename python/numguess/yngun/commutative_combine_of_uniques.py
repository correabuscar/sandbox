#!/usr/bin/python3
#nvm#!/usr/bin/pypy3 -bb

#created on 12 July 2021 ...
#fixed type decls on 09 Aug 2022

import pprint

#nums:int =[1,2,4,8,16,32,64,128,256,512]
#nums:int = []
#n=10 #tables [1..2^(n-1)] as first number in each table
#emptytable=[]
#wtwFIXME: am I declaring the type right?
#tables: int=[[] for x in range(0,n)]
#tables: int = []
#^ tables[cols][rows]

#def try1_nogood() -> None: #this was for commutative_combine_of_uniques.py
#    for i in nums:
#        print(i)
#    #print(nums[0])
#    for j in range(0,n+1):
#        print('Table',j,"(incomplete)")
#        for k in range(j,n+1):
#            sum=0
#            for l in range(j,k+1):
#                #print(nums[l], end=',' if l<k else '')
#                sum+=nums[l]
#            print(sum)

def try2_good(n: int) -> None: #this is just subtraction now, no combine!
    nums:list[int] = []
    tables:list[list[int]] = []
    #^ tables[cols][rows]

    #tables[0].append(10)
    #tables[0].append(1)
    #print(tables[0][1])
    for i in range(0,n): #[0..n) aka [0..n-1] because 'range' is special like that
        assert i>=0
        assert i<n
        assert i<=(n-1)
        nums.append(2**i)
        tables.append([])
    pprint.pprint(nums)
    assert len(nums) == n
    assert nums[0] == 1
    assert nums[n-1] == 2**(n-1)
    #pprint.pprint(tables)
    assert len(tables) == n
    assert tables[0] == []
    assert tables[n-1] == []
    #tables[5].append(1)
    #tables[5].append(2)
    #pprint.pprint(tables)
    for num in range(2**n-1,0,-1): # eg. [1023..1] for n=10
        #print(num)
        assert num >= 1
        assert num <= ((2**n)-1)

        newnum: int = num
        for i in range((n-1),-1,-1): # from n-1 to 0, step -1
            assert i <= n-1
            assert i >= 0
            #from highest number in nums eg. 512 down to 1
            #if num == 9:
            #    print(newnum,'-',nums[i],'=', newnum - nums[i])
            oldnum = newnum
            newnum = newnum - nums[i]
            if newnum >= 0:
                #print('table',i)
                tables[i].append(num)
            if newnum == 0:
                break;
            if newnum < 0:
                newnum = oldnum
                continue
        #for i in range(0,n):
        #    print(tables[i])
    #    if num <= 991:
    #        break;
    #pprint.pprint(tables)

    #must reverse once
    for i in range(0,n):
        assert i>=0
        assert i<n
        tables[i].reverse()
        assert tables[i][0] == 2**i, f"first number must be 2^X where X={i}"

    #verify tables: (this takes a long time!)
    #for num in range(2**n-1,0,-1): # eg. [1023..1] for n=10
    for num in range(1,2**n): # eg. [1..1023] for n=10
        assert num >= 1
        assert num < 2**n
        sum1=0
        for i in range(0,n):
            assert i >= 0
            assert i<n
            if num in tables[i]:
                what=tables[i][0]
                assert what == 2**i
                sum1+=what
                #print(f"{num} in table {i} thus adding {2**i} => {sum1}")
                #print(tables[i])
        assert sum1 == num, f"{sum1} != {num}"
        #print()


    #show:
    for i in range(0,n):
        print('Table',i)
        print(tables[i])

#this one made on 09 Aug 2022
#def try3_good(n: int, max_num_in_list:int=(2**n-1)) -> None: #this is just checking the binary bits instead! definitely the one to use/best variant!
def try3_good(max_num_in_list:int) -> None: #this is just checking the binary bits instead! definitely the one to use/best variant!
    #if max_num_in_list <=0:
    #    max_num_in_list=(2**n-1)
    assert max_num_in_list>=1
    n:int=get_power_for_maxlimit(max_num_in_list)
    assert 2**n -1 >= max_num_in_list
    nums:list[int] = []
    tables:list[list[int]] = []
    #^ tables[cols][rows]

    #tables[0].append(10)
    #tables[0].append(1)
    #print(tables[0][1])
    for i in range(0,n): #[0..n) aka [0..n-1] because 'range' is special like that
        assert i>=0
        assert i<n
        assert i<=(n-1)
        nums.append(2**i)
        tables.append([])
        assert nums[i] == 2**i

    pprint.pprint(nums)
    assert len(nums) == n
    assert nums[0] == 1
    assert nums[n-1] == 2**(n-1)
    #pprint.pprint(tables)
    assert len(tables) == n
    assert tables[0] == []
    assert tables[n-1] == []
    #tables[5].append(1)
    #tables[5].append(2)
    #pprint.pprint(tables)
    #for num in range(2**n-1,0,-1): # eg. [1023..1] for n=10
    #XXX: +1 for range, below, so as to include max_num_in_list itself!
    for num in range(1, max_num_in_list+1): #2**n): #2**n-1,0,-1): # eg. [1023..1] for n=10
        #print(num)
        assert num >= 1
        assert num <= ((2**n)-1)

        #newnum: int = num
        #for i in range((n-1),-1,-1): # from n-1 to 0, step -1
        for i in range(0,n):
            assert i <= n-1
            assert i >= 0
            #from highest number in nums eg. 512 down to 1
            #if num == 9:
            #    print(newnum,'-',nums[i],'=', newnum - nums[i])
            #if (num & nums[i] == nums[i]):
            if (num & nums[i]):
                tables[i].append(num)
            #oldnum = newnum
            #newnum = newnum - nums[i]
            #if newnum >= 0:
            #    #print('table',i)
            #    tables[i].append(num)
            #if newnum == 0:
            #    break;
            #if newnum < 0:
            #    newnum = oldnum
            #    continue
        #for i in range(0,n):
        #    print(tables[i])
    #    if num <= 991:
    #        break;
    #pprint.pprint(tables)

    #just check
    for i in range(0,n):
        assert i>=0
        assert i<n
        #tables[i].reverse()
        assert tables[i][0] == 2**i, f"first number must be 2^X where X={i}"
        assert nums[i] == 2**i

    #verify tables: (this takes a long time!)
    #for num in range(2**n-1,0,-1): # eg. [1023..1] for n=10
    for num in range(1, max_num_in_list+1): #2**n): # eg. [1..1023] for n=10
        assert num >= 1
        assert num < 2**n
        sum1=0
        for i in range(0,n):
            assert i >= 0
            assert i<n
            if num in tables[i]:
                what=tables[i][0]
                assert what == 2**i
                sum1+=what
                #print(f"{num} in table {i} thus adding {2**i} => {sum1}")
                #print(tables[i])
        assert sum1 == num, f"{sum1} != {num}"
        #print()


    #show:
    for i in range(0,n):
        print('Table',i)
        print(tables[i])

def get_power_for_maxlimit(number_maxlimit:int) -> int:
    #assumed number_maxlimit is wanted in the table(s)
    assert number_maxlimit >=1
    #XXX: implied minimum limit is 1, I don't think we can get rid of it, else we can't really use '1' in the list of numbers in the tables! and so it would be harder to add +1 to numbers to get to desired number...
    n=1
    nold=0
    goup=1
    wantedmax:float=number_maxlimit+1
    while (wantedmax > 1):
        wantedmax/=2
        nold+=1
    while (goup < number_maxlimit):
        goup*=2
        if(goup%2==0):
            goup+=1
        n+=1
    another_way=len(bin(number_maxlimit))-2 #credit to unkn2000 for this, 09Aug2022
    assert nold == another_way
    assert n == nold, f"{n}!={nold}"
    return n

def gen_for_maxlimit(number_maxlimit:int) -> None:
    n=get_power_for_maxlimit(number_maxlimit)
    print('max limit for',number_maxlimit,'is (2^',n,')-1=',2**n-1,
            '' if n==1 else ' or you could use a number <= than the previous max limit which is '
            + str( 1 if n==1 else 2**(n-1)-1 )
            + ' in order to get one less table.')
    try3_good(2**n-1)

#print("Boo:",len(bin(3000))-2)
#print("Boo:",len(bin(31))-2)
#
#exit()

try3_good(2**10-1)
gen_for_maxlimit(3)
gen_for_maxlimit(4)
gen_for_maxlimit(5)
gen_for_maxlimit(3000)
gen_for_maxlimit(4096)
gen_for_maxlimit(4097)
gen_for_maxlimit(24)
gen_for_maxlimit(31)
gen_for_maxlimit(32)
gen_for_maxlimit(36)
gen_for_maxlimit(2)
gen_for_maxlimit(1)
#gen_for_maxlimit(0)

#gen_for_maxlimit(31)
try3_good(17)
try3_good(31)

