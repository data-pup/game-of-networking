# Tokio Game of Life

This repository contains a (WIP) implementation of Conway's Game of Life.
There is an important caveat to this implementation however: it aims to be
a self-admittedly absurd version of the Game of Life, with individual cells
represented by individual threads that calculate their future states via
network requests over localhost.

The first question you might ask is "...Why?" To which I would answer,
"...Why *Not*?" This snarky answer would be followed by the explanation that
implementing the Game of Life in this manner presents a good chance to practice
working with operations involving asynchronous network requests, and to gain
a better understanding of how the `tokio` library works.

