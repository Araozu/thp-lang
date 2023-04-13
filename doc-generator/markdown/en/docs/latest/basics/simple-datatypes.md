# Simple datatypes

The following are the primitive datatypes. They have literal representations.

## Num

Equivalent to a number in JS.

```misti
val f0 = 322
val f1 = 10.304
val f2 = -34.343223
val f3 = 0.234234e+2
val f4 = 1e+4
```

A floating point number __must__ have a digit before and after the dot.

```misti
val valid1 = 0.45
val valid2 = 13.0

// Will not be considered as floating point numbers
val invalid1 = .45  // Will be interpreted as the operator `.` and the integer 45
val invalid2 = 13.  // Will be interpreted as the integer 13 and the operator `.`
```

## Bool

True and false

```
true
false
```
