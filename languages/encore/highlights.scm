(comment) @comment

(string_literal) @string
(integer_literal) @number
(float_literal) @number.float
(boolean_literal) @boolean

(numeric_suffix) @type.builtin
(visibility_modifier) @keyword

[
  "import"
  "struct"
  "enum"
  "trait"
  "impl"
  "for"
  "extern"
  "fn"
  "ret"
  "let"
  "mut"
  "do"
  "while"
  "loop"
  "if"
  "elif"
  "else"
  "match"
  "unsafe"
  "break"
  "continue"
] @keyword

"self" @variable.special

[
  "="
  "=>"
  "->"
  "?"
  "+="
  "-="
  "*="
  "/="
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
  signature: (struct_signature
    name: (identifier) @type))

(enum_definition
  name: (identifier) @type)

(enum_definition
  (struct_signature
    name: (identifier) @constructor))

(trait_definition
  name: (identifier) @type)

(impl_definition
  trait_name: (identifier) @type)

(function_definition
  signature: (function_signature
    name: (identifier) @function))

(extern_function_definition
  signature: (function_signature
    name: (identifier) @function))

(trait_definition
  method: (function_signature
    name: (identifier) @function.method))

(impl_method_definition
  signature: (function_signature
    name: (identifier) @function.method))

(method_call_expression
  method: (identifier) @function.method)

(call_expression
  function: (path
    (path_segment
      name: (identifier) @function)))

(typed_parameter
  name: (identifier) @variable.parameter)

(let_statement
  name: (identifier) @variable)

(match_binding
  name: (identifier) @variable.parameter)

(loop_label
  name: (identifier) @label)

(field_access_expression
  field: (identifier) @property)

(c_like_struct_fields
  (typed_parameter
    name: (identifier) @property))

(type
  pointer: (any_pointer_suffix) @operator)

(type
  pointer: (smart_pointer_suffix) @type.builtin)

(type
  name: (identifier) @type)

((type
   name: (identifier) @type.builtin)
 (#match? @type.builtin "^(Self|bool|char|str|usize|isize|[ui][0-9]+|f[0-9]+)$"))

(struct_initializer
  type: (type
    name: (identifier) @constructor))
