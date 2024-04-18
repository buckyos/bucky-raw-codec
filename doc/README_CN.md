# bucky-raw-codec

实现buckyos中数据编码的库，支持buckyos自定义的编码以及支持protobuf编码

```toml
bucky-raw-codec = {version = "0.1", feature = ["derive"]}
```

自定义编码使用：

```rust
#[derive(RawEncode, RawDecode)]
struct Test {
    t1: u32,
    t2: String,
}

#[derive(RawEncode, RawDecode)]
struct Test2 {
    t1: Test,
}

let t = Test2 {
    t1: Test {
        t1: 1,
        t2: "test".to_string(),
    }
};

let buf = t.to_vec().unwrap();
let t2 = Test2::clone_from_slice(&buf).unwrap();
assert_eq!(t.t1.t1, t2.t1.t1);
assert_eq!(t.t1.t2, t2.t1.t2);
```

protobuf编码使用：

1. 工程配置：

```
[dependencies]
prost = {version = "0.9.0"}

[build-dependencies]
prost-build = {version = "0.9.0"}
```

build.rs

```rust
fn main() {
   set_var("OUT_DIR", "src/proto_rs/");
   let content = r#"
   mod test;
   pub use test::*;
   "#;
   let path = Path::new("src/proto_rs");
   if !path.exists() {
	   create_dir_all(path).unwrap();
   }
   std::fs::write("src/proto_rs/mod.rs", content).unwrap();
   let mut config = prost_build::Config::new();
   config.compile_protos(&["src/proto/test.proto"],
						 &["src/proto"]).unwrap();
}
```

2. 普通结构：

如下代码所示，普通结构编码成protobuf时，结构前面必须加上ProtobufTransform宏，同时也要设置bucky_protobuf_type属性指示对应的proto生成对象结构。proto文件中定义对应的message结构，message中的成员名称必须更rust中成员一致。

如果要为结构添加RawEncode和RawDecode trait实现，可添加ProtobufEncode, ProtobufDecode宏

```rust
#[derive(ProtobufTransform)]
#[bucky_protobuf_type(crate::proto_rs::TestSubStruct)]
pub struct TestSubStruct {
	data: Vec<u8>,
}

#[derive(ProtobufEncode, ProtobufDecode, ProtobufTransform)]
#[bucky_protobuf_type(crate::proto_rs::TestStruct)]
pub struct TestStruct {
	string_type: String,
	bytes_type: Vec<u8>,
	float_type: f32,
	double_type: f64,
	u8_type: u8,
	u16_type: u16,
	u32_type: u32,
	bool_type: bool,
	self_type: TestSubStruct,
	option_type: Option<TestSubStruct>,
}
```

```protobuf
message TestSubStruct {
  bytes data = 1;
}

message TestStruct {
  string string_type = 1;
  bytes bytes_type = 2;
  float float_type = 3;
  double double_type = 4;
  int32 u8_type = 5;
  int32 u16_type = 6;
  uint32 u32_type = 7;
  bool bool_type = 8;
  TestSubStruct self_type = 9;
  optional TestSubStruct option_type = 10;
}
```

3. 枚举定义：

无数据枚举

bucky_protobuf_type属性需设置为i32类型，编码的值根据枚举值的定义顺序从0开始的数值

```rust
#[derive(ProtobufTransform)]
#[bucky_protobuf_type(i32)]
pub enum TestEnum {
   Test1,
   Test2,
   Test3,
}
#[derive(ProtobufTransform)]
#[bucky_protobuf_type(crate::proto_rs::Test)]
pub struct Test {
   test_enum: TestEnum
}
```

```protobuf
message Test {
	int32 test_enum = 1;
}
```

​	有数据枚举

现在宏只支持枚举中存在一个数据成员，如果存在多个数据成员需转成struct，proto中的定义必须在使用的message中用oneof

```rust
#[derive(ProtobufTransform)]
#[bucky_protobuf_type(crate::proto_rs::test::TestEnum)]
pub enum TestEnum {
	Test1(i32),
	Test2(i32),
}

#[derive(ProtobufTransform)]
#[bucky_protobuf_type(crate::proto_rs::Test)]
pub struct Test {
	test_enum: TestEnum
}
```

```protobuf
message Test {
	oneof TestEnum {
		int32 test1 = 1;
		int32 test2 = 2;
	};
}
```

4. 空结构定义

```rust
#[derive(Default, ProtobufEmptyEncode, ProtobufEmptyDecode)]
pub struct Empty {

}
```

5. 已有类型映射

|    rust     | protobuf |
| :---------: | :------: |
|  ObjectId   |  bytes   |
|   ChunkId   |  bytes   |
| CoinTokenId |  bytes   |
|     u8      |  uint32  |
|     i8      |  int32   |
|     u16     |  uint32  |
|     i16     |  int32   |
|   HashMap   |   map    |

6. 自定义ProtobufTransform接口实现

   如果内部数据结构太复杂，则可以自定义ProtobufTransform接口实现，如下代码所式：

```
message ParamStruct {
string key = 1;
string value = 2;
}

message ComplexStruct {
repeated ParamStruct param = 1;
}
```

```
#[derive(Clone, ProtobufEncode, ProtobufDecode, ProtobufTransformType)]
#[bucky_protobuf_type(crate::codec::protos::ComplexStruct)]
pub struct ComplexStruct {
    params: HashMap<String, String>,
}

impl ProtobufTransform<protos::ComplexStruct> for ComplexStruct {
    fn transform(value: protos::ComplexStruct) -> BuckyResult<Self> {
        let mut params: HashMap<String, String> = HashMap::new();
        for item in value.param {
            let key = item.key;
            let value = item.value;
            if let Some(old) = params.insert(key, value) {
                error!("decode ComplexStruct param but got repeated item: {}", old);
            }
        }

        Ok(Self { params })
    }
}

impl ProtobufTransform<&ComplexStruct> for protos::ComplexStruct {
    fn transform(value: &ComplexStruct) -> BuckyResult<Self> {
        let mut ret = Self { param: vec![] };
        let mut params = Vec::new();
        for (k, v) in &value.params {
            let item = protos::ComplexStruct {
                key: k.to_string(),
                value: v.to_string(),
            };
            params.push(item);
        }
        ret.param = params.into();

        Ok(ret)
    }
}
```