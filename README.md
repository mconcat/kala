# Nessie

Babel AST => Proto-Nessie AST => 
    1. Interpreted directly
    2. Compiled into rust native code and compiled into wasm
    3. Compiled into moddable-style bytecode and interpreted

1 will be PoC, 2 will be neccesary for cosmwasm integration, 3 will be an optiomization pass for 1.