// Test raw blocks.

---
// No extra space.
`A``B`

---
// Typst syntax inside.
`#let x = 1` \
`#f(1)`

---
// Multiline block splits paragraphs.

First
```
Second
```
Third

---
// Lots of backticks inside.
````
```backticks```
````

---
// Trimming.

// Space between "rust" and "let" is trimmed.
The keyword ```rust let```.

// Trimming depends on number backticks.
<``> \
<` untrimmed `> \
<``` trimmed` ```> \
<``` trimmed ```> \
<``` trimmed```>

// Multiline trimming.
```py
import this

def hi():
  print("Hi!")
```

---
// Make everything block-level.
#let raw(text) = raw(text, block: true)

// The new raw's argument list doesn't contain `block`.
// Error: 6-10 unexpected argument
This `is` block-level.

---
// Unterminated.
// Error: 2:1 expected backtick(s)
`endless