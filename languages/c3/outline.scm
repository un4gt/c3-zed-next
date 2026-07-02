(module_declaration
  path: (path_ident) @name) @item

(struct_declaration
  name: (type_ident) @name) @item

(bitstruct_declaration
  name: (type_ident) @name) @item

(interface_declaration
  name: (type_ident) @name) @item

(enum_declaration
  name: (type_ident) @name) @item

(constdef_declaration
  name: (type_ident) @name) @item

(faultdef_declaration
  (const_ident) @name) @item

(typedef_declaration
  name: (type_ident) @name) @item

(alias_declaration
  name: (_) @name) @item

(attrdef_declaration
  name: (at_type_ident) @name) @item

(func_definition
  (func_header
    name: (_) @name)) @item

(func_declaration
  (func_header
    name: (_) @name)) @item

(macro_declaration
  (macro_header
    name: (_) @name)) @item

(doc_comment) @annotation
(attributes) @annotation
(attribute) @annotation
