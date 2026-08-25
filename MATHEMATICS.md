# 双平衡三进制的数学结构与探索方向

## 1. 精确的代数模型

把每一位记成坐标

\[
d_k=(x_k,y_k),\qquad x_k,y_k\in\{-1,0,1\},
\]

并定义

\[
\Phi(d_k)=y_k+x_k i,\qquad
\Phi(a)=\sum_k \Phi(d_k)3^k.
\]

这解释了项目里看似特殊的方向约定：数字 `1` 的坐标是 `(0,1)`，
所以它对应复数 `1`；数字 `3` 的坐标是 `(1,0)`，所以它对应 `i`。
九个数字只是高斯数字 `{a+bi | a,b∈{-1,0,1}}` 的 Lo Shu（洛书）编号。

直接推论：

- 只有非负幂时，每个高斯整数 `Z[i]` 都有唯一有限表示，因为实部和虚部
  分别有唯一的有限平衡三进制表示。
- 允许有限个负幂后，得到 `Z[1/3, i]`，即分母为 `3` 的幂的高斯有理数。
- 共轭就是左右翻转，四分之一转就是乘以 `±i`，范数
  `N(z)=z·conj(z)=x²+y²` 落回 `1/5/9` 这条实轴。

这和“使用复数作为进制基数”的经典研究有关，但并不相同：这里的基数仍是
实数 `3`，每一位同时携带两个平衡坐标。高斯整数进制系统的经典背景可参见
[Kátai–Szabó (1975)](https://acta.bibl.u-szeged.hu/14536/)。

## 2. 最有意思的发现：每一位就是有限域 F9

九个数字是商环

\[
\mathbb Z[i]/3\mathbb Z[i]
\]

的一组完整剩余类。因为 `X²+1` 在 `F3` 上没有根（`0²=0`，`±1²=1`，
都不等于 `-1=2`），所以它不可约，于是

\[
\mathbb Z[i]/(3)\cong \mathbb F_3[X]/(X^2+1)\cong\mathbb F_9.
\]

因此，把单个位加法或乘法产生的“进位”丢掉，剩下的个位运算不是随意的
九元运算，而是一个真正的域：

- `5` 是加法零元；
- `1` 是乘法单位元；
- 其余八个元素都有乘法逆元；
- 非零元素构成八阶循环群；当前编号里的 `8 = 1+i` 是一个生成元。

这已经在 `tests/properties_test.rs` 中做了穷举验证。它带来很实际的研究入口：

- 在同一套数字 UI 上实现 `F9` 的多项式、LFSR 和小型 Reed–Solomon/BCH 实验；
- 把“进位”理解为从模 `3` 的有限域运算提升到高斯整数运算；
- 研究基于八阶乘法群的 8 点有限域变换、置换和可逆混合层。

需要注意：当前 `{−1,0,1}²` 是方便的剩余代表，不是乘法封闭的
Teichmüller 代表；所以普通 DBT 乘法仍会产生进位。

## 3. 两种完全不同的“距离”

### 欧氏/空间视角

小数点后每增加一位，就把当前正方形等分成 `3×3` 个子方格并选择其中一个。
因此长度为 `n` 的小数前缀天然是一个分层二维网格键，类似 centered geohash。
截断 `n` 位后的每个坐标误差满足

\[
|e_x|,|e_y|\leq\sum_{k=n+1}^{\infty}3^{-k}
=\frac{1}{2\cdot3^n},
\]

欧氏误差至多 `1/(sqrt(2)·3^n)`。这适合：

- 以原点为中心、可正可负的多分辨率平面索引；
- 图像金字塔、稀疏栅格、分块缓存和 level-of-detail；
- 精确的 90° 旋转、镜像和整数尺度缩放。

但洛书的字符顺序 `1..9` 本身不保持邻近性；若要数据库范围扫描，应另外设计
空间遍历顺序（例如 Morton/Hilbert 风格），不要直接按字符串字典序排序。

### 3-adic 视角

如果向更高位无限延伸，而不是向小数位延伸，就得到逆极限

\[
\varprojlim_n \mathbb Z[i]/3^n\mathbb Z[i],
\]

也就是 `Q3(i)` 的整数环；它是 `Q3` 的二次非分歧扩张，剩余域正是 `F9`。
在这种度量下，“低位相同很多位”表示两个数很接近，和欧氏空间里比较小数
前缀的方向恰好相反。这可以用于研究同余、自动机和逐位提升（Hensel lifting）。

## 4. 值得继续实现的四个小项目

1. `F9` 模块：已实现类型安全的无进位加减乘、幂、逆元、除法、Frobenius、
   域迹和域范数；下一步可以在其上增加多项式与纠错码实验。
2. `SpatialKey`：固定深度、边界盒、父子格、邻居和 Morton/Hilbert 排序层。
3. `GaussianInteger`：只允许整数位，提供精确范数、gcd 与高斯素数分解实验。
4. `Dbt3Adic<N>`：固定 `N` 个低位，所有运算模 `3^N`，用于同余与自动机。

其中 `F9` 的标量层已经落地，固定深度空间键仍最贴近现有表示；3-adic 与高斯
素数方向更偏研究型，但能真正利用这套表示法独有的“有限域个位 + 多位进位”。

更具体的 API 扩展顺序可以是：

1. `F9Polynomial::{evaluate, div_rem, gcd}`，再实现长度不超过 9 的小型
   Reed–Solomon 编解码实验；原始 Reed–Solomon 构造正是对有限域上的多项式
   在域元素处求值。
2. `GaussianInteger::{div_rem, gcd, extended_gcd}`；高斯整数以范数支持欧几里得
   除法，所以还可自然得到 Bézout 系数、互素判定和模逆元。
3. `SpatialKey::{parent, children, neighbors, bounds}`，并把“数值的加法”和
   “固定深度网格键的邻接”分成两个类型，避免边界与精度语义混在一起。
4. `Dbt3Adic<N>::valuation`、模 `3^N` 逆元和逐位提升；这一层的接近关系由
   低位共同前缀决定，适合有限自动机，而不是欧氏坐标距离。

## 5. 当前实现边界

- 从 `f64` 转换只能得到有限精度近似；精确有理数需要新增整数/分数构造器。
- `/` 使用固定工作精度；需要可控误差时应新增显式 precision/context API。
- 无限实数展开在 3-adic 边界和方格边界上会出现类似 `0.999…` 的多表示问题；
  固定长度键必须规定规范化策略。
- 现有二进制格式为兼容性保留，空间利用率不高；新格式应带版本字节，不能静默
  改写旧格式。

## 参考

- [双平衡三进制（百度百科）](https://baike.baidu.com/item/%E5%8F%8C%E5%B9%B3%E8%A1%A1%E4%B8%89%E8%BF%9B%E5%88%B6/10423772)
- [Canonical number systems for complex integers (1975)](https://acta.bibl.u-szeged.hu/14536/)
- [Number systems over orders](https://pmc.ncbi.nlm.nih.gov/articles/PMC6190796/)
- [On a generalization of the radix representation — a survey](https://math.tsukuba.ac.jp/~akiyama/papers/cnsams.pdf)
- [Polynomial Codes Over Certain Finite Fields (Reed–Solomon, 1960)](https://doi.org/10.1137/0108018)
- [A Division Algorithm for the Gaussian Integers' Minimal Euclidean Function](https://arxiv.org/abs/2502.21136)
- [Discrete phase space based on finite fields](https://arxiv.org/abs/quant-ph/0401155)
- [Automata as p-adic Dynamical Systems](https://arxiv.org/abs/1709.02644)
