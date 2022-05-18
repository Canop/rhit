
use {
    glassbench::*,
    rhit::*,
    rhit::args::Args,
    std::path::PathBuf,
};

fn filter_resources(bench: &mut Bench) {
    bench.task("filter resources", |task| {
        let package_dir = std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir not set");
        let paths = vec![PathBuf::from(package_dir).join("test-data/")];
        let mut args = Args::default();
        args.silent_load = true;
        let base = LogBase::new(&paths, &args).unwrap();
        assert_eq!(base.lines.len(), 33468);
        task.iter(|| {
            let count = base.lines.iter()
                .filter(|line| line.is_resource())
                .count();
            assert_eq!(count, 5772);
            pretend_used(count);
        });
    });
}

glassbench!(
    "Resources filtering",
    filter_resources,
);
