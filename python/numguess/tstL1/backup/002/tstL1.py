#tstL1 rev. 2, python 3.8.10, win7

import random
#import os
from clrprint import *
import datetime
from tinydb import TinyDB,Query
from pprint import pprint

#class bcolors:
    #HEADER = '\033[95m'
    #RED = '\033[91m]'
    #OKBLUE = '\033[94m'
    #OKCYAN = '\033[96m'
    #OKGREEN = '\033[92m'
    #WARNING = '\033[93m'
    #FAIL = '\033[91m'
    #ENDC = '\033[0m'
    #BOLD = '\033[1m'
    #UNDERLINE = '\033[4m'

#def red(txt):
    #return f"{bcolors.RED}{txt}{bcolors.ENDC}"
    
#def green(txt):
    #return f"{bcolors.OKGREEN}{txt}{bcolors.ENDC}"


#print("Hello world\n");

#clrhelp()  # to see colors available
#user_input = clrinput('INPUT MESSAGE', clr='green')  # just like input()
#clrprint('YOURTEXT', user_input, clr='color')  # just like print()
#os.system('color')
min=0
max=1
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
import sys
#max = sys.maxsize
session:int=random.randrange(1,sys.maxsize # this is sys.maxint now
                             ,1)
#FIXME: ensure 'session' random number doesn't already exist in database! if it does, just choose another until it doesn't! ensure the prev and current ones are always different to avoid infinite loop tho!
def regen():
    global selectedone
    global selectedone_dt
    selectedone=random.randrange(min, max+1, 1)
    selectedone_dt= datetime.datetime.now()
    #print(type(selectedone_dt)) #datetime.datetime
    print("A new number was generated")

regen()

#clrprint('tex_clr1','tex_clr2',clr=['r','g'])
while True:
    clrprint('Please guess number between',min,'and',max,'(q=quit): ',clr=['d','y','d','y','d'])
    try:
        str=clrinput('')
        if 'q' == str.lower():
            print("Quitting")
            break
        user_input = int(str)
        user_input_dt = datetime.datetime.now()
    except Exception as e:
        print("Invalid number!",e)
        pass
        continue
    if user_input in [min,max]:
        wasgood=anygood(user_input,selectedone,True,False)
        goodcolor=anygood(user_input,selectedone,'g','r')
        clrprint('You said:', user_input, ' selected was:', selectedone,' ', clr=['d',goodcolor,'d','g','d'],end="")
        if wasgood:
            clrprint('OK',clr='g')
        else:
            clrprint('FAIL',clr='r')
        #break
        with TinyDB('mytstdb.json') as db:
            #table=db.table('wtwtable')
            db.insert({'session':session, 'wasgood': wasgood, 'random': selectedone, 'min': min, 'max': max,
                          'user_input': user_input, 'rndtime': selectedone_dt.strftime("%Y-%m-%d %H:%M:%S.%f"),
                          'guesstime':user_input_dt.strftime("%Y-%m-%d %H:%M:%S.%f UTC%z")})
        #anew:
        regen()
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
        plt.pie(sizes, labels=labels, colors=colors, autopct='%1.1f%%', shadow=True, startangle=140)
        plt.title(f"This session ({session})")
        plt.show(block = True)

    #alltime pie
    #print()
    the_list=db.all()
    #pprint(the_list)
    good,bad=compute(the_list)
    sizes = [ good, bad ]
    plt.pie(sizes, labels=labels, colors=colors, autopct='%1.1f%%', shadow=True, startangle=140)
    plt.title("All time")
    plt.show(block = False)
    #import json
    # Create Python object from JSON string data
    #obj = json.loads(table.all())
    #json_formatted_str = json.dumps(obj, indent=4)
    #print(json_formatted_str)


