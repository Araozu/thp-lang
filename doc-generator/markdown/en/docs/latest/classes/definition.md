# Classes

```misti
class Token(Str value, Int lineNumber, Int position)

val token1 = Token("if", 0, 0)
val token2 = Token(value: "else", lineNumber: 1, position: 4)
```

```misti
class FunToken(Int lineNumber, Int position) 
   -> Token("fun", lineNumber, position)

val funToken1 = FunToken(3, 0)
```
