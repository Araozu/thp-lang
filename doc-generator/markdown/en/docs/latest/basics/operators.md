# Operators

Misti has similar operators to JS.

```md-warning
Misti will enforce type checking in operators at compile time.
<p>
However, due to how JS works, automatic type casting may still occur if you combine the Misti output with JS.
</p>
```

## Basic operators

```misti
4 + 2
4 - 2
4 * 2
4 / 2
4 % 2
4 ** 2
```

## Asignment operators

```misti
i += 1
i -= 1
i *= 2
i /= 2
i %= 2
i **= 2
```

## Bitwise operators

Not considered at the moment.


## Comparison

```misti
1 == 2
1 != 2
1 > 2
1 < 2
1 >= 2
1 <= 2
```

```md-warning
In Misti there's only double equals `==`.
<br>
<br>
<code>x == y</code> will ALWAYS compile to <code>x === y</code>
```

## Logical operators

```misti
true && false
true || false

!true
```

```md-warning
Multiple `!` are invalid, since there is no automatic type casting.
<br>
<br>
<code>!!</code> would be considered a different operator.
```

```md-warning
There is no short-circuit like so something like:
<br>
<br>
<code>true && "value"</code> will throw an error at compile time.
```


