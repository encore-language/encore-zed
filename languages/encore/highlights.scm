(comment) @comment

(function_attribute
  "#" @punctuation.special
  "attr" @attribute
  name: (identifier) @attribute)

[
  "fn"
  "struct"
  "enum"
  "trait"
  "impl"
  "for"
  "let"
  "mut"
  "ret"
  "while"
  "do"
  "loop"
  "break"
  "continue"
  "if"
  "elif"
  "else"
  "match"
  "import"
  "extern"
  "unsafe"
] @keyword

(visibility_modifier) @keyword

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "="
  "+="
  "-="
  "*="
  "/="
  "&&"
  "||"
  "&"
  "|"
  "^"
  "~"
  "<<"
  ">>"
  "?"
  "!"
  "=>"
  "->"
  "::"
  "."
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
  ":"
] @punctuation.delimiter

(string_literal) @string
(integer_literal) @number
(float_literal) @number.float
(boolean_literal) @boolean
(numeric_suffix) @type.builtin

(smart_pointer_suffix) @type.builtin
(any_pointer_suffix) @operator

(typed_parameter name: (identifier) @variable.parameter)
(receiver_parameter "self" @variable.special)
(match_binding name: (identifier) @variable.parameter)
(let_statement name: (identifier) @variable)
(loop_label name: (identifier) @label)

((path_segment name: (identifier) @variable)
 (#match? @variable "^[a-z_][A-Za-z0-9_]*$"))

(import_path module: (identifier) @namespace)
((import_path
   (import_path
     module: (identifier) @namespace)))
((import_path
   (import_path
     (import_path
       module: (identifier) @namespace))))

(function_signature name: (identifier) @function)
(call_expression function: (path (path_segment name: (identifier) @function.call)))
(method_call_expression method: (identifier) @function.method.call)

(field_access_expression field: (identifier) @property)
(c_like_struct_fields (typed_parameter name: (identifier) @property))

(struct_definition signature: (struct_signature name: (identifier) @type))
(enum_definition name: (identifier) @type)
(trait_definition name: (identifier) @type)
(impl_definition trait_name: (identifier) @type)
(impl_definition target: (type name: (identifier) @type))
(struct_signature name: (identifier) @type)
(type name: (identifier) @type)

((type name: (identifier) @type.builtin)
 (#match? @type.builtin "^(Self|bool|char|str|usize|isize|[ui][0-9]+|f[0-9]+)$"))

(enum_definition (struct_signature name: (identifier) @constructor))
(struct_initializer type: (type name: (identifier) @constructor))

((path_segment name: (identifier) @type)
 (#match? @type "^[A-Z][A-Za-z0-9_]*$"))

((path
   (path_segment name: (identifier) @type)
   "::"
   (path_segment name: (identifier) @constructor))
 (#match? @type "^[A-Z][A-Za-z0-9_]*$")
 (#match? @constructor "^[A-Z][A-Za-z0-9_]*$"))
