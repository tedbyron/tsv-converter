//! Style for UI elements.

use iced::{button, container, rule};

pub mod color;
pub mod font;

use self::color::{
    BG, BG_DARK, BG_DARKER, BG_LIGHT, BG_LIGHTER, FG, GRAY, GRAY_PURPLE, PURPLE, PURPLE_GRAY, RED,
};

macro_rules! style {
    ($ty:ty, $ele:ident, $style:tt) => {
        impl $ele::StyleSheet for $ty {
            fn style(&self) -> $ele::Style {
                $ele::Style $style
            }
        }
    };
    ($ty:ty, $ele:ident, $( $F:ident : $style:tt ),+) => {
        impl $ele::StyleSheet for $ty {
            $(fn $F(&self) -> $ele::Style {
                $ele::Style $style
            })+
        }
    };
}

pub struct Main;

style!(Main, container, {
    background: (*BG).into(),
    text_color: (*FG).into(),
    ..container::Style::default()
});

style!(Main, button,
    active: {
        background: (*BG_DARK).into(),
        border_width: 1.0,
        border_color: *PURPLE_GRAY,
        text_color: *FG,
        ..button::Style::default()
    },
    hovered: {
        background: (*BG_LIGHT).into(),
        border_width: 1.0,
        border_color: *PURPLE,
        text_color: *FG,
        ..button::Style::default()
    },
    pressed: {
        background: (*BG).into(),
        border_width: 1.0,
        border_color: *PURPLE,
        text_color: *FG,
        ..button::Style::default()
    },
    disabled: {
        background: (*GRAY).into(),
        text_color: *PURPLE_GRAY,
        ..button::Style::default()
    }
);

pub struct TextButton;

style!(TextButton, button,
    active: {
        text_color: *FG,
        ..button::Style::default()
    },
    hovered: {
        text_color: *PURPLE,
        ..button::Style::default()
    },
    pressed: {
        text_color: *PURPLE,
        ..button::Style::default()
    }
);

pub struct Menu;

style!(Menu, container, {
    background: (*BG_LIGHT).into(),
    ..container::Style::default()
});

style!(Menu, rule, {
    color: *GRAY_PURPLE,
    ..rule::Style::default()
});

pub struct Notification;

style!(Notification, container, {
    text_color: (*BG_DARKER).into(),
    background: (*RED).into(),
    ..container::Style::default()
});

style!(Notification, button,
    active: {
        background: (*BG).into(),
        border_width: 1.0,
        border_color: *PURPLE_GRAY,
        text_color: *FG,
        ..button::Style::default()
    },
    hovered: {
        background: (*BG_LIGHTER).into(),
        border_width: 1.0,
        border_color: *PURPLE,
        text_color: *FG,
        ..button::Style::default()
    },
    pressed: {
        background: (*BG_LIGHT).into(),
        border_width: 1.0,
        border_color: *PURPLE,
        text_color: *FG,
        ..button::Style::default()
    },
    disabled: {
        background: (*GRAY).into(),
        text_color: *PURPLE_GRAY,
        ..button::Style::default()
    }
);
