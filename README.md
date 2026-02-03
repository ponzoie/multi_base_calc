Multi-base integer calculator (Rust + WASM)  
Codexのみで作成

要件・制約（概要）
- 整数式のみ（浮動小数点なし）
- リテラル: 2進 `0b...`, 16進 `0x...`, 10進（接頭辞なし）
- `_` 区切りを許可（例: `0x1_FF`, `1_000`）
- 演算子: `+ - * %`、括弧 `()`、単項マイナス
- 空白は無視
- 余り `%` は Euclidean（`0 <= r < |m|`、`m == 0` はエラー）
- 内部は `i64` で計算するが、常に 32-bit 符号付き範囲 `[-2^31, 2^31-1]` を強制
  - リテラル/演算結果が範囲外ならエラー
- 結果は bin / dec / hex を同時に表示

起動・コマンド
- テスト実行（全体）:
  - `cargo test`
- WASM UI 起動（開発用）:
  - `cd crates/calc_wasm`
  - `trunk serve`
- WASM ビルド:
  - `cd crates/calc_wasm`
  - `trunk build`
