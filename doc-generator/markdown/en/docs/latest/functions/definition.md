# Function definition

Functions are defined with `fun` followed by the name of the function,
the parameters, the return type, `=` and an expression.

## Function with no parameters

To declare a function with no parameters include `()`, and call it with `()`.

```misti
fun getName() =
    // code...

val name = getName()
```

## Function that returns void/nothing

In Misti we return `Unit` instead of `void`, `null`, `undefined` or others.

So a function that doesn't return anything, would return `Unit`.

```misti
// This function just prints Hello and returns
fun printHello() -> Unit =
    print("Hello")
```

This type, `Unit`, is treated like `void`, so it is ignored.

If a function doesn't return anything, `Unit` can be omitted.

```misti
// This first declaration
fun doSomething() -> Unit =
    something()


// is equivalent to this one
fun doSomething() =
    something()
```

## Function with return

First, the return type must be defined in the declaration.

Let's say that a function `getLuckyNumber` returns a Float, then it
would be declared like this:

```misti
fun getLuckyNumber() -> Float =
    // Body of the function
```

And finally, the return value is the last expression in the function.
The following function will return 7.

```misti
fun getLuckyNumber() -> Float =
    // This '7' is the last expression, so it will be returned
    7


val number = getLuckyNumber()  // number = 7
```

## Return multiple values

We can use a tuple if we need to return multiple values.

```misti
fun getPerson() -> #(Str, Int) =
    // Logic...
    #("Kim", 33)


fun tupleContains(#(Str, Int) data, Str key) -> #(Bool, Int) =
    val #(currentKey, value) = data
    if currentKey == key do
        #(true, value)
    else
        #(false, 0)


tupleContains(#("Test", 200), "Test")
```


