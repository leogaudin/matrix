<h1 align="center">matrix</h1>

**`matrix` is a 42 project covering linear algebra.**

> *⚠️ This guide assumes you have watched or are watching the awesome [Essence of Linear Algebra](https://www.youtube.com/playlist?list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab) series by 3Blue1Brown.*
>
> *Moreover, it will mostly focus on the linear algebra part of the project, and less on the specificities of Rust or other implementation details.*

## Table of Contents

- [Exercises](#exercises) 🏋🏻
- [Resources](#resources) 📖

## Exercises

### 00 - Add, Subtract and Scale

> ```rust
> fn add(&mut self, v: &Matrix<K>);
> fn sub(&mut self, v: &Matrix<K>);
> fn scl(&mut self, a: K);
> ```
>
> Maximum time complexity : $O(n)$
>
> Maximum space complexity : $O(n)$

This exercise is pretty straightforward:

- The addition and subtraction can only be done with matrices of the same shape, and is the result of **adding or subtracting each element of the first matrix with the corresponding element of the second matrix**.
- The scaling is depends on a scalar, and is the result of **multiplying each element of the matrix by the scalar**.

### 01 - Linear combination

> ```rust
> fn linear_combination::<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>;
> ```
>
> Maximum time complexity : $O(n)$
>
> Maximum space complexity : $O(n)$

Linear combination is the result of **adding the result of multiplying each vector by its corresponding coefficient**.

For example, the linear combination of the vectors $[1, 2]$ and $[3, 4]$ with the coefficients $2$ and $3$ respectively is:

$$
2 \begin{bmatrix} 1 \\ 2 \end{bmatrix} + 3 \begin{bmatrix} 3 \\ 4 \end{bmatrix} = \begin{bmatrix} 2 \\ 4 \end{bmatrix} + \begin{bmatrix} 9 \\ 12 \end{bmatrix} = \begin{bmatrix} 11 \\ 16 \end{bmatrix}
$$

### 02 - Linear interpolation

> ```rust
> fn lerp::<V>(u: V, v: V, t: f32) -> V;
> ```
>
> Maximum time complexity : $O(n)$
>
> Maximum space complexity : $O(n)$

Interpolating two numbers $u$ and $v$ with a factor $t$ (being $0 \leq t \leq 1$) vulgarly means **finding the number that is $t$ percent between $u$ and $v$**.

For example, $\text{lerp}(1, 3, 0.5) = 2$, because $2$ is halfway between $1$ and $3$. Simple.

With vectors, it is easier to visualize. Finding the linear interpolation between two vectors $u$ and $v$ would vulgarly mean:

1. Drawing an imaginary line between the extremities of the vectors.
2. Finding the point that is $t$ percent between the two extremities.
3. Create a vector that points to there.

![lerp](./assets/lerp.gif)

Here, $V_t = \text{lerp}(u, v, t)$ points at the point that is $t\%$ between $u$ and $v$.

Find the formula for the linear interpolation of two numbers in the GIF!

> 💡 For those who did `dslr`, it can remind you of the negative log-loss formula.

If you implemented well the previous exercises, this one should be a piece of cake: just overload the operators with your functions!

> 💡 Rust's syntax can sometimes be a pain, so here is a tip:
>
> ```rust
> fn lerp<
>    V: std::fmt::Display
>       + std::ops::Add<Output = V>
>       + std::ops::Sub<Output = V>
>       + std::ops::Mul<f32, Output = V>
> >
> ```
>
> Basically tells "this function will take any custom type `V`, as long as it implements `Display` (to print it), `Add`, `Sub` and a `Mul` that takes a `f32` and returns a `V`".

With all these tips, you should be able to implement `lerp` in one line!

## Resources

- [📺 ]()
- [💬]()
- [📖]()
