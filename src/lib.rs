#[cfg(feature = "json")]
pub mod json;

#[macro_export]
macro_rules! exit_with {
    ($x:literal) => {{
        println!($x);
        std::process::exit(1);
    }};
    ($x:literal,$code:literal) => {{
        println!($x);
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! match_exit {
    ($result:expr,$exit_print:literal) => {
        match $result {
            Ok(val) => val,
            Err(_) => {
                eprintln!($exit_print);
                std::process::exit(1);
            }
        }
    };
    ($result:expr,$exit_print:literal,$code:literal) => {
        match $result {
            Ok(val) => val,
            Err(_) => {
                eprintln!($exit_print);
                std::process::exit($code);
            }
        }
    };
}

#[macro_export]
macro_rules! iter_map {
    ($vector:expr,$func:expr) => {
        $vector.iter().map($func).collect()
    };
    (into $vector:expr,$func:expr) => {
        $vector.into_iter().map($func).collect()
    };
}

#[macro_export]
macro_rules! iter_filter {
    ($vector:expr,$func:expr) => {
        $vector.iter().filter($func).collect()
    };
    (into $vector:expr,$func:expr) => {
        $vector.into_iter().filter($func).collect()
    };
}

#[macro_export]
macro_rules! iter_filter_map {
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
    else{$false}
};
}

#[macro_export]
macro_rules! svec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod tests {
    use super::{iter_filter, iter_filter_map, iter_map, tern};
    #[test]
    fn test_tern() {
        assert!(tern![1<2 => true ; false]);
        assert_eq!(false, tern![2<1 => true;false]);
    }

    #[test]
    fn test_iter_map() {
        let base = vec![1, 2, 3, 4, 5];
        let comp = vec![2, 4, 6, 8, 10];
        let ans: Vec<i32> = iter_map!(base, |x| { x * 2 });
        let ans2: Vec<i32> = iter_map!(into base, |x|{x*2});
        assert_eq!(comp, ans);
        assert_eq!(comp, ans2);
    }

    #[test]
    fn test_iter_filter() {
        let base = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![&2, &4, &6, &8];
        let result: Vec<&i32> = iter_filter!(base, |x| (*x) % 2 == 0);
        assert_eq!(expected, result);
        let expected = vec![2, 4, 6, 8];
        let result: Vec<i32> = iter_filter!(into base,|x| x%2 == 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_iter_filter_map() {
        let base = vec!["1", "two", "tree", "5"];
        let expected = vec![1, 5];
        let result: Vec<i32> = iter_filter_map!(base, |s| s.parse().ok());
        assert_eq!(expected, result);
        let expected = vec![1, 5];
        let result: Vec<i32> = iter_filter_map!(into base,|s| s.parse().ok());
        assert_eq!(expected, result);
    }
}
