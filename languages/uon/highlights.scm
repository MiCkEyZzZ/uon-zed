(string) @string
(number) @number
(boolean) @boolean
(null) @constant.builtin
(date) @string.special
(duration) @number

(comment) @comment

(pair
  key: (identifier) @property)
(pair
  key: (string) @property)

["{" "}"] @punctuation.bracket
["[" "]"] @punctuation.bracket
[":" ","] @punctuation.delimiter
