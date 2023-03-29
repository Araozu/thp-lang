# Variables and Constants

Misti uses `var` for variables and `val` for constant reference variables.

Variables and constants must always be initialized.

```misti
// A variable, its value can be modified
var accumulator = 0
accumulator += 1
```

```misti
// A "constant", its value cannot be modified
val name = "Bob"

// Illegal, will raise an error in compile time
name = "Mike"
```

```md-info
If a constant's value is a primitive value, it cannot be changed.
<br />
However, if it is a reference value, its inner attributes can still
be changed, but not the reference itself.
```

## Identifiers

Identifiers start with a lower case letter or underscore, 
and then can contain letters,  numbers and underscores. 
They can not start with a dollar sign.

```misti
variable
_variable

var1
v4r
```

If an identifier starts with an upper case letter, it is considered to be
a Datatype.

```misti
Str
Num
Bool
Array
Map

CustomDatatype
```

## Type inference

Variable declarations have type inference.

```misti
val anInteger = 40
val aFloat = 10.20e+4
var aBoolean = true
```

The datatype can be optionally specified by placing it before `var` or `val`.

```misti
Int val anInteger = 40
Float val aFloat = 10.20e+4
Bool var aBoolean = true
```

However, if the variable is a constant created with `val`,
this keyword is optional.

So, the following are equivalent.

```misti
Str val name = "Juan"
// is equivalent to
Str name = "Juan"
```

It is not possible to omit `var`. So the following will not mean the same.

```misti
Str var age = 20
// is not equivalent to
Str age = 20

// the previous statement is equivalent to:
// Str val age = 20
```

The second statement will declare a constant, not a variable.


## Assign a block to a variable

Assigning a block to a variable will execute the block,
and the last expression will be the value of the variable.

```misti
val roi =
    val income = someIncomeCalculation()
    val investment = 25000
    income / investment   // This will be the value of `roi`
```
