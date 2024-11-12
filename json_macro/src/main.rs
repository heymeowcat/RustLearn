#[macro_export]
macro_rules! json {
    ($key:ident : $value:expr) => {
        {
            let mut map = std::collections::HashMap::new();
            map.insert(stringify!($key).to_string(), $value);
            map
        }
    };

    ($($key:ident : $value:expr),+ $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert(stringify!($key).to_string(), $value);
            )+
            map
        }
    };
}

fn main() {
    let single = json!(name : "Vidura");
    println!("{:?}", single);

    let data = json!(
        name : "Vidura",
        age : "24",
        city : "Gampaha",
    );

    println!("{:?}", data);
}
