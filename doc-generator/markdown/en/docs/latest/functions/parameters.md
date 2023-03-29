# Function parameters

## Function with 1 parameter

Place the parameter's datatype after the function's name, then the name of the parameter.

For example, a function that takes a `Str` as parameter is defined as follows:

```misti
fun sayHello(Str name) =
    // Body of the function
```

Then the parameter `name` can be used.

```misti
fun sayHello(Str name) =
    print("Hello {name}")
```

## Function with 2 or more parameters

The parameters are separated with commas:

```misti
// 2 parameters: x and y, both Int
fun add(Int x, Int y) -> Int =
    x + y
```

```misti
// 3 parameters
fun substring(Str input, Int start, Int end) -> Str =
    // Logic...
```

And so on.

## Generic parameters

Generic parameters consist of an uppercase letter enclosed in square brackets.
They are placed after the function name, but before the parameters list.

```misti
fun getItemAt[T](Array[T] arr, Int pos) -> T =
    // Function body
```

When calling the function, the generic parameter is placed in the same position.

```misti
val thirdName = getItemAt[String](names, 2)
```

If the generic parameter can be inferred, it's not necessary to put it.

```misti
// Will be a String, inferred
val thirdName = getItemAt(names, 2)
```

## Named parameters

When calling a function you can link the name of an argument to its value.
In the following function, `substring` has 3 parameters: `string`, `start` and `end`.

```misti
fun substring(Str string, Int start, Int end) =
    // Body of the function
```

Then, when calling the function, you can specify each parameter and their values.

```misti
// Without named parameters
substring("Hello, world!", 7, 12)

// With named parameters
substring(string: "Hello, world!", start: 7, end: 12)

substring(
    string: "Hello, world!",
    start: 7,
    end: 12,
)
```

This will return `"world"`.

You can do computations with named parameters as well.

```misti
substring(string: "Hello, world!", start: 12 - 5, end: 48 / 4)

substring(
    string: "Hello, world!",
    start: 12 - 5,
    end: 48 / 4,
)
```

