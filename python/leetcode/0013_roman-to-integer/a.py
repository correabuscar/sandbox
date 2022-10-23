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

#i fkd up
def romanToInt(s:str) -> int:
    print(f"----------{s}")
    prev_roman:int=-1
    cur_roman:int=-1
    sub_mode:bool=False
    index:int=0
    result:int=0
    len_s=len(s)
    while index < len_s:
        cur_roman=ROMAN[s[index]]
        print(cur_roman)
        if prev_roman>0 and prev_roman < cur_roman:
            result-=prev_roman
            sub_mode=True
        else:
            if sub_mode:
                sub_mode=False
                print(f"{result}+={cur_roman}-{prev_roman}")
                result+=cur_roman-prev_roman
            else:
                print(f"{result}+={cur_roman}")
                result+=cur_roman
        index+=1

        prev_roman=cur_roman
    print(f"return {result}")
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

