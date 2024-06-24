hi Normal guifg=#cccccc guibg=#000000 gui=NONE cterm=NONE
hi Statement guifg=#ffff60 guibg=NONE gui=NONE cterm=NONE
hi Type guifg=#60ff60 guibg=NONE gui=NONE cterm=NONE
hi StatusLine guifg=#ffffff guibg=#0000ee gui=bold cterm=bold
set nocompatible
"syntax on
highlight Normal ctermfg=250 ctermbg=none

" Attempt to determine the type of a file based on its name and possibly its
" contents. Use this to allow intelligent auto-indenting for each filetype,
" and for plugins that are filetype specific.
filetype plugin indent on
set nomodeline

set history=9999
set mouse=

set cmdheight=2
"^ this changes which char becomes ��� thus showing that's all about the colors not the chars themselves

map :Q :q
set directory=/tmp
"required stuff:
set cursorline
highlight CursorLine gui=reverse cterm=reverse ctermbg=NONE guibg=NONE
" This does a Ctrl+L equivalent to work around this nvim issue: https://github.com/neovim/neovim/issues/21064 thanks to:  zeertzjq
:mode
