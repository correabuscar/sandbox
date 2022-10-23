#!/usr/bin/env python3
#tstL1 rev. 3, python 3.8.10, win7, 21-22 may 2021

import random
#import os
from clrprint import *
import datetime
from tinydb import TinyDB,Query
from pprint import pprint

#print("Hello world\n");

#clrhelp()  # to see colors available
#user_input = clrinput('INPUT MESSAGE', clr='green')  # just like input()
#clrprint('YOURTEXT', user_input, clr='color')  # just like print()
#os.system('color')
min=1
max=2
showdiff:bool=False #if to show higher or lower if number is higher/lower than guessed one!

#print("A random number from list is : ", end="")
#print(random.choice([1, 4, 8, 10, 3]))
#print("A random number from range is : ", end="")
#print(random.randrange(1, 50, 1))

def anygood(left,right, rtrue,rfalse):
    if left == right:
        return rtrue
    else:
        return rfalse

def compute(the_list):
    good=0
    bad=0
    for record in the_list:
        #print(record)
        if True == record['wasgood']:
            good+=1
            assert record['user_input'] == record['random'], record
        else:
            bad+=1
            assert record['user_input'] != record['random'], record
    return (good,bad)

    
#print("A random number between 1 and 0 is : ", end="")
selectedone:int=-1 #global
selectedone_dt:datetime.datetime=datetime.datetime.now() #global
have_gen_one:bool=False #global
import sys
#max = sys.maxsize
session:int=random.randrange(1,sys.maxsize # this is sys.maxint now
                             ,1)
numfails:int=0
showdiff_was_true:int=0 #showdiff was true/false this number of times (which may be higher than the number of times user guessed a number, due to just showing prev. guess in diff context or not)
showdiff_was_false:int=0
#FIXME: ensure 'session' random number doesn't already exist in database! if it does, just choose another until it doesn't! ensure the prev and current ones are always different to avoid infinite loop tho!
def regen():
    global selectedone
    global selectedone_dt
    global have_gen_one
    global numfails
    global showdiff_was_true
    global showdiff_was_false
    selectedone=random.randrange(min, max+1, 1)
    selectedone_dt= datetime.datetime.now()
    #print(type(selectedone_dt)) #datetime.datetime
    print("A new number was generated")
    have_gen_one=True
    numfails=0
    showdiff_was_true=0
    showdiff_was_false=0


#clrprint('tex_clr1','tex_clr2',clr=['r','g'])
while True:
    if not have_gen_one:
        regen()

    clrprint('Please guess number between',min,'and',max,'(q=quit): ',clr=['d','y','d','y','d'])
    bypass:bool=False #whether or not to bypass actually asking user for guess number, if True doesn't count as guess just redisplays
    try:
        str=clrinput('')
        processing=str.lower()
        
        if 'q' == processing:
            print("Quitting")
            break
        elif 'w' == processing:
            print(selectedone)
            have_gen_one=False
            continue
        elif 'n' == processing:
            #regen a new one
            have_gen_one=False
            continue
        elif 'd' == processing:
            #toggle showdiff
            showdiff=not showdiff
            print("showdiff is now",showdiff)
            try:
                user_input
            except NameError:
                #not yet defined, not bypassing, re-asking instead!
                continue
            else:
                #it's defined, can bypass
                bypass=True
        if not bypass:
            user_input = int(str)
            user_input_dt = datetime.datetime.now()
    except Exception as e:
        print("Invalid number!",e)
        pass
        continue
    if user_input in range(min,max+1):
        wasgood=anygood(user_input,selectedone,True,False)
        goodcolor=anygood(user_input,selectedone,'g','r')
        
        if ((max-min==1) or wasgood):
            whattoshow=selectedone
        elif showdiff:
            if selectedone > user_input:
                whattoshow='higher'
            else:
                whattoshow='lower'
        else:
            whattoshow='keep guessing'
        clrprint('You said:', user_input, ' selected was:', whattoshow,' ', clr=['d',goodcolor,'d','g','d'],end="")
        if wasgood:
            clrprint('OK',clr='g')
        else:
            if not bypass:
                numfails+=1
            clrprint('FAIL','(',numfails,')',clr=['r','d','b','d'])
        #break
        if showdiff:
            showdiff_was_true+=1
        else:
            showdiff_was_false+=1
        print(f"Showdiff, true={showdiff_was_true}, false={showdiff_was_false}")
        if not bypass:
            with TinyDB('mytstdb.json') as db:
                #table=db.table('wtwtable')
                db.insert({'session':session, 'wasgood': wasgood, 'random': selectedone, 'min': min, 'max': max,
                              'user_input': user_input, 'rndtime': selectedone_dt.strftime("%Y-%m-%d %H:%M:%S.%f"),
                              'guesstime':user_input_dt.strftime("%Y-%m-%d %H:%M:%S.%f UTC%z"),
                              'showdiff_now':showdiff, 'showdiff_was_true':showdiff_was_true,
                              'showdiff_was_false':showdiff_was_false,
                              'failssofarforthisnumberincludingcurrenttry': numfails})
            #anew:
            if wasgood or (not wasgood and max-min <=1):
                #gen new number if failed and had only two choices
                #or if was good guess
                have_gen_one=False
            
    else:
        clrprint("Number should be between",min,'and',max,', so', user_input, 'is not valid', clr=['d','y','d','y','d','r','d'])



with TinyDB('mytstdb.json') as db:
    #table=db.table('wtwtable')

    #session pie
    import matplotlib.pyplot as plt
    labels = ['good guess', 'miss']
    colors = ['green', 'red']
    plt.axis('equal')
    
    this_session = Query()
    res=db.search(this_session.session == session)
    if len(res) > 0:
        #print(session,res, flush=True)
        #quit()
        
        good,bad=compute(res)
        sizes = [ good, bad ]
        print(f"This session({session}) GOOD: {good}, FAILS: {bad}")
        pprint(res)
        plt.pie(sizes, labels=labels, colors=colors, autopct='%1.1f%%', shadow=True, startangle=140)
        plt.title(f"This session ({session})")
        plt.show(block = True)

    #alltime pie
    #print()
    the_list=db.all()
    #pprint(the_list)
    good,bad=compute(the_list)
    print(f"Alltime GOOD: {good}, FAILS: {bad}")
    sizes = [ good, bad ]
    plt.pie(sizes, labels=labels, colors=colors, autopct='%1.1f%%', shadow=True, startangle=140)
    plt.title("All time")
    plt.show(block = False)
    #import json
    # Create Python object from JSON string data
    #obj = json.loads(table.all())
    #json_formatted_str = json.dumps(obj, indent=4)
    #print(json_formatted_str)


