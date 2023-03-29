# Tuples

Tuples contain a fixed number of values of any datatype. They are denoted with
a hash and parenthesis.

```misti
val person = #("John", "Doe", 25)
val result = #(true, 200)
```

## Signature

```misti
// A tuple of Str, Str and Int
#(Str, Str, Int)

// An array of tuples. These tuples have a Str, Int and Bool
Array[#(Str, Int, Bool)]

// A function that takes a Str and Int, and return a tuple of Str and Int
(Str, Int) -> #(Str, Int)

// A function that takes a tuple of Str and Int, and returns a Bool
(#(Str, Int)) -> Bool
```

## Destructuring

In variable declaration

```misti
val data = #("String", 322, true)

val #(string, number, boolean) = data
val #(string, _, _) = data
```

In function parameters

```misti
// Without parameter destructuring
fun destructure(#(Str, Int) data) {
    val #(name, age) = data
    // Computations with name, age
}


// With parameter destructuring
fun destructure(#(Str name, Int age)) {
    // Computations with name, age
    // Note that now there is no way to refer to the whole tuple
}


// Parameter destructuring with tuple preservation
fun destructure(#(Str name, Int age) data) {
    // Computations with name, age
    // The tuple `data` can still be referred to
}
```

