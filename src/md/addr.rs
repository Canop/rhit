use {
    super::*,
    crate::*,
    have::Fun,
    itertools::*,
    std::cmp::Reverse,
    minimad::OwningTemplateExpander,
    termimad::*,
};


static MD: &str = r#"
## ${remote-addr-count} distinct remote adresses. ${remote-addr-limit} most used:
|:-:|:-|:-:
|**#**|**remote address**|**hits**
|-:|:-|-:
${popular-remote-addresses
|${idx}|${remote-address}|${count}
}
|-:
"#;

pub fn print_popular_remote_addresses(
    log_lines: &[LogLine],
    detail_level: usize,
    skin: &MadSkin,
) {
    let mut expander = OwningTemplateExpander::new();
    let n = match detail_level {
        0 => 3,
        1 => 5,
        l => l * 10,
    };
    log_lines.iter()
        .into_group_map_by(|ll| ll.remote_addr)
        .fun(|g| {
            expander
                .set("remote-addr-count", g.len())
                .set("remote-addr-limit", n);
        })
        .into_iter()
        .sorted_by_key(|e| Reverse(e.1.len()))
        .take(n)
        .enumerate()
        .for_each(|(idx, e)| {
            expander.sub("popular-remote-addresses")
                .set("idx", idx+1)
                .set("remote-address", e.0)
                .set("count", e.1.len());
        });
    print(expander, MD, skin);
}

