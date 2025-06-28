.strings |
if length == 0 then
  []
else
  [range(length - 1) as $i | .[$i:$i+2]] as $windows |
  [$windows[] | "For want of a \(.[0]) the \(.[1]) was lost."] +
  ["And all for the want of a \(.[0])."]
end
