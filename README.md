# 作业

### 编写存证模块的单元测试代码，包括：
* 创建存证的测试用例
* 撤销存证的测试用例
* 转移存证的测试用例

![作业](https://i.imgur.com/3o468yf.png)


### 创建存证时，为存证内容的哈希值 Vec
* 设置长度上限，超过限制时返回错误
* 并编写测试用例

![作业](https://i.imgur.com/3o468yf.png)

```rust
#[test]
fn create_claim_failed_when_claim_is_too_long() {
    new_test_ext().execute_with(|| {
        // 一个长度超过限制的凭证
        let claim = vec![
            0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8,
        ];
        // 创建凭证时检查是否报错
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofTooLong
        );
    })
}
```