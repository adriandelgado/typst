// Test page margins.

---
// Set all margins at once.
[
  #set page(height: 20pt, margins: 5pt)
  #place(top + left)[TL]
  #place(bottom + right)[BR]
]

---
// Set individual margins.
#set page(height: 40pt)
[#set page(left: 0pt); #align(left)[Left]]
[#set page(right: 0pt); #align(right)[Right]]
[#set page(top: 0pt); #align(top)[Top]]
[#set page(bottom: 0pt); #align(bottom)[Bottom]]

// Ensure that specific margins override general margins.
[#set page(margins: 0pt, left: 20pt); Overriden]