.strings as $w |
if ($w | length) == 0 then
  []
else
  [range($w | length - 1) | . as $i | "For want of a \($w[$i]) the \($w[$i + 1]) was lost."] +
  ["And all for the want of a \($w[0])."]
end
