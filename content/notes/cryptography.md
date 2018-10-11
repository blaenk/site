+++
title = "Cryptography"
date = 2016-05-04

[note]
kind = "concept"
+++

Diffie-Hellman key exchange works by agreeing on two publicly shared values: a large prime number `$q$` and a primitive root `$g$`. Alice and Bob each generate a _secret key_---a large random number---`$x_a$` and `$x_b$` respectively, and each raise the _primitive root_ to the power of the _secret key_, modulo the _large prime number_.

<div>
$$
\begin{align*}
y_a &= g^{x_a} \bmod q \\
y_b &= g^{x_b} \bmod q
\end{align*}
$$
</div>

The results are sent to each other and the shared key is computed by raising the received value to the _secret key_ modulo the _primitive root_.

<div>
$$
\begin{align*}
k_{ab} &= y_b^{x_a} \bmod q \\
k_{ab} &= y_a^{x_b} \bmod q
\end{align*}
$$
</div>

Given a prime number `$q$`, a primitive root `$g$` is a number such that every number from 1 up to `$q - 1$` can be computed by raising the primitive root to some number `$k$`.

<div>
$$
\forall x \in \{1, 2, ..., q - 1\}\ \mathbb{Z}_q \\
\exists k \text { such that } g^k = x
$$
</div>

<img src="//i.imgur.com/aAnzCCF.png" class="center" />

A common analogy is that of mixing paint.

<img src="//i.imgur.com/Cp3ZKm4.png" class="center" />
