#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "gmap")]
mod maps;

#[macro_export]
macro_rules! exit_with {
    ($x:literal) => {{
        eprintln!($x);
        std::process::exit(1);
    }};
    ($x:literal,$code:literal) => {{
        eprintln!($x);
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! match_exit {
    ($result:expr,$exit_print:literal) => {
        match $result {
            Ok(val) => val,
            Err(_) => {
                exit_with!($exit_print);
            }
        }
    };
    ($result:expr,$exit_print:literal,$code:literal) => {
        match $result {
            Ok(val) => val,
            Err(_) => {
                exit_with!($exit_print);
            }
        }
    };
}

#[macro_export]
macro_rules! map {
    // more verbose
    ($func:block on $vector:expr) => {
        $vector.iter().map($func).collect()
    };
    ($func:block into $vector:expr) => {
        $vector.into_iter().map($func).collect()
    };
    // normal
    ($vector:expr,$func:expr) => {
        $vector.iter().map($func).collect()
    };
    (into $vector:expr,$func:expr) => {
        $vector.into_iter().map($func).collect()
    };
}

#[macro_export]
macro_rules! filter {
    ($vector:expr,$func:expr) => {
        $vector.iter().filter($func).collect()
    };
    (into $vector:expr,$func:expr) => {
        $vector.into_iter().filter($func).collect()
    };
}

#[macro_export]
macro_rules! filter_map {
    ($vector:expr,$func:expr) => {
        $vector.iter().filter_map($func).collect()
    };

    (into $vector:expr,$func:expr) => {
        $vector.into_iter().filter_map($func).collect()
    };
}

#[macro_export]
macro_rules! tern {
    [$cond:expr => $true:expr ; $false:expr] => {
        if $cond {$true}
        else {$false}
    };
}

#[macro_export]
macro_rules! s_vec {
    [$($x:expr),*] => (vec![$($x.to_string()),*]);
}

#[macro_export]
macro_rules! const_declare {
    ($type:ty; $x:ident,$y:expr) => {
        const $x: $type = $y;
    };
    ($type:ty; $x:ident,$y:expr, $($nx:ident,$ny:expr),+) => {
        const $x: $type = $y;
        const_declare!{$type; $($nx,$ny),+}
    };
}

#[macro_export]
macro_rules! sleep {
    ($time:expr; ns) => {
        std::thread::sleep(std::time::Duration::from_nanos($time));
    };
    ($time:expr; us) => {
        std::thread::sleep(std::time::Duration::from_micros($time));
    };
    ($time:expr; ms) => {
        std::thread::sleep(std::time::Duration::from_millis($time));
    };
    ($time:expr; s) => {
        std::thread::sleep(std::time::Duration::from_secs($time));
    };
    ($time:expr; fs) => {
        std::thread::sleep(std::time::Duration::from_secs_f64($time));
    };
}

#[cfg(test)]
mod tests {
    use super::{filter, filter_map, map, s_vec, tern};

    #[test]
    fn test_tern() {
        assert!(tern![1<2 => true ; false]);
        assert_eq!(false, tern![2<1 => true;false]);
    }

    #[test]
    fn test_map() {
        let base = vec![1, 2, 3, 4, 5];
        let comp = vec![2, 4, 6, 8, 10];
        let anss: Vec<i32> = map!({|x|{x*2}} on base);
        assert_eq!(comp, anss);
        let ans2: Vec<i32> = map!({|x|{x*2}} into base);
        assert_eq!(comp, ans2);
    }

    #[test]
    fn test_filter() {
        let base = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![&2, &4, &6, &8];
        let result: Vec<&i32> = filter!(base, |x| (*x) % 2 == 0);
        assert_eq!(expected, result);
        let expected = vec![2, 4, 6, 8];
        let result: Vec<i32> = filter!(into base,|x| x%2 == 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_filter_map() {
        let base = vec!["1", "two", "tree", "5"];
        let expected = vec![1, 5];
        let result: Vec<i32> = filter_map!(base, |s| s.parse().ok());
        assert_eq!(expected, result);
        let expected = vec![1, 5];
        let result: Vec<i32> = filter_map!(into base,|s| s.parse().ok());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_string_vector() {
        let expected = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ];
        let actual = s_vec!["1", "2", "3", "4"];
        assert_eq!(expected, actual);
    }
}
