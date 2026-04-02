(comment) @comment

(string_literal) @string
(integer_literal) @number
(float_literal) @number.float
(boolean_literal) @boolean

(numeric_suffix) @type.builtin
(float_suffix) @type.builtin

[
  ; "pub"
  "import"
  "struct"
  "enum"
  "fn"
  "ret"
  "let"
  "do"
  "while"
  "loop"
  "if"
  "elif"
  "else"
  "match"

  ; "break"
  ; "continue"
] @keyword

[
  "="
  "=>"
  "->"
  "+"
  "-"
  "*"
  "/"
  "%"
  "||"
  "&&"
  "|"
  "^"
  "&"
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "<<"
  ">>"
  "!"
  "~"
  "++"
  "--"
] @operator

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

[
  ","
  "."
  ":"
  "::"
] @punctuation.delimiter

(import_path
  module: (identifier) @namespace)

(struct_definition
  (struct_body
    name: (identifier) @type))

(enum_definition
  name: (identifier) @type)

(enum_definition
  (struct_body
    name: (identifier) @constructor))

(fn_definition
  name: (identifier) @function)

(typed_parameter
  name: (identifier) @variable.parameter)

(let_statement
  name: (identifier) @variable)

(match_binding
  name: (identifier) @variable.parameter)

(field_access_expression
  field: (identifier) @property)

(c_like_struct_fields
  (typed_parameter
    name: (identifier) @property))

(type
  (identifier) @type)

((type
   (identifier) @type.builtin)
 (#match? @type.builtin "^(usize|isize|[ui][0-9]+|f[0-9]+)$"))

(struct_initializer
  type: (type
    (identifier) @constructor))

(call_expression
  function: (path
    (path_segment
      name: (identifier) @function)))
