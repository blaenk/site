---
title = "Cryptography"
published = "May 4, 2016"
excerpt = "Cryptography"
comments = false
---

Diffie-Hellman key exchange works by agreeing on two publicly shared values: a large prime number $q$ and a primitive root $g$. Alice and Bob each generate a _secret key_---a large random number---$x_a$ and $x_b$ respectively, and each raise the _primitive root_ to the power of the _secret key_, modulo the _large prime number_.

$$
\begin{align*}
y_a &= g^{x_a} \bmod q \\
y_b &= g^{x_b} \bmod q
\end{align*}
$$

The results are sent to each other and the shared key is computed by raising the received value to the _secret key_ modulo the _primitive root_.

$$
\begin{align*}
k_{ab} &= y_b^{x_a} \bmod q \\
k_{ab} &= y_a^{x_b} \bmod q
\end{align*}
$$

Given a prime number $q$, a primitive root $g$ is a number such that every number from 1 up to $q - 1$ can be computed by raising the primitive root to some number $k$.

$$
\forall x \in \{1, 2, ..., q - 1\}\ \mathbb{Z}_q \\
\exists k \text { such that } g^k = x
$$

<img src="http://i.imgur.com/aAnzCCF.png" class="center" />

A common analogy is that of mixing paint.

<img src="http://i.imgur.com/Cp3ZKm4.png" class="center" />
