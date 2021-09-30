use heck::TitleCase;
use iced::{container, svg, Color, Column, Container, Length, Row, Svg, Text};

use bl3_save_edit_core::bl3_item::{Bl3Item, ItemFlags, ItemRarity, ItemType};

use crate::bl3_ui::InteractionMessage;
use crate::resources::fonts::{JETBRAINS_MONO_BOLD, JETBRAINS_MONO_LIGHT_ITALIC};
use crate::resources::svgs::{FAVOURITE, TRASH};

pub fn view(item: &Bl3Item) -> Column<InteractionMessage> {
    let balance_part = item.balance_part();

    let label = balance_part.name.clone().unwrap_or_else(|| {
        balance_part
            .short_ident
            .clone()
            .unwrap_or_else(|| balance_part.ident.clone())
    });

    let mut tags_row = Row::new()
        .push(
            Container::new(
                Text::new(format!("Level {}", item.level()))
                    .font(JETBRAINS_MONO_LIGHT_ITALIC)
                    .size(15),
            )
            .padding(5)
            .style(ItemInfoStyle),
        )
        .width(Length::Fill)
        .spacing(10);

    if let Some(mut manufacturer_short) = item.manufacturer_part().short_ident.clone() {
        if manufacturer_short != "CoV" && manufacturer_short != "Class Mod" {
            manufacturer_short = manufacturer_short.to_title_case();
        }

        tags_row = tags_row.push(
            Container::new(
                Text::new(manufacturer_short)
                    .font(JETBRAINS_MONO_LIGHT_ITALIC)
                    .size(15),
            )
            .padding(5)
            .style(ItemInfoStyle),
        )
    }

    if let Some(item_parts) = &item.item_parts {
        if item.item_type == ItemType::Weapon {
            if let Some(weapon_type) = &item_parts.weapon_type {
                tags_row = tags_row.push(
                    Container::new(
                        Text::new(weapon_type.to_string())
                            .font(JETBRAINS_MONO_LIGHT_ITALIC)
                            .size(15),
                    )
                    .padding(5)
                    .style(ItemInfoStyle),
                )
            }
        }

        if item_parts.rarity != ItemRarity::Unknown {
            tags_row = tags_row.push(
                Container::new(
                    Text::new(item_parts.rarity.to_string())
                        .font(JETBRAINS_MONO_LIGHT_ITALIC)
                        .size(15),
                )
                .padding(5)
                .style(ItemRarityStyle {
                    rarity: item_parts.rarity.clone(),
                }),
            );
        }
    }

    if let Some(flags) = item.flags {
        let fav_or_trash = if flags.contains(ItemFlags::FAVOURITE) {
            let favourite_icon_handle = svg::Handle::from_memory(FAVOURITE);

            let favourite_icon = Svg::new(favourite_icon_handle)
                .height(Length::Units(15))
                .width(Length::Units(15));

            Some((
                favourite_icon,
                FavouriteTrashStyle::Favourite(ItemFavouriteStyle),
            ))
        } else if flags.contains(ItemFlags::TRASH) {
            let trash_icon_handle = svg::Handle::from_memory(TRASH);

            let trash_icon = Svg::new(trash_icon_handle)
                .height(Length::Units(15))
                .width(Length::Units(15));

            Some((trash_icon, FavouriteTrashStyle::Trash(ItemTrashStyle)))
        } else {
            None
        };

        if let Some((icon, style)) = fav_or_trash {
            let mut icon_content = Container::new(icon).padding(5);

            match style {
                FavouriteTrashStyle::Favourite(s) => icon_content = icon_content.style(s),
                FavouriteTrashStyle::Trash(s) => icon_content = icon_content.style(s),
            }

            tags_row = tags_row.push(icon_content)
        }
    }

    Column::new()
        .push(
            Text::new(&label)
                .font(JETBRAINS_MONO_BOLD)
                .size(17)
                .color(Color::from_rgb8(224, 224, 224)),
        )
        .push(tags_row)
        .spacing(10)
}

pub struct ItemInfoStyle;

impl container::StyleSheet for ItemInfoStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(224, 224, 224)),
            background: Some(Color::from_rgb8(38, 38, 38).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(46, 46, 46),
        }
    }
}

pub enum FavouriteTrashStyle {
    Favourite(ItemFavouriteStyle),
    Trash(ItemTrashStyle),
}

pub struct ItemFavouriteStyle;

impl container::StyleSheet for ItemFavouriteStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(19, 232, 240)),
            background: Some(Color::from_rgb8(29, 53, 54).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(36, 60, 61),
        }
    }
}

pub struct ItemTrashStyle;

impl container::StyleSheet for ItemTrashStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(240, 149, 149)),
            background: Some(Color::from_rgb8(54, 29, 29).into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(61, 36, 36),
        }
    }
}

pub struct ItemRarityStyle {
    rarity: ItemRarity,
}

impl container::StyleSheet for ItemRarityStyle {
    fn style(&self) -> container::Style {
        let (text_color, background, border_color) = match self.rarity {
            ItemRarity::Common => (
                Color::from_rgb8(242, 233, 218),
                Color::from_rgb8(54, 51, 48),
                Color::from_rgb8(61, 59, 55),
            ),
            ItemRarity::Uncommon => (
                Color::from_rgb8(172, 240, 149),
                Color::from_rgb8(35, 54, 29),
                Color::from_rgb8(42, 61, 36),
            ),
            ItemRarity::Rare => (
                Color::from_rgb8(149, 202, 240),
                Color::from_rgb8(29, 44, 54),
                Color::from_rgb8(36, 51, 61),
            ),
            ItemRarity::VeryRare => (
                Color::from_rgb8(208, 149, 240),
                Color::from_rgb8(45, 29, 54),
                Color::from_rgb8(52, 36, 61),
            ),
            ItemRarity::Legendary => (
                Color::from_rgb8(240, 213, 149),
                Color::from_rgb8(54, 47, 29),
                Color::from_rgb8(61, 54, 36),
            ),
            ItemRarity::NamedWeapon => (
                Color::from_rgb8(232, 240, 149),
                Color::from_rgb8(51, 54, 29),
                Color::from_rgb8(59, 61, 36),
            ),
            ItemRarity::Unknown => Default::default(),
        };

        container::Style {
            text_color: Some(text_color),
            background: Some(background.into()),
            border_radius: 3.0,
            border_width: 1.0,
            border_color,
        }
    }
}
