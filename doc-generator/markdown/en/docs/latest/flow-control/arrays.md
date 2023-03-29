# Arrays

Arrays in Misti don't have special syntax for declaration.
They are created the following way instead:

```misti
// Array[Int]
val numbers = Array(10, 20, 30)
// Array[Str]
val names = Array("Pablo", "Kim", "Mike")
```

Accessing or mutating the array use a similar syntax to other languages.

```misti
// Access. Note the dot
val secondNumber = numbers.[1]

// Mutation. Note the dot
names.[2] = "Josh"
```

```md-warning
Place a dot between the array and square brackets to access or mutate an array.
<br>
<br>
If you don't place a dot, it will be interpreted as a generic parameter.
```

## Importance of placing a dot

If there is no dot between the array and square brackets, then it is parsed
as a generic parameter.

```misti
// Access or mutation
variable.[index]
val n = numbers.[0]
numbers.[1] = 20

// Generic parameter
arrayOf[Datatype]
arrayOf[Str]
arrayOf[Bool]
```




