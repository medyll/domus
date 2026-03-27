//! Primitive UI components - the building blocks of Domius UI.

pub mod button;
pub mod input;
pub mod text;
pub mod icon;
pub mod card;
pub mod grid;
pub mod divider;
pub mod space;
pub mod tag;
pub mod typography;
pub mod affix;
pub mod backtop;
pub mod checkcard;
pub mod countdown;
pub mod qrcode;
pub mod scrolltext;

pub use button::{Button, ButtonProps, ButtonVariant, ButtonSize};
pub use input::{Input, InputProps, InputType};
pub use text::{Text, TextProps, TextVariant};
pub use icon::{Icon, IconProps, IconName};
pub use card::{card, CardProps};
pub use grid::{row, col, RowProps, ColProps};
pub use divider::{divider, DividerProps, DividerOrientation};
pub use space::{space, SpaceProps, SpaceDirection, SpaceAlign, SpaceJustify};
pub use tag::{tag, TagProps, TagColor};
pub use typography::{title, paragraph, link, TitleProps, ParagraphProps, LinkProps, TitleLevel};
pub use affix::{affix, AffixProps};
pub use backtop::{backtop, BackTopProps};
pub use checkcard::{checkcard, CheckCardProps};
pub use countdown::{countdown, CountdownProps, CountdownFormat};
pub use qrcode::{qrcode, QRCodeProps, QRCodeErrorLevel};
pub use scrolltext::{scrolltext, ScrollTextProps, ScrollTextDirection};
