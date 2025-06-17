# TOO BIG FLOAT
- ものすごく大きな数を表現するためのライブラリ
- 1e1e100とか

## やりたいこと
- 加減乗除
- 指数対数関数
- 文字列への変換と文字列のパース
- float/doubleとの相互変換と加減乗除。桁あふれはinfなどにする。

## 実装の方針
- rust
- ついててほしいTraitはなるべくつける
- BigFloat: mantissa, exponent partからなるstruct
- mantissaは`[1, 10)`, exponent_partはlong|BigFloat
- baseは10だけでいい
- 掛け算割り算と指数対数はdoubleにcastせず仮数と指数に対する演算で実装
- 足し算引き算は大きい方にスケールを合わせて実装。100+1=BigFloat(1.0, 2.0)-BigFloat(1,0.0)=BigFloat(1+0.01, 2)など。doubleの精度で足りない分は無視。