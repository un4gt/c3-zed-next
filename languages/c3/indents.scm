(_ "{" "}" @end) @indent

(_ "(" ")" @end) @indent
(_ "[" "]" @end) @indent
(_ "[<" ">]" @end) @indent

(ct_if_stmt "$endif" @end) @indent
(ct_for_stmt "$endfor" @end) @indent
(ct_foreach_stmt "$endforeach" @end) @indent
(ct_switch_stmt "$endswitch" @end) @indent

[
  "$else"
  "$endif"
  "$endfor"
  "$endforeach"
  "$endswitch"
] @outdent
