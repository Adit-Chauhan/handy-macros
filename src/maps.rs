macro_rules! gmap {
    ($map:ident <$key:ty,$val:ty>) => {
        lazy_static::lazy_static! {
            static ref $map: std::sync::Mutex<std::collections::HashMap<$key,$val>> = {
                let m = std::collections::HashMap::new();
                std::sync::Mutex::new(m)
            };
        }
    };
    ($map:ident <$key:ty,$val:ty> with $vals:expr) => {
        lazy_static::lazy_static! {
            static ref $map: std::sync::Mutex<std::collections::HashMap<$key,$val>> = {
                let m = std::collections::HashMap::from($vals);
                std::sync::Mutex::new(m)
            };
        }
    };
}

#[cfg(test)]
mod tests {

    gmap!(HASHMAP<i32,i32>);
    gmap!(HASHMAP2<i32,i32> with [(1,1),(2,2),(3,3)]);

    #[test]
    fn test_maps() {
        let mut map_1 = HASHMAP.lock().unwrap();
        map_1.insert(1, 1);
        map_1.insert(2, 2);
        map_1.insert(3, 3);
        let map_2 = HASHMAP2.lock().unwrap();
        if map_1.keys().len() != map_2.keys().len() {
            assert!(false, "keys quantity not same");
            return;
        }
        for k in map_1.keys() {
            assert_eq!(map_1.get(k), map_2.get(k));
        }
    }
}
