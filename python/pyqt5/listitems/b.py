#!/usr/bin/python3
#doesn't work with pypy: ModuleNotFoundError: No module named 'PyQt5'
#!/usr/bin/pypy3

#src: https://stackoverflow.com/questions/42682544/pyqt5-listwidget-add-list-items

#FIXME: ugly workaround for: "Skipping analyzing 'PyQt5': found module but no type hints or library stubs" aka "Skipping analyzing X: found module but no type hints or library stubs", the following commented line is the workaround, as per src: https://mypy.readthedocs.io/en/stable/running_mypy.html
# type: ignore
from PyQt5 import QtGui, QtCore, QtWidgets
from PyQt5.QtWidgets import *
from PyQt5.QtGui import QKeyEvent, QKeySequence
from PyQt5.QtCore import Qt

import sys

global app

class Something(QListWidget):
    def __init__(self): #, parent: QWidget):
        #super(Something, self).__init__() #parent)
        #self._listWidget = QListWidget()
        super(Something, self).__init__()

        #rather then assume they aren't pressed on app startup, get their state
        current_modifiers = QtWidgets.QApplication.queryKeyboardModifiers() #ie. what's held down at the time of this call!
        #^ these work regardless of virtualkeys settings! ie. shift+alt will not be seen as Meta, so it correctly detects that they are held down!!
        self._alt_pressed:bool = current_modifiers & Qt.Key_Alt
        self._ctrl_pressed:bool = current_modifiers & Qt.Key_Control
        self._shift_pressed:bool = current_modifiers & Qt.Key_Shift
        #self._listWidget = super(Something, self).__init__()
        #src: https://doc.qt.io/qt-5/model-view-programming.html#using-drag-and-drop-with-item-views
        self.setSelectionMode(QtWidgets.QAbstractItemView.SingleSelection)
        self.setDragEnabled(True)
        #listWidget.viewport().setAcceptDrops(True)
        self.setDropIndicatorShown(True)

        #src: https://stackoverflow.com/questions/52873025/pyqt5-qlistview-drag-and-drop-creates-new-hidden-items/52877656#52877656
        self.setDragDropMode(QtWidgets.QListView.InternalMove)
        self.setAcceptDrops(True)
        self.setDefaultDropAction(QtCore.Qt.MoveAction)

        #listWidget.selectionModel().selectionChanged.connect(something)
        #QtCore.pyqtSignal().connect(on_key)
        self.keyPressEvent = self.on_key_press
        self.keyReleaseEvent = self.on_key_release

        self.setGeometry(QtCore.QRect(10, 10, 211, 291))


        ls = ['test', 'test2', 'test3']

        #listWidget.addItem('test')
        #listWidget.addItem('test2')
        #listWidget.addItem('test3')

        self.addItems(ls)
        self.show()

    def something(self):
        print(self)

    def on_key_release(self, event: QKeyEvent):
        assert event.type() == event.KeyRelease, event.type()
        modifiers = event.modifiers()
        if event.key() == Qt.Key_Alt:
            self._alt_pressed = False
        elif event.key() == Qt.Key_Control:
            self._ctrl_pressed = False
        elif event.key() == Qt.Key_Shift:
            self._shift_pressed = False
        super(Something,self).keyReleaseEvent(event)

    def on_key_press(self, event: QKeyEvent):
        assert event.type() == event.KeyPress, event.type()
        cur=self.currentItem()
        global_modifiers = QtWidgets.QApplication.keyboardModifiers()
        current_modifiers = QtWidgets.QApplication.queryKeyboardModifiers()
        #global app
        #modifiers = app.keyboardModifiers() #still broken in the same way as QtWidgets.QApplication.keyboardModifiers()
        modifiers = event.modifiers()
        #assert QtWidgets.QApplication.keyboardModifiers == event.modifiers #not true
        #assert QtWidgets.QApplication.keyboardModifiers() == event.modifiers() #not true always
        #alt_held = (modifiers == Qt.AltModifier)  #odd! The "&" operation won't work! ah because non zero(actually some object eg. <PyQt5.QtCore.Qt.KeyboardModifiers object at 0x7fd1f561fdd0>) doesn't equal True!!
        #alt_held = (modifiers & Qt.AltModifier) == Qt.AltModifier
        #alt_held:bool = (modifiers & Qt.AltModifier)
        if event.key() == Qt.Key_Alt:
            self._alt_pressed = True
        elif event.key() == Qt.Key_AltGr:  #on my keyboard it's RAlt, but it's not detected here via key()!
            print("-------AltGr")
        elif event.key() == Qt.Key_Control:
            self._ctrl_pressed = True
        elif event.key() == Qt.Key_Shift:
            self._shift_pressed = True
        elif event.key() == 16777250:
            self._shift_pressed = True
            self._alt_pressed = True
            #yes really, it's called Meta aka pressing and holding Shift then Alt key.
            #see: https://keycode.info/  to see that ^

        #FIXME: with event.modifiers(), when holding ctrl+shift+alt , alt isn't detected as held, but it's detected as press event(ie. when it's the third pressed key), wtf?! ok, LAlt and RAlt(true for Shift and Ctrl too) when pressed both, no Alt is detected, and Shift+Alt doesn't detect Shift but the prev. state (of either being held) remains. Things are more odd with QtWidgets.QApplication.keyboardModifiers() where two modifiers have to be pressed for any bool to show True !! Ok the problem is that when press+hold Key_Alt then Key_Shift (or in reverse order) then the latter event.key() == 0, which is VERY odd!  well of course it was because alt+shift was set to "Change layout option" aka keyboard layout in `xfce4-keyboard-settings`! so that fixes alt+shift combination! ok wait, something is still wrong: shift+alt gives 16777250 instead of 16777251 which is alt! it's not a pyqt5 issue though, since it happens inside `xfce4-keyboard-settings` also, in the input field; no other combinations give different event.key() values!
        #print(Qt.ShiftModifier, Qt.ControlModifier, Qt.AltModifier) #numbers!
        print(modifiers & Qt.ShiftModifier, modifiers & Qt.ControlModifier, modifiers & Qt.AltModifier)
        print(modifiers == Qt.ShiftModifier, modifiers == Qt.ControlModifier, modifiers == Qt.AltModifier)
        print(bool(modifiers & Qt.ShiftModifier), bool(modifiers & Qt.ControlModifier), bool(modifiers & Qt.AltModifier))
        print("shift=",self._shift_pressed, "ctrl=",self._ctrl_pressed, "alt=",self._alt_pressed)
        assert QtCore.Qt.AltModifier == Qt.AltModifier  #identic because I'm importing Qt from QtCode, heh
        print(cur.text(), modifiers, event)
        if event.key() == QtCore.Qt.Key_Escape or event.matches(QKeySequence.Quit):
            print("closing", event.key())
            self.close()
        elif event.matches(QKeySequence.Save):
            print("save")
        elif event.matches(QKeySequence.Copy):
            #selected= self.selectionModel().selectedIndexes()
            #assert len(selected) <= 1
            #if len(selected) == 1:
            #    firstselected=selected[0]
            #    row = firstselected.row()
            #else:
            #    row=0
            print("copy", cur.text())
        elif event.matches(QKeySequence.Cut):
            print("cut")
        elif event.matches(QKeySequence.Paste):
            print("paste")
        elif event.key() == Qt.Key_Down and self._alt_pressed:
            print("alt+down")


        #else:
        #    print("super")
        #    super(Something,self).keyPressEvent(event)
        print('key=',event.key(), "nativeScanCode=",event.nativeScanCode(), "nativeModifiers=", event.nativeModifiers(), "nativeVirtualKey=", event.nativeVirtualKey())
        print("super")
        super(Something,self).keyPressEvent(event)
        movedtoitem=self.currentItem()
        print("cursor moved to:", movedtoitem.text())
        print(modifiers & Qt.ShiftModifier, modifiers & Qt.ControlModifier, modifiers & Qt.AltModifier, modifiers & Qt.Key_Alt)
        #print(modifiers == Qt.ShiftModifier, modifiers == Qt.ControlModifier, modifiers == Qt.AltModifier) #obvious crap
        print(bool(modifiers & Qt.ShiftModifier), bool(modifiers & Qt.ControlModifier), bool(modifiers & Qt.AltModifier), bool(modifiers & Qt.Key_Alt))
        #if self._shift_pressed and self._ctrl_pressed:
        print('key=',event.key(), "nativeScanCode=",event.nativeScanCode(), "nativeModifiers=", event.nativeModifiers(), "nativeVirtualKey=", event.nativeVirtualKey())
        #XXX WARNING: this shows it only when the second key is pressed, ie. shows shift is true only when a is pressed in shift+a
        print(bool(global_modifiers & Qt.ShiftModifier), bool(global_modifiers & Qt.ControlModifier), bool(global_modifiers & Qt.AltModifier))
        print(bool(current_modifiers & Qt.ShiftModifier), bool(current_modifiers & Qt.ControlModifier), bool(current_modifiers & Qt.AltModifier))

if __name__ == '__main__':

    app = QApplication(sys.argv)

    i=Something()

    sys.exit(app.exec_())

#Keep this last:
#vim filetype is set to python below, otherwise vim will use tabs instead of spaces for indentation when pypy3 is interpreter
# vim: set ft=python
