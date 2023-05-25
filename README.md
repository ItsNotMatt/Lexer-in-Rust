Easily scalable lexer/scanner written in rust. Turns a string into tokens that can be used to create an AST.

This lexer discards whitespace, not meant to be used for any form of whitespace syntax. Can be scaled easily by adding new keywords to the flush_identifier() fn.