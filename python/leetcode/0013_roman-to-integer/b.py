#!/usr/bin/python3

#https://leetcode.com/problems/roman-to-integer/

ROMAN={
'I':             1,
'V':             5,
'X':             10,
'L':             50,
'C':             100,
'D':             500,
'M':             1000,
        }

#this was easy (used Hint 1 which I accidentally opened when exploring but a.py was done in spite of it)
def romanToInt(s:str) -> int:
    #print(f"----------{s}")
    prev_roman:int=-1
    cur_roman:int=-1
    index:int=0
    result:int=0
    #assuming non-empty string because problem assumes 's' is valud roman numeral
    index=len(s)-1
    while index >= 0:
        cur_roman=ROMAN[s[index]]
        #print(cur_roman)
        if prev_roman>0 and cur_roman < prev_roman:
            result-=cur_roman
        else:
            result+=cur_roman
        index-=1

        prev_roman=cur_roman
    #print(f"return {result}")
    return result

print(romanToInt("III"))
assert 3 == romanToInt("III")
print(romanToInt("LVIII"))
assert 58 == romanToInt("LVIII")
print(romanToInt("MCMXCIV"))
assert 1994 == romanToInt("MCMXCIV")
#however the problem assumes the roman numeral is guaranteed to be valid! so the following won't fail hmm, this sux.
#romanToInt("IIV") #should fail
#romanToInt("IIII") #should fail, IV is ok tho
#romanToInt("VV") #should fail
#romanToInt("IVX") #should fail, XIV would work.

assert 200 == romanToInt("CC") #200

