### ソフトウェア工学演習

- 第八回の内容を踏まえて数学でのベクトルを扱うためのライブラリ作成でTDDを実践した
- 演習に用いたのはRust
  - 組み込みフレームワークにより自動化テスト
  - GitHub ActionsでCIを構成

最終的に確認したところcoverageは 96.84% であった。
```
yuya@dp101 ~/w/mini-math (hotfix/validate)> cargo llvm-cov
info: cargo-llvm-cov currently setting cfg(coverage); you can opt-out it by passing --no-cfg-coverage
   Compiling mini-math v0.1.0 (/home/yuya/workspace/mini-math)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/mini_math-efa7c78a2c97ee5e)

running 12 tests
test vector::tests::test_add ... ok
test vector::tests::test_add_failure_on_different_dimensions ... ok
test vector::tests::test_equals_on_different_dimensions ... ok
test vector::tests::test_add_failure_on_overflow ... ok
test vector::tests::test_scale_failure_on_overflow ... ok
test vector::tests::test_scale ... ok
test vector::tests::test_equals_on_different_vectors ... ok
test vector::tests::test_try_new_success_on_finite ... ok
test vector::tests::test_dimensions ... ok
test vector::tests::test_equals_on_same_vector ... ok
test vector::tests::test_try_new_failure_on_infinite ... ok
test vector::tests::test_try_new_failure_on_zero_dimension ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Filename                                         Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
/home/yuya/workspace/mini-math/src/vector.rs         214                 3    98.60%          19                 1    94.74%          95                 3    96.84%           0                 0         -
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                                                214                 3    98.60%          19                 1    94.74%          95                 3    96.84%           0                 0         -
```