#[macro_export]
macro_rules! json_travel {
	    ($jsn:expr,$x:expr) => (&$jsn[$x]);

        ($jsn:expr,$x:expr,$($y:expr),*) => (
            json_travel!($jsn[$x],$($y),*)
        )
    }

#[macro_export]
macro_rules! json_write_file {
    ($x:expr,$file:expr) => {
        let j = serde_json::to_string($x).expect("Coudnot serialize");
        let mut fp = std::fs::File::create($file).expect("could not create File");
        fp.write(j.as_bytes()).expect("failed to write to file");
    };
}
