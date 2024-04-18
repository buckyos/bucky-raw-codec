# bucky-raw-codec

[Chinese](./doc/README_CN.md)

A library that implements data encoding in buckyos, supports buckyos custom encoding and supports protobuf encoding.

```toml
bucky-raw-codec = {version = "0.1", feature = ["derive"]}
```

raw encoding usage：

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

protobuf encoding usage：

1. Project configuration：

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

2. Simple structure：

As shown in the following code, when a simple structure is encoded into protobuf, the ProtobufTransform macro must be added in front of the structure, and the cyfs_protobuf_type attribute must also be set to indicate the corresponding proto generated object structure. The corresponding message structure is defined in the proto file, and the member names in the message must be consistent with the members in rust.

If you want to add RawEncode and RawDecode trait implementations to the structure, you can add the ProtobufEncode, ProtobufDecode macros.

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

3. Enumeration：

No data enumeration

The bucky_protobuf_type attribute needs to be set to the i32 type. The encoded value is a value starting from 0 according to the definition order of the enumeration value.

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

​	Data enumeration

Now the macro only supports one data member in the enumeration. If there are multiple data members, they need to be converted into struct. The definition in proto must be used oneof in the message. 

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

4. Empty structure

```rust
#[derive(Default, ProtobufEmptyEncode, ProtobufEmptyDecode)]
pub struct Empty {

}
```

5. Existing type mapping

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

6. Customized ProtobufTransform interface implementation

   If the internal data structure is too complex, you can customize the ProtobufTransform interface implementation, as shown in the following code:

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