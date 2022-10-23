#!/usr/bin/python3 -bb
#using python-pyqt5 5.14.2-1 in ArchLinux

#doesn't work with pypy: ModuleNotFoundError: No module named 'PyQt5'
#!/usr/bin/pypy3

#src: https://stackoverflow.com/questions/42682544/pyqt5-listwidget-add-list-items

#FIXME: ugly workaround for: "Skipping analyzing 'PyQt5': found module but no type hints or library stubs" aka "Skipping analyzing X: found module but no type hints or library stubs", the following commented line is the workaround, as per src: https://mypy.readthedocs.io/en/stable/running_mypy.html
# type: ignore
from PyQt5 import QtGui, QtCore, QtWidgets
from PyQt5.QtWidgets import *
from PyQt5.QtGui import QKeyEvent, QKeySequence, QFont
from PyQt5.QtCore import Qt, QTimer
from typing import Any

import sys
from enum import Enum
import signal

global app
global win

GRUB_CFG_TEXT:Qt.ItemDataRole = Qt.UserRole+1  #technically doesn't have to be +1
NEW_MENUENTRY_NUMBER:Qt.ItemDataRole = Qt.UserRole+2
ORIGINAL_MENUENTRY_NUMBER:Qt.ItemDataRole = Qt.UserRole+3
MENUENTRY_TITLE:Qt.ItemDataRole = Qt.UserRole+4
GRUB_CFG_FILENAME: str = "/boot/grub/grub.cfg"

#well ok I don't need my own subclassed QListWidgetItem, I can just use setData and roles
class MyItem(QListWidgetItem):
    def __init__(self):
        super(MyItem, self).__init__(type=QListWidgetItem.UserType)
    def getData(self, role: Qt.ItemDataRole) -> Any:
        return super(MyItem, self).data(role)

class MyQListWidget(QListWidget):
    def recompute_item(self, item: MyItem) -> None:
        item.setText(f"{item.getData(NEW_MENUENTRY_NUMBER)}: {item.getData(MENUENTRY_TITLE)}")
        font=item.font()
        omen=item.getData(ORIGINAL_MENUENTRY_NUMBER)
        assert type(omen) == int, f"{type(omen)} {omen}"
        nmen=item.getData(NEW_MENUENTRY_NUMBER)
        assert type(nmen) == int, f"{type(nmen)} {nmen}"
        moved_from_orignal_location= (omen != nmen)
        font.setBold(moved_from_orignal_location)
        if moved_from_orignal_location:
            item.setForeground(Qt.green)
        else:
            item.setForeground(Qt.black)
        item.setFont(font)

    def recompute_items(self) -> None:
        me_idx=0
        for idx in range(self.count()):
            item=self.item(idx)
            #if not item.isHidden():
            if None != item.data(ORIGINAL_MENUENTRY_NUMBER): #aka it's a ME, as opposed to a non-ME
                me_idx+=1
                #print(idx, item.isHidden())
                #print(item.data(GRUB_CFG_TEXT))
                item.setData(NEW_MENUENTRY_NUMBER, me_idx)
                self.recompute_item(item)

    def save_items(self) -> bool:
        with open(GRUB_CFG_FILENAME,"w") as f:
            for idx in range(self.count()):
                item=self.item(idx)
                data=item.getData(GRUB_CFG_TEXT)
                f.write(data)

        return True

    #last_captured_nonme is changed within the function and thus reflected outside, on the caller!
    def non_ME(self, last_captured_nonme: str) -> str:
        if "" != last_captured_nonme:
            #need to write out the so far collected non-ME block:
            item=MyItem()
            item.setData(GRUB_CFG_TEXT, last_captured_nonme)
            size=len(last_captured_nonme)
            item.setText(f"/// {size} bytes coalesced non-MENUENTRY grub code from grub.cfg")
            item.setData(Qt.ToolTipRole, last_captured_nonme)
            item.setForeground(Qt.gray)
            self.addItem(item)
            #item.setHidden(True)
            #assert item.isHidden()
            return ""
        else:
            return last_captured_nonme
    def __init__(self, parent: QWidget=None):
        #super(MyQListWidget, self).__init__() #parent)
        #self._listWidget = QListWidget()
        super(MyQListWidget, self).__init__(parent)

        #Rather then assume they aren't pressed on app startup, get their state
        current_modifiers = QtWidgets.QApplication.queryKeyboardModifiers() #ie. what's held down at the time of this call!
        #^ these work regardless of virtualkeys settings! ie. shift+alt will not be seen as Meta, so it correctly detects that they are held down!!
        self._alt_helddown:bool = bool(current_modifiers & Qt.AltModifier) #the bool cast is needed, odly enough!
        self._ctrl_helddown:bool = bool(current_modifiers & Qt.ControlModifier)
        self._shift_helddown:bool = bool(current_modifiers & Qt.ShiftModifier)
        #the following are used to build the above:
        self._lalt_helddown:bool = False
        self._lctrl_helddown:bool = False
        self._lshift_helddown:bool = False
        self._ralt_helddown:bool = False
        self._rctrl_helddown:bool = False
        self._rshift_helddown:bool = False

        #self._listWidget = super(MyQListWidget, self).__init__()
        #src: https://doc.qt.io/qt-5/model-view-programming.html#using-drag-and-drop-with-item-views
        self.setSelectionMode(QtWidgets.QAbstractItemView.SingleSelection)
        self.setDragEnabled(True)
        #listWidget.viewport().setAcceptDrops(True)
        self.setDropIndicatorShown(True)

        #src: https://stackoverflow.com/questions/52873025/pyqt5-qlistview-drag-and-drop-creates-new-hidden-items/52877656#52877656
        self.setDragDropMode(QtWidgets.QListView.InternalMove)
        #self.setDragDropMode(QtWidgets.QListView.DragOnly)
        self.setAcceptDrops(True)
        self.setDefaultDropAction(QtCore.Qt.MoveAction)

        #listWidget.selectionModel().selectionChanged.connect(something)
        #QtCore.pyqtSignal().connect(on_key)
        self.keyPressEvent = self.on_key_press
        self.keyReleaseEvent = self.on_key_release
        self.closeEvent = self.on_close_attempt
        #self.quitEvent = self.on_close_attempt #no effect

        self.setGeometry(QtCore.QRect(410, 210, 511, 491))

        #src: https://doc.qt.io/qt-5/qabstractitemview.html#alternatingRowColors-prop
        self.setSortingEnabled(False)
        self.setAlternatingRowColors(True)
        self.setAutoScroll(True)
        self.setDragDropOverwriteMode(False)
        self.setTabKeyNavigation(True) #Tab and shift+tab acts like DownArrow key and UpArrow key
        self.setTextElideMode(Qt.ElideNone) #this doesn't matter for this program

        #TODO: add the grub.cfg lines into an external list and only refer to its indexes in the self list
        #done: or make blocks of non MEs, ie. coalesced or collapsed into one item
        #done: add non MEs as items and setHidden(True), see if alt+up moves it, fix it if so
        first_me_item=None
        with open(GRUB_CFG_FILENAME, "r") as f:
            inme=False
            me_level=0
            last_captured_me=""
            mes: List[str]=[]
            count=0
            last_captured_nonme=""
            for line in f:
                sline=line.strip()
                if sline.startswith("menuentry "):
                    assert sline.endswith("{"), sline
                    inme=True
                    me_level+=1
                    assert me_level <=1, f"{me_level} {line}" #for now not expecting a bigger than 1 level, so if found more coding may be needed!
                    #self.non_ME(last_captured_nonme) #changes value of last_captured_nonme! nvm, doesn't work for str, int etc, see: https://www.tutorialspoint.com/how-are-arguments-passed-by-value-or-by-reference-in-python
                    last_captured_nonme=self.non_ME(last_captured_nonme)
                    #if "" != last_captured_nonme:
                    #    #need to write out the so far collected non-ME block:
                    #    item=MyItem()
                    #    item.setData(GRUB_CFG_TEXT, last_captured_nonme)
                    #    size=len(last_captured_nonme)
                    #    item.setText(f"{size} bytes coalesced non-MENUENTRY grub code from grub.cfg")
                    #    item.setData(Qt.ToolTipRole, last_captured_nonme)
                    #    self.addItem(item)
                    #    #item.setHidden(True)
                    #    #assert item.isHidden() #done: this fails! unless it's after self.addItem()!
                    #    #XXX: ok, new strategy, don't hide these because we lose contextual position of the MEs after move
                    #    last_captured_nonme=""
                elif not inme:
                    last_captured_nonme+=line

                # pause, not connected to the above 'if's
                if inme:
                    last_captured_me+=line
                    if sline=="}":
                        me_level-=1
                        assert me_level >= 0, f"{me_level} {line}"
                        assert me_level == 0, f"{me_level} {line}" #for now not expecting a bigger than 1 level, so if found more coding may be needed!
                        if me_level == 0:
                            inme=False
                            mes.append(last_captured_me)
                            count+=1
                            item=MyItem()
                            #first_line_of_lcme=last_captured_me.partition('\n')[0]
                            #just_the_me_title=
                            try:
                                import re
                                #found = re.search('^menuentry [\'"]{1}([^\'"]+)[\'"]{1}.+{$', first_line_of_lcme)
                                found = re.search('^menuentry [\'"]{1}([^\'"]+)[\'"]{1}.+{$', last_captured_me, re.MULTILINE)
                                just_the_me_title = found.group(1)  #this will throw, if unexpected shie
                                #print(found.group(0), found.group(1))
                            except:
                                #print(first_line_of_lcme)
                                print(last_captured_me)
                                raise
                            item.setData(GRUB_CFG_TEXT, last_captured_me)
                            item.setData(NEW_MENUENTRY_NUMBER, count)
                            item.setData(ORIGINAL_MENUENTRY_NUMBER, count)
                            item.setData(MENUENTRY_TITLE, f"{just_the_me_title}")
                            self.recompute_item(item)
                            item.setData(Qt.ToolTipRole, last_captured_me)
                            #item.setData(Qt.StatusTipRole, f"originally in the position number {count}") #cannot be shown unless win.show() is used
                            self.addItem(item)
                            if None == first_me_item:
                                first_me_item = item
                            last_captured_me=""

                assert me_level >= 0
            #done: this code block is duplicated! dedupe it!
            #outside of 'for', for the last non-ME block/line(s):
            self.non_ME(last_captured_nonme)

        #i=0
        #for each_me in mes:
        #    i+=1
        #    print(i, each_me)

        ##ls = ['test4', 'test2', 'test3', "hjskhfkgweiyfgowefhoewhfwoefowe"]
        #item = MyItem()
        ##font = QtGui.QFont()
        ##font.setBold(True)
        ##font.setWeight(75) #TODO what is this?
        #item.setText("blah0")
        ##item.setData(Qt.UserRole+1,"meh")
        ##item.setData(Qt.UserRole+2,"beh")
        #item.setData(GRUB_CFG_TEXT,"beh")
        #self.addItem(item)
        ##listWidget.addItem('test2')
        ##listWidget.addItem('test3')

        #self.addItems(ls)

        #font = QtGui.QFont("Arial", 13, QFont.Regular)
        font = QtGui.QFont("Arial", 13)
        self.setFont(font)
        #self.setCurrentRow(0)
        self.setCurrentItem(first_me_item)
        self.scrollToItem(self.currentItem(), QtWidgets.QAbstractItemView.PositionAtTop)
        #TODO: save/restore last row position, as current.

        #self.show()

    #def something(self):
    #    print(self)

    def on_close_attempt(self,event) -> None:
        print("close event") #, event)
        cd = QMessageBox()
        cd.setText(
            "Save before quit?")
        #TODO: detect when modified and only then Present the Save button?
        #cd.setStandardButtons(
        #    QMessageBox.Save | QMessageBox.Close | QMessageBox.Cancel,
        #    )
        #    #QMessageBox.Save)
        cd.setIcon(QMessageBox.Warning)
        close_button = QPushButton("&Close")
        #close_button.setDefault(True)
        save_button = QPushButton("&Save")
        save_button.setDefault(True)
        cancel_button = QPushButton("C&ancel")
        cd.addButton(close_button, QtWidgets.QMessageBox.RejectRole)
        cd.addButton(save_button, QtWidgets.QMessageBox.AcceptRole)
        cd.addButton(cancel_button, QtWidgets.QMessageBox.RejectRole)
        cd.setEscapeButton(cancel_button)
        cd.setDefaultButton(save_button) #redundant?!
        #reply = cd.exec() #int value when custom buttons are used
        cd.exec()
        #reply=cd.result()
        reply=cd.clickedButton()

        #if reply == QMessageBox.Close:
        if reply == close_button:
            print("close accepted")
            event.accept()
        #elif reply == QMessageBox.Save:
        elif reply == save_button:
            print("save&quit")
            #app.quit()
            #done: accept only if save was successful
            if self.save_items():
                event.accept()
            else:
                print("Save failed! Retry?")
                event.ignore()
        #elif reply == QMessageBox.Cancel:
        elif reply == cancel_button:
            print("cancelled, don't quit")
            event.ignore()
        else:
            raise Exception(f"impossible {reply}")

    def recompute_modifiers(self) -> None:
        self._alt_helddown = self._lalt_helddown | self._ralt_helddown
        self._ctrl_helddown = self._lctrl_helddown | self._rctrl_helddown
        self._shift_helddown = self._lshift_helddown | self._rshift_helddown

    #def dragMoveEvent(self, event):
    #    print("dragMoveEvent")
    #    event.accept()

    #def dragEnterEvent(self, event):
    #    print("dragEnterEvent")
    #    event.accept()

    #step can be positive or negative, usually +-1
    def find_row(self, fro:int,step:int=1) -> int:
        which_row=fro
        length=self.count()
        while True:
        #for which_row in range(fro+step,step, step):
            if which_row < 0 or which_row >= length:
                print(f"hit range 0 <= {which_row} < {length}")
                break
            item_at_row=self.item(which_row)
            if None == item_at_row:
                print("hit none at index {which_row}")
                break
            #elif not item_at_row.isHidden(): #self.isItemHidden(item_at_row):
            elif None != item_at_row.data(ORIGINAL_MENUENTRY_NUMBER): #aka it's a ME, as opposed to a non-ME
                return which_row
            which_row+=step
        return -1


    def dropEvent(self, event):
        #print("dropEvent", self.count())
        #event.accept() #calling this and then super() will delete an item on each call!
        super(MyQListWidget, self).dropEvent(event)
        self.on_move()

    def on_move(self):
        #print("on_move pre", self.count())
        self.recompute_items()
        #print("on_move aft", self.count())

    def on_key_release(self, event: QKeyEvent):
        assert event.type() == event.KeyRelease, event.type()
        modifiers = event.modifiers()
        if event.nativeScanCode() == 64: #LAlt
            self._lalt_helddown = False
        elif event.nativeScanCode() == 37: #LCtrl
            self._lctrl_helddown = False
        elif event.nativeScanCode() == 50: #LShift
            self._lshift_helddown = False
        elif event.nativeScanCode() == 62: #RShift
            self._rshift_helddown = False
        elif event.nativeScanCode() == 105: #RCtrl
            self._rctrl_helddown = False
        elif event.nativeScanCode() == 108: #RAlt
            self._ralt_helddown = False
        self.recompute_modifiers()
        super(MyQListWidget,self).keyReleaseEvent(event)
        #print("shift=",self._shift_helddown, "ctrl=",self._ctrl_helddown, "alt=",self._alt_helddown)

    def on_key_press(self, event: QKeyEvent):
        assert event.type() == event.KeyPress, event.type()
        #global_modifiers = QtWidgets.QApplication.keyboardModifiers()
        #current_modifiers = QtWidgets.QApplication.queryKeyboardModifiers()
        #global app
        #modifiers = app.keyboardModifiers() #still broken in the same way as QtWidgets.QApplication.keyboardModifiers()
        modifiers = event.modifiers()
        #assert QtWidgets.QApplication.keyboardModifiers == event.modifiers #not true
        #assert QtWidgets.QApplication.keyboardModifiers() == event.modifiers() #not true always
        #alt_held = (modifiers == Qt.AltModifier)  #odd! The "&" operation won't work! ah because non zero(actually some object eg. <PyQt5.QtCore.Qt.KeyboardModifiers object at 0x7fd1f561fdd0>) doesn't equal True!!
        #alt_held = (modifiers & Qt.AltModifier) == Qt.AltModifier
        #alt_held:bool = (modifiers & Qt.AltModifier)
        #if event.key() == Qt.Key_Alt:
        #    self._alt_helddown = True
        #elif event.key() == Qt.Key_AltGr:  #on my keyboard it's RAlt, but it's not detected here via key()!
        #    print("-------AltGr")
        #elif event.key() == Qt.Key_Control:
        #    self._ctrl_helddown = True
        #elif event.key() == Qt.Key_Shift:
        #    self._shift_helddown = True
        #elif event.key() == 16777250:
        #    self._shift_helddown = True
        #    self._alt_helddown = True
        #    #yes really, it's called Meta aka pressing and holding Shift then Alt key.
        #    #see: https://keycode.info/  to see that ^
        if event.nativeScanCode() == 64: #Alt
            self._lalt_helddown = True
        elif event.nativeScanCode() == 37: #Ctrl, or Lshift+Rshift+a  yes, odd!
            self._lctrl_helddown = True
        elif event.nativeScanCode() == 50: #Shift
            self._lshift_helddown = True
        elif event.nativeScanCode() == 62: #RShift
            self._rshift_helddown = True
        elif event.nativeScanCode() == 105: #RCtrl
            self._rctrl_helddown = True
        elif event.nativeScanCode() == 108: #RAlt
            self._ralt_helddown = True
        self.recompute_modifiers()

        cur=self.currentItem()
        cur_row=self.currentRow()
        key=event.key()
        #mehFIXME: with event.modifiers(), when holding ctrl+shift+alt , alt isn't detected as held, but it's detected as press event(ie. when it's the third pressed key), wtf?! ok, LAlt and RAlt(true for Shift and Ctrl too) when pressed both, no Alt is detected, and Shift+Alt doesn't detect Shift but the prev. state (of either being held) remains. Things are more odd with QtWidgets.QApplication.keyboardModifiers() where two modifiers have to be pressed for any bool to show True !! Ok the problem is that when press+hold Key_Alt then Key_Shift (or in reverse order) then the latter event.key() == 0, which is VERY odd!  well of course it was because alt+shift was set to "Change layout option" aka keyboard layout in `xfce4-keyboard-settings`! so that fixes alt+shift combination! ok wait, something is still wrong: shift+alt gives 16777250 instead of 16777251 which is alt! it's not a pyqt5 issue though, since it happens inside `xfce4-keyboard-settings` also, in the input field; no other combinations give different event.key() values!
        #print(Qt.ShiftModifier, Qt.ControlModifier, Qt.AltModifier) #numbers!
        #print(modifiers & Qt.ShiftModifier, modifiers & Qt.ControlModifier, modifiers & Qt.AltModifier)
        #print(modifiers == Qt.ShiftModifier, modifiers == Qt.ControlModifier, modifiers == Qt.AltModifier)
        #print(bool(modifiers & Qt.ShiftModifier), bool(modifiers & Qt.ControlModifier), bool(modifiers & Qt.AltModifier))
        #print("shift=",self._shift_helddown, "ctrl=",self._ctrl_helddown, "alt=",self._alt_helddown)
        assert QtCore.Qt.AltModifier == Qt.AltModifier  #identic because I'm importing Qt from QtCode, heh
        #print(cur, cur.text(), modifiers, event)
        was_moved=False
        no_super=False
        if event.key() == QtCore.Qt.Key_Escape or event.matches(QKeySequence.Quit):
            print("closing by key event", event.key())
            self.close()
            #keep_alive()
            #app.quit() #bypasses the close event!
            #reached when cancelling the close!
            no_super=True
        elif event.matches(QKeySequence.Save):
            print("save")
            #done: save
            if self.save_items():
                event.accept()
            else:
                print("Save failed! Retry?")
                event.ignore()
            no_super=True
        elif event.matches(QKeySequence.Copy):
            #selected= self.selectionModel().selectedIndexes()
            #assert len(selected) <= 1
            #if len(selected) == 1:
            #    firstselected=selected[0]
            #    row = firstselected.row()
            #else:
            #    row=0
            print("copy", cur.text())
            #TODO:
        elif event.matches(QKeySequence.Cut):
            print("cut")
            #TODO:
        elif event.matches(QKeySequence.Paste):
            print("paste")
            #TODO:
        elif self._alt_helddown:
            if key == Qt.Key_Down:
                #print("alt+down")
                #TODO: find the row to move it to, must be in place of an already existing ME aka menuentry; OR, hide the items that aren't supposed to be seen!
                #TODO: do the finding of right row for drag&drop too!
                #move_to_row=cur_row + 1
                move_to_row=self.find_row(cur_row+1, +1)
                if -1 != move_to_row:
                    taken=self.takeItem(cur_row)
                    self.insertItem(move_to_row, taken)
                    was_moved=True
                else:
                    print("no move down")
                    no_super=True
                #TODO: maybe put this was_moved detection in an event?
            elif key == Qt.Key_Up:
                #print("alt+up")
                #move_to_row=cur_row - 1
                move_to_row=self.find_row(cur_row-1, -1)
                if -1 != move_to_row:
                    taken=self.takeItem(cur_row)
                    self.insertItem(move_to_row, taken)
                    #self.setCurrentRow(cur_row) #only needed for up to not end up on second row due to takeItem
                    #self.setCurrentRow(move_to_row)
                    #font=taken.font()
                    #font.setBold(True)
                    #taken.setFont(font)
                    was_moved=True
                else:
                    print("no move up")
                    no_super=True
        elif event.nativeScanCode() in [133,135]: #LWinKey, RMenuKey(this one repeats when held!)
            #global win
            #win.showToolTip()
            #QtWidgets.QToolTip.hideText()
            #QtWidgets.QToolTip.showText(QtGui.QCursor.pos(), "message")
            #done: find out how to make it show current item's tooltip? hmm wait
            if QtWidgets.QToolTip.isVisible():
                QtWidgets.QToolTip.hideText()
            else:
                QtWidgets.QToolTip.showText(QtGui.QCursor.pos(), cur.toolTip())
            #print("ah")
            no_super=True


        #else:
        #    print("super")
        #    super(MyQListWidget,self).keyPressEvent(event)
        #print('key=',event.key(), "nativeScanCode=",event.nativeScanCode(), "nativeModifiers=", event.nativeModifiers(), "nativeVirtualKey=", event.nativeVirtualKey())
        if __debug__:
            beforesuper_key=event.key()
            beforesuper_nsc=event.nativeScanCode()
            beforesuper_nm=event.nativeModifiers()
            beforesuper_nvk=event.nativeVirtualKey()
        #import time
        #time.sleep(1)
        #print("super")
        if not no_super:
            super(MyQListWidget,self).keyPressEvent(event)
        assert event.key() == beforesuper_key, f"{event.key()}!={beforesuper_key}"
        assert event.nativeScanCode() == beforesuper_nsc, f"{event.nativeScanCode()}!={beforesuper_nsc}"
        assert event.nativeModifiers() == beforesuper_nm, f"{event.nativeModifiers()}!={beforesuper_nm}"
        assert event.nativeVirtualKey() == beforesuper_nvk, f"{event.nativeVirtualKey()}!={beforesuper_nvk}"

        #XXX: probably must be after super():
        if was_moved:
            self.on_move()
            self.setCurrentRow(move_to_row)

        #XXX: must be after super():
        movedtoitem=self.currentItem()
        #print("cursor moved to:", movedtoitem.text())
        #print("data of first user role=", movedtoitem.data(GRUB_CFG_TEXT))
        #print(modifiers & Qt.ShiftModifier, modifiers & Qt.ControlModifier, modifiers & Qt.AltModifier, modifiers & Qt.Key_Alt)
        #print(modifiers == Qt.ShiftModifier, modifiers == Qt.ControlModifier, modifiers == Qt.AltModifier) #obvious crap
        #print(bool(modifiers & Qt.ShiftModifier), bool(modifiers & Qt.ControlModifier), bool(modifiers & Qt.AltModifier), bool(modifiers & Qt.Key_Alt))
        #if self._shift_helddown and self._ctrl_helddown:
        #print('key=',event.key(), "nativeScanCode=",event.nativeScanCode(), "nativeModifiers=", event.nativeModifiers(), "nativeVirtualKey=", event.nativeVirtualKey())
        #XXX WARNING: the following shows it only when the second key is pressed, ie. shows shift is true only when a is pressed in shift+a
        #print(bool(global_modifiers & Qt.ShiftModifier), bool(global_modifiers & Qt.ControlModifier), bool(global_modifiers & Qt.AltModifier))
        #print(bool(current_modifiers & Qt.ShiftModifier), bool(current_modifiers & Qt.ControlModifier), bool(current_modifiers & Qt.AltModifier))

#def keep_alive():
#    global app
#    try:
#        app.lastWindowClosed.disconnect(keep_alive)
#    except TypeError as e:
#        print(e)
#        pass
#    global win
#    #win.setVisibility(QtGui.QWindow.Minimized) #win must be QWindow type here, not QMainWindow
#    #print(win)
#    reply = QMessageBox.question(
#            win, "Message",
#            "Are you sure you want to quit? Any unsaved work will be lost.",
#            QMessageBox.Save | QMessageBox.Close | QMessageBox.Cancel,
#            QMessageBox.Save)
#
#    print(reply)
#    if reply == QMessageBox.Close:
#        print("close in keep_alive")
#        app.quit()
#    elif reply == QMessageBox.Save:
#        print("saving")
#        app.quit()
#    elif reply == QMessageBox.Cancel:
#        print("cancelled")
#        #FIXME: the main window can be already closed here, hence why Ctrl+C in console was needed to can quit...
#        app.lastWindowClosed.connect(keep_alive)
#    else:
#        raise Exception(f"impossible {reply}")

#FIXME: get the tooltip to update even though mouse didn't move but underlaying items order changed, maybe see: https://stackoverflow.com/questions/19427625/continuously-updating-tooltip

def signal_handler(sig, frame):
    print('\nYou pressed Ctrl+C! Exiting...')
    QApplication.quit()
    #sys.exit(0)

##needed to catch Ctrl+C 1of3, src: https://stackoverflow.com/questions/4938723/what-is-the-correct-way-to-make-my-pyqt-application-quit-when-killed-from-the-co/11705366#11705366
## You HAVE TO reimplement QApplication.event, otherwise it does not work.
## I believe that you need some python callable to catch the signal
## or KeyboardInterrupt exception.
#class MeApplication(QApplication):
#    def event(self, e):
#        return QApplication.event(self, e)


def main():
    signal.signal(signal.SIGINT, signal_handler)
    app = QApplication(sys.argv)
    #app = MeApplication(sys.argv) 2of3
    # And start a timer to call Application.event repeatedly.
    # You can change the timer parameter as you like.
    #app.startTimer(500) #needed to catch Ctrl+C 3of3


    #another way to catch/run Ctrl+C 1of1, src: https://stackoverflow.com/questions/4938723/what-is-the-correct-way-to-make-my-pyqt-application-quit-when-killed-from-the-co/4939113#4939113
    timer = QTimer()
    timer.start(500)  # You may change this if you wish.
    timer.timeout.connect(lambda: None)  # Let the interpreter run each 500 ms.

    #app.setQuitOnLastWindowClosed(False)
    app.setQuitOnLastWindowClosed(True) #using another way, with closeEvent now
    #app.lastWindowClosed.connect(keep_alive)
    win = QtWidgets.QMainWindow()
    #win.statusBar().showMessage('Message in statusbar.') #probably only works if win.show() is used!

    #win = QtGui.QWindow()
    #win.show()

    #window = QtGui.QWindow()
    #window.show()
    i=MyQListWidget()
    #i=MyQListWidget(win)
    i.show()
    #win.closeEvent = self.on_close_attempt
    #win.show()  #works but doesn't detect alt+f4

    sys.exit(app.exec_())

#src: https://github.com/barneygale/elevate/blob/master/elevate/posix.py
def elevate(graphical=True):
    import os
    if os.geteuid() == 0:
        return

    #FIXME: need to preserve env. vars $DISPLAY and $XAUTHORITY (at least), else it won't start! and pkexec clears them also!
    #FIXME: is shlex.quote() needed for sys.argv values?
    #args = [sys.executable] + sys.argv
    args = [sys.executable]
    args.append(os.path.realpath(__file__))
    args+=sys.argv[1:]
    print(args)
    commands = []

    if graphical:
        if sys.platform.startswith("linux") and os.environ.get("DISPLAY"):
            commands.append(["pkexec"] + args)
            commands.append(["gksudo"] + args)
            commands.append(["kdesudo"] + args)

    commands.append(["sudo"] + args)

    for args in commands:
        try:
            os.execlp(args[0], *args)
        except OSError as e:
            if e.errno != errno.ENOENT or args[0] == "sudo":
                raise

if __name__ == '__main__':
    elevate(graphical=False)
    main()

#Keep this last:
#vim filetype is set to python below, otherwise vim will use tabs instead of spaces for indentation when pypy3 is interpreter
# vim: set ft=python
