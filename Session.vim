let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/www/rust/elpc
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +56 ~/www/rust/elpc/src/ast/parser.rs
badd +7 ~/www/rust/elpc/src/main.rs
badd +14 ~/www/rust/elpc/src/ast/nodes.rs
badd +2 ~/www/rust/elpc/src/ast/mod.rs
badd +24 src/elp.pest
badd +0 examples/HelloWorld/Package.elp
badd +1 ~/www/rust/elpc/.envrc
badd +1 ~/www/rust/elpc/examples/HelloWorld/screens/blog/\[id]/index.velp
badd +0 fugitive:///Users/dave/www/rust/elpc/.git//
argglobal
%argdel
edit ~/www/rust/elpc/src/ast/parser.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
split
1wincmd k
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd _ | wincmd |
split
1wincmd k
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd w
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
wincmd =
argglobal
balt ~/www/rust/elpc/examples/HelloWorld/screens/blog/\[id]/index.velp
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=6
setlocal fml=1
setlocal fdn=20
setlocal fen
35
normal! zo
42
normal! zo
43
normal! zo
43
normal! zo
44
normal! zo
44
normal! zo
44
normal! zo
44
normal! zo
57
normal! zo
let s:l = 53 - ((1 * winheight(0) + 14) / 29)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 53
normal! 012|
wincmd w
argglobal
if bufexists(fnamemodify("~/www/rust/elpc/src/ast/nodes.rs", ":p")) | buffer ~/www/rust/elpc/src/ast/nodes.rs | else | edit ~/www/rust/elpc/src/ast/nodes.rs | endif
if &buftype ==# 'terminal'
  silent file ~/www/rust/elpc/src/ast/nodes.rs
endif
balt src/elp.pest
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=1
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 14 - ((8 * winheight(0) + 7) / 14)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 14
normal! 017|
wincmd w
argglobal
if bufexists(fnamemodify("src/elp.pest", ":p")) | buffer src/elp.pest | else | edit src/elp.pest | endif
if &buftype ==# 'terminal'
  silent file src/elp.pest
endif
balt ~/www/rust/elpc/src/ast/nodes.rs
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 18 - ((11 * winheight(0) + 7) / 14)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 18
normal! 020|
wincmd w
argglobal
if bufexists(fnamemodify("~/www/rust/elpc/src/main.rs", ":p")) | buffer ~/www/rust/elpc/src/main.rs | else | edit ~/www/rust/elpc/src/main.rs | endif
if &buftype ==# 'terminal'
  silent file ~/www/rust/elpc/src/main.rs
endif
balt examples/HelloWorld/Package.elp
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=2
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 7 - ((6 * winheight(0) + 7) / 14)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 7
normal! 018|
wincmd w
argglobal
if bufexists(fnamemodify("examples/HelloWorld/Package.elp", ":p")) | buffer examples/HelloWorld/Package.elp | else | edit examples/HelloWorld/Package.elp | endif
if &buftype ==# 'terminal'
  silent file examples/HelloWorld/Package.elp
endif
balt ~/www/rust/elpc/src/main.rs
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 7) / 14)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 02|
wincmd w
argglobal
if bufexists(fnamemodify("fugitive:///Users/dave/www/rust/elpc/.git//", ":p")) | buffer fugitive:///Users/dave/www/rust/elpc/.git// | else | edit fugitive:///Users/dave/www/rust/elpc/.git// | endif
if &buftype ==# 'terminal'
  silent file fugitive:///Users/dave/www/rust/elpc/.git//
endif
balt examples/HelloWorld/Package.elp
setlocal fdm=expr
setlocal fde=nvim_treesitter#foldexpr()
setlocal fmr=<<<<<<<<,>>>>>>>>
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 1 - ((0 * winheight(0) + 7) / 14)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 02|
wincmd w
4wincmd w
wincmd =
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
