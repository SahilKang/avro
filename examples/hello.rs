extern crate avro;
#[macro_use] extern crate serde_derive;

use avro::Codec;
use avro::from_value;
use avro::reader::Reader;
use avro::schema::Schema;
use avro::types::{Record, ToAvro};
use avro::writer::Writer;

#[derive(Debug, Deserialize, Serialize)]
struct Test {
    a: i64,
    b: String,
}

fn main() {
    let raw_schema = r#"
        {
            "type": "record",
            "name": "test",
            "fields": [
                {"name": "a", "type": "long", "default": 42},
                {"name": "b", "type": "string"}
            ]
        }
    "#;

    let schema = Schema::parse_str(raw_schema).unwrap();

    println!("{:?}", schema);

    let mut writer = Writer::with_codec(&schema, Vec::new(), Codec::Deflate);

    let mut record = Record::new(writer.schema()).unwrap();
    // record.put("a", 27);
    record.put("b", "foo");

    // writer.append(record.avro()).unwrap();

    let test = Test {
        a: 27,
        b: "foo".to_owned(),
    };

    writer.append(test).unwrap();

    let input = writer.into_inner();
    let reader = Reader::new(&schema, &input[..]);

    for record in reader {
        println!("{:?}", from_value::<Test>(&record));
    }
}
