def isTrue(x): return x == 1

def if_(i, y, z): return y if isTrue(i) else z

def and_(i, j): return if_(i, j, 0)
def AND(listi, listj): return [and_(i, j) for i, j in zip(listi,listj)]

def not_(i): return if_(i, 0, 1)
def NOT(listi): return [not_(x) for x in listi]

#return true if either i or j is true but not both at the same time:W
def xor(i, j): return if_(i, not_(j), j)
def XOR(listi, listj): return [xor(i, j) for i, j in zip(listi, listj)]

#if number of truth values is odd then return true
def xorxor(i, j, l): return xor(i, xor(j, l))
def XORXOR(listi, listj, listl): return [xorxor(i, j, l) for i, j, l, in zip(listi, listj, listl)]

#get the majority of results, i.e., if 2 or more of three values are the same
def maj(i,j,k): return max([i,j,], key=[i,j,k].count)

def rotr(x, n): return x[-n:] + x[:-n]

def shr(x, n): return n * [0] + x[:-n]

def add(i, j):
  length = len(i)
  sums = list(range(length))
  c = 0
  for x in range(length-1,-1,-1):
    sums[x] = xorxor(i[x], j[x], c)
    c = maj(i[x], j[x], c)
  return sums

