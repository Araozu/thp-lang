# Anonymous functions/Lambdas

```md-info
Subject to change
```

An anonymous consists of the `fn` keyword, parameters and body.

```misti
fn (x, y) {
    // Body
    x + y
}
```

The following types are optional, if we can be able to infer them:

- The types of the parameters
- The return type

If used, they would look like this:

```misti
fn (Int x, Int y) -> Int {
    // Body
    x + y
}
```

## Lambdas

If an anonymous function doesn't have parameters, the `fn` keyword may be omitted.
This is called a lambda.

```misti
{ doSomething() }

// The above lambda is equivalent to:
fn () {
    doSomething()
}
```

## Inferred arguments

If the arguments of the lambda don't need names, the following
syntax can be used.

```misti
{ $1 + $2 }
```

Inside a short lambda you can use `$1`, `$2`, `$3`, etc. to refer to the
parameters of the lambda.

So the following are the same:

```misti
{ $1 + $2 }

// The same as:
fn (x, y) {
    x + y
}
```


