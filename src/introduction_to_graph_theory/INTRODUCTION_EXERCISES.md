# Introduction to Graph Theory: Exercises

Richard J. Trundeau in his [Introduction to graph theory][introduction-to-graph-theory] ends each chapter with exercises which refer to what he introduced in the chapter. This document lists the tasks in the first chapter and my solutions to them.

**IMPORTANT**: I have not double checked my solutions and some of them may and will be incorrect. I often have no idea what am I talking about. Proofreading is much appreciated.

---
## 1. List of subsets of the set `{1, 2, 3}`. There are eight of them.

`{}`

`{1}`, `{2}`, `{3}`

`{1, 2}`, `{1, 3}`, `{2, 3}`

`{1, 2, 3}`

---
## 2. Letting _A_ be a set and _J_ an empty set, show that the statement "_J_ is a subset of _A_" cannot be false and so is true. This proves that an empty set is a subset of every set.

a)

We have proven that a set is a subset of itself (_S_ ⊆ _S_).

If _A_ can be any set, then we pick _A_ to be an empty set as well. _J_ ⊆ _J_ cannot be false otherwise we would be inconsistent with the theorem of _S_ ⊆ _S_.

b)

A set _S'_ is a subset of set _S_ if all _x_ in _S_ are also in _S'_.

Therefore for "_J_ is a subset of _A_" to be false there has to exist at least one _x_ that is in _J_ and isn't in _A_.

But _J_ is empty, so there cannot be any _x_ which would make above true.

---
## 3. There is another version of Russel's Paradox. The village barber shaves those and only those men who live in the village and do not shave themselves. The village barber is a man and he lives in the village. Consider the question "Who shaves the barber?" Then explain how is it equivalent to Russell's Paradox.

If barber doesn't shave himself, then he is in the group of men who are shaved by the barber. But if he is shaved by himself, he breaks the condition. Therefore he can't shave himself. That makes him not break the condition again. To the question "Who shaves the barber" we cannot respond as any response is immediately inconsistent with the predicate "barber shaves all men who don't shave themselves".

The predicate "barber shaves those men who don't shave themselves" corresponds to the "set _S_ contains all not self-referring sets" of Russell's Paradox.

```
IF          barber shaves himself       |   _S_ contains itself
THEN        barber cannot shave himself |   _S_ cannot contain itself
SO AGAIN    barber shaves himself       |    _S_ contains itself
```

---
## 4. Let "_S_" be the collection of all sets that can be described in an English sentence of twenty-five words or less. Is "_S_" a set? Why or why not?

Since the sentence in the task is less than 25 words, the set would have to include itself. Therefore _S_ is not a set. That would lead to problems described in task **3**.

---
## 7. Use Theorem 2 to prove that `1 + 2 + ... + (v - 1) = (1/2)v(v - 1)` for any integer _v_ greater or equal to 2. Do not use any arithmetics or algebra.

> Theorem 2: The number of edges in a complete graph `K[v]` is given by the formula `e = (1/2)v(v - 1)`.

Start with 1 vertex. Keep adding vertexes until `v`. For every added vertex, draw edge from that vertex to all existing vertexes. That means you will draw `i - 1` edges each time. Which means you will start with `1` and with every new vertex, you will have drawn one more edge than previous turn. Since for two vertexes, you draw one edge, for three, you draw two additional edges (three in total), you will end up with `1 + 2 + ... + (v - 1)` edges. Theorem 2 proves says you should end up with `(1/2)v(v - 1)`. Therefore those two must equal.s

---
## 8. Let _G_ be graph with _v_ vertices and _e_ edges. In terms of _v_ and _e_, how many edges has _G'_?

We know that `K[v]` has `(1/2)v(v - 1)` edges. We know that number of edges in _G_ plus number of edges in _G'_ equal number of edges in `K[v]`.

_G'_ has `(1/2)v(v - 1) - e` edges.

<!-- Invisible List of References -->
[introduction-to-graph-theory]: https://www.goodreads.com/book/show/388049.Introduction_to_Graph_Theory
