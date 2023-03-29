# Conditionals

Conditionals in Misti surround the condition with keywords,
and the body of each condition is defined with indentation.

```Misti
if condition do
    // code...
else if anotherCondition do
    // code...
else
    // more code...
```

Conditionals are expressions, they evaluate to the last expression
in each branch.

```misti
val result = if condition do value1 else value2
```

## Early return

If you need to return early based on a condition,
you can use `ret` instead of `do` in a confition. The last expression of
the block will be returned

```misti
if condition ret
    // code...
    computedValue  // this will be returned
```

