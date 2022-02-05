use iced::{container, rule};

pub struct Main;
impl container::StyleSheet for Main {
    fn style(&self) -> container::Style {
        container::Style {
            background: (*crate::color::BG).into(),
            text_color: (*crate::color::FG).into(),
            ..container::Style::default()
        }
    }
}

pub struct Menu;
impl container::StyleSheet for Menu {
    fn style(&self) -> container::Style {
        container::Style {
            background: (*crate::color::BG_LIGHT).into(),
            ..container::Style::default()
        }
    }
}

pub struct MenuRule;
impl rule::StyleSheet for MenuRule {
    fn style(&self) -> rule::Style {
        rule::Style {
            color: *crate::color::GRAY_PURPLE,
            ..rule::Style::default()
        }
    }
}
