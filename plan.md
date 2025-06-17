# TOO BIG FLOAT Rust実装計画

## 概要
1e1e100などの超大きな数を表現するRustライブラリの実装計画

## 基本設計

### BigFloat構造体
```rust
pub struct BigFloat {
    mantissa: f64,      // [1, 10)の範囲の仮数
    exponent: Exponent, // 指数部（long値またはBigFloat）
}

pub enum Exponent {
    Long(i64),
    BigFloat(Box<BigFloat>),
}
```

## 実装フェーズ

### Phase 1: 基本構造体とコンストラクタ
- [ ] BigFloat構造体の定義
- [ ] Exponent enum の定義
- [ ] 基本コンストラクタの実装
- [ ] 正規化処理（mantissaを[1, 10)に収める）

### Phase 2: 基本演算
- [ ] 加算実装（スケール合わせ）
- [ ] 減算実装（スケール合わせ）
- [ ] 乗算実装（仮数と指数の分離演算）
- [ ] 除算実装（仮数と指数の分離演算）

### Phase 3: 数学関数
- [ ] 指数関数（exp）
- [ ] 対数関数（log）
- [ ] べき乗関数（pow）

### Phase 4: 型変換とパース
- [ ] 文字列からのパース
- [ ] 文字列への変換（Display trait）
- [ ] f64/f32との相互変換
- [ ] オーバーフロー時のinf処理

### Phase 5: Traitの実装
- [ ] Display, Debug
- [ ] PartialEq, PartialOrd
- [ ] Add, Sub, Mul, Div
- [ ] From<f64>, Into<f64>
- [ ] Clone, Copy（可能な場合）

## 具体的な実装方針

### 加減算
- 大きい方の数にスケールを合わせる
- 例: 100 + 1 = BigFloat(1.0, 2) + BigFloat(1.0, 0) = BigFloat(1.01, 2)
- doubleの精度を超える部分は切り捨て

### 乗除算
- doubleにキャストせず、仮数と指数を分離して計算
- mantissa同士の演算とexponent同士の演算を独立して実行

### 指数部の処理
- 通常の数値範囲内ならi64で管理
- 範囲を超える場合はBigFloatで再帰的に管理

## テスト計画
- [ ] 基本演算のユニットテスト
- [ ] 極大値での演算テスト
- [ ] エッジケースのテスト（0, inf, NaN）
- [ ] 精度テスト
- [ ] パフォーマンステスト

## ファイル構成
```
src/
├── lib.rs           # モジュール定義
├── bigfloat.rs      # BigFloat構造体とコア実装
├── arithmetic.rs    # 四則演算
├── math.rs          # 数学関数
├── convert.rs       # 型変換とパース
└── traits.rs        # Trait実装
```