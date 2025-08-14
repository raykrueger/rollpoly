# RKDice Terminal Dice Rolling

A simple Rust 2024 application to roll dice in the terminal. Also supports being imported as a library.

## Syntax

### Basic Syntax

```
4d10 + 17: Roll a 10-sided die 4 times and add 17 to the result
2d20 - 3: Roll a 20 sided die 2 times and subtract 3
1d4 * 3: Roll a 4 sided die once and multiply by 3
d4 * 3: Same as above, leaving out the number of dice will default to 1
5d6 / 3: Roll 5d6 and divide by 3
5d6 // 3: Same as above but floor division
```

### Advanced Syntax

#### Keep Highest (K):

Used to (K)eep the highest roll. Can be followed by a number to keep that number of dice or by nothing to indicate keeping only one.

```
4d10K: Roll 4d10 and keep only the highest roll
7d12K3: Roll 7d12 and keep the highest three rolls
7d12K3 + 4: Roll as above then add 4
```

#### Keep Lowest (k):

Same as above but keeping the lowest.

```
3d3k: Roll 3d3 and keep the lowest roll
100d6k99: Roll 100d6 and keep all but the highest.
2d20k: Roll 2d20 and keep the lowest. This is a disadvantage roll in 5e
```

#### Drop Highest (X):

Used to drop the highest roll. Can be followed by a number to drop that number
of dice or by nothing to indicate dropping just one.

```
6d8X: Roll 6d8 and drop the highest
5d10X3 Roll 5d10 and drop the highest 3
```

#### Drop Lowest (x):


```
6d8x: Roll 6d8 and drop the lowest
5d10x3 Roll 5d10 and drop the lowest 3
```
#### Count Successes (> or <):

Counts the number of rolls above or below a certain value.

```
4d20>19: Rolls 4d20 and counts the number of rolls above 19
10d12<3: Rolls 10d12 and counts the number of rolls below 3
```

#### Count failures (f):

Addition to counting successes to specify an additional 'failure' condition.
Each failure will decrease the score by 1 while each success will still increase
by 1.

```
10d10>6f<3: Roll 10d10 and count successes over 6 and failures under 3
4d20<5f>19: Roll 4d20 and count successes under 5 and failures over 19
5d100<5f>3: Invalid, you cannot have your failure and success comparison both be more than or less than.
```

#### Exploding Dice (!):

Exploding dice is usually known as 'Rule of 6' or 'Rule of 10,' as it is in
Shadowrun. As long as the roll passes the specified comparison, another dice is
rolled and added to the total. This process repeats until a number that does not
match the comparison is rolled.

```
2d20!: Roll 2d20 and explode every time a 20 is rolled
7d20!3: Roll 7d20 and explode every time a 3 is rolled
4d6! Roll 4d6 and explode every time a 6 is rolled
d20!>10: Roll a d20 and explode every time a number higher than 10 is rolled
3d12!<2: Roll 3d12 and explode every time a 1 is rolled.
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
