
use {
    glassbench::*,
    rhit::*,
    rhit::args::Args,
    std::path::PathBuf,
};

fn bench_base_loading(bench: &mut Bench) {
    bench.task("read access.log", |task| {
        let package_dir = std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir not set");
        let paths = vec![PathBuf::from(package_dir).join("test-data/")];
        let mut args = Args::default();
        args.silent_load = true;
        task.iter(|| {
            let base = LogBase::new(&paths, &args).unwrap();
            assert_eq!(base.lines.len(), 33468);
            pretend_used(base);
        });
    });
}

glassbench!(
    "Log Base Loading",
    bench_base_loading,
);
