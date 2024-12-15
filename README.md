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
2
\cdot
\begin{bmatrix}
1 \\ 2
\end{bmatrix}
+ 3
\cdot
\begin{bmatrix}
3 \\ 4
\end{bmatrix}
= \begin{bmatrix}
2 \\ 4
\end{bmatrix}
+ \begin{bmatrix}
9 \\ 12
\end{bmatrix}
= \begin{bmatrix}
11 \\ 16
\end{bmatrix}
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

### 03 - Dot product

> ```rust
> impl<K> Vector::<K> {
>   fn dot::<K>(&self, v: Vector::<K>) -> K;
> }
> ```
>
> Maximum time complexity : $O(n)$
>
> Maximum space complexity : $O(n)$

The dot product of two vectors is the sum-product of their corresponding elements.

For example:

$$
\begin{bmatrix}
1 \\ 2
\end{bmatrix}
\cdot
\begin{bmatrix}
3 \\ 4
\end{bmatrix}
= 1 \times 3 + 2 \times 4
= 11
$$

Simple.

### 04 - Norm

> ```rust
> impl<K> Vector::<K> {
>   fn norm_1(&self) -> f32;
>   fn norm(&self) -> f32;
>   fn norm_inf(&self) -> f32;
> }
> ```
>
> Maximum time complexity : $O(n)$
>
> Maximum space complexity : $O(n)$

The norms to implement are:

- **$l_1$ norm**: $\|v\|_1$ (also called the Taxicab norm or Manhattan norm)
    - **Definition**: the sum of the absolute values of the elements of the vector.
    - **Geometry**: the distance traveled if you can only move along the axes.

- **$l_2$ norm**: $\|v\|$ or $\|v\|_2$ (also called the Euclidean norm)
    - **Definition**: the square root of the sum of the squares of the elements of the vector.
    - **Geometry**: the length of the vector.

- **$l_\infty$ norm**: $\|v\|_\infty$ (also called the supremum norm or uniform norm)
    - **Definition**: the maximum absolute value of the elements of the vector.
    - **Geometry**: the maximum distance traveled along one axis. For example, if your vector travels 3 units along $x$ and 4 units along the $y$, the $l_\infty$ norm would be 4.

### 05 - Cosine

> ```rust
> fn angle_cos::<K>(u: &Vector::<K>, v: &Vector::<K>) -> f32;
> ```
>
> Maximum time complexity : $O(n)$
>
> Maximum space complexity : $O(n)$

The cosine of the angle between two vectors is the dot product of the two vectors divided by the product of their norms. Simple.

As the subject says, *"use the functions you wrote during the two previous exercises"*.

### 06 - Cross product

> ```rust
> fn cross_product::<K>(u: &Vector::<K>, v: &Vector::<K>) -> Vector::<K>;
> ```

The cross product of two vectors outputs a vector that is perpendicular to the plane formed by the two vectors.

<!-- For example, the cross product of $\begin{bmatrix} 1 \\ 0 \\ 0 \end{bmatrix}$ and $\begin{bmatrix} 0 \\ 1 \\ 0 \end{bmatrix}$ is $\begin{bmatrix} 0 \\ 0 \\ 1 \end{bmatrix}$, because the two vectors form the $xy$ plane, and the cross product points in the $z$ direction. -->

For example:

$$
\begin{bmatrix}
1 \\ 0 \\ 0
\end{bmatrix}
\times
\begin{bmatrix}
0 \\ 1 \\ 0
\end{bmatrix}
= \begin{bmatrix}
0 \\ 0 \\ 1
\end{bmatrix}
$$

Here the two vectors form the $xy$ plane, and the cross product points in the $z$ direction.

The cross product is only defined for two $(3, 1)$ vectors, and the formula can be found in the *Cross product* [Wikipedia page](https://en.wikipedia.org/wiki/Cross_product#Matrix_notation).

However, it has other interesting properties, have a look at the [Cross product](https://www.youtube.com/watch?v=eu6i7WJeinw) video of the *Essence of Linear Algebra* series.

## Resources

- [📺 YouTube − The Lp Norm for Vectors and Functions](https://www.youtube.com/watch?v=NKuLYRui-NU)
- [📖 Wikipedia − Cross product (Matrix notation)](https://en.wikipedia.org/wiki/Cross_product#Matrix_notation)
- [💬]()
