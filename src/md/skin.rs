use {
    crossterm::style::{Attribute::*, Color::*},
    minimad::Compound,
    termimad::*,
};

pub fn make_skin(color: bool) -> MadSkin {
    if color {
        make_color_skin()
    } else {
        make_no_color_skin()
    }
}

fn make_color_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(AnsiValue(178));
    skin.headers[1].compound_style.remove_attr(Underlined);
    skin.italic.remove_attr(Italic);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(AnsiValue(204));
    skin.special_chars.insert(
        Compound::raw_str("U").code(),
        StyledChar::from_fg_char(Green, '➚'),
    );
    skin.special_chars.insert(
        Compound::raw_str("D").code(),
        StyledChar::from_fg_char(Red, '➘'),
    );
    skin
}

fn make_no_color_skin() -> MadSkin {
    let mut skin = MadSkin::no_style();
    skin.special_chars.insert(
        Compound::raw_str("U").code(),
        StyledChar::nude('➚'),
    );
    skin.special_chars.insert(
        Compound::raw_str("D").code(),
        StyledChar::nude('➘'),
    );
    skin
}
