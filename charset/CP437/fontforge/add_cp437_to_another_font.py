#!/usr/bin/fontforge

#TODO: ideally here, we'd copy/paste from chars from the 437 to our Cousine font and resize them too! and move them a bit, but PerfectDOSVGA437Unicode.ttf kinda already has all that, tho it doesn't look like Cousine for other chars.

#from fontforge import *
import fontforge  #this errors in vim but works when ran!
amb=fontforge.open("Perfect DOS VGA 437.ttf")               #Open a font
amb.selection.select(("ranges",None),"A","Z")    #select A-Z
amb.copy()                                       #Copy those glyphs into the clipboard

n=fontforge.font()                               #Create a new font
n.selection.select(("ranges",None),"A","Z")      #select A-Z of it
n.paste()                                        #paste the glyphs above in
print(n["A"].foreground)                          #test to see that something
                                                 #  actually got pasted
n.fontname="NewFont"                             #Give the new font a name
n.save("NewFont.sfd")

