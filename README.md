# rvalue
> rust版本的万能值变量，目标是支持json,BSON,msgpack等
# 用法
```rust
    let mut dvalue = DxValue::newObject();
    println!("{}",dvalue);
    dvalue.setKeyString("name","不得闲");
    println!("{}",dvalue);
    if let Some(st) = dvalue.value_byName("name"){
        *st = DxValue::Int(32);
    }
    println!("{}",dvalue);
```
