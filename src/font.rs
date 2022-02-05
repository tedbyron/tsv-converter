use iced::Font;

pub const _IOSEVKA: Font = Font::External {
    name: "Iosevka",
    bytes: include_bytes!("../assets/fonts/iosevka-ss06-extended.ttf"),
};

pub const _IOSEVKA_BOLD: Font = Font::External {
    name: "Iosevka Bold",
    bytes: include_bytes!("../assets/fonts/iosevka-ss06-extendedbold.ttf"),
};

pub const IOSEVKA_HEAVY_ITALIC: Font = Font::External {
    name: "Iosevka Heavy Italic",
    bytes: include_bytes!("../assets/fonts/iosevka-ss06-extendedheavyitalic.ttf"),
};

pub const _IOSEVKA_LIGHT_ITALIC: Font = Font::External {
    name: "Iosevka Light Italic",
    bytes: include_bytes!("../assets/fonts/iosevka-ss06-extendedlightitalic.ttf"),
};
