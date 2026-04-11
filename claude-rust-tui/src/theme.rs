use std::sync::OnceLock;

use ratatui::style::Color;

pub const BASE: Color    = Color::Rgb(25,  23,  36);
pub const SURFACE: Color = Color::Rgb(31,  29,  46);
pub const OVERLAY: Color = Color::Rgb(38,  35,  58);
pub const MUTED: Color   = Color::Rgb(110, 106, 134);
pub const SUBTLE: Color  = Color::Rgb(144, 140, 170);
pub const TEXT: Color    = Color::Rgb(224, 222, 244);
pub const LOVE: Color    = Color::Rgb(235, 111, 146);
pub const GOLD: Color    = Color::Rgb(246, 193, 119);
pub const ROSE: Color    = Color::Rgb(235, 188, 186);
pub const PINE: Color    = Color::Rgb(49,  116, 143);
pub const FOAM: Color    = Color::Rgb(156, 207, 216);
pub const IRIS: Color    = Color::Rgb(196, 167, 231);
pub const HL_MED: Color  = Color::Rgb(64,  61,  82);
pub const HL_HIGH: Color = Color::Rgb(82,  79, 103);

static TRUECOLOR: OnceLock<bool> = OnceLock::new();

pub fn supports_truecolor() -> bool {
    *TRUECOLOR.get_or_init(|| {
        let ct = std::env::var("COLORTERM").unwrap_or_default();
        if ct.contains("truecolor") || ct.contains("24bit") { return true; }
        let term = std::env::var("TERM").unwrap_or_default();
        term.contains("truecolor") || term.contains("direct")
    })
}

/// Dynamic color functions — return 256-color indexed when truecolor unavailable.
/// Use these in all widgets instead of the raw constants above.

pub fn base()    -> Color { if supports_truecolor() { BASE }    else { Color::Indexed(16) } }
pub fn surface() -> Color { if supports_truecolor() { SURFACE } else { Color::Indexed(17) } }
pub fn overlay() -> Color { if supports_truecolor() { OVERLAY } else { Color::Indexed(59) } }
pub fn muted()   -> Color { if supports_truecolor() { MUTED }   else { Color::Indexed(103) } }
pub fn subtle()  -> Color { if supports_truecolor() { SUBTLE }  else { Color::Indexed(139) } }
pub fn text()    -> Color { if supports_truecolor() { TEXT }    else { Color::Indexed(188) } }
pub fn love()    -> Color { if supports_truecolor() { LOVE }    else { Color::Indexed(174) } }
pub fn gold()    -> Color { if supports_truecolor() { GOLD }    else { Color::Indexed(215) } }
pub fn rose()    -> Color { if supports_truecolor() { ROSE }    else { Color::Indexed(217) } }
pub fn pine()    -> Color { if supports_truecolor() { PINE }    else { Color::Indexed(31) } }
pub fn foam()    -> Color { if supports_truecolor() { FOAM }    else { Color::Indexed(152) } }
pub fn iris()    -> Color { if supports_truecolor() { IRIS }    else { Color::Indexed(182) } }
pub fn hl_med()  -> Color { if supports_truecolor() { HL_MED }  else { Color::Indexed(59) } }
pub fn hl_high() -> Color { if supports_truecolor() { HL_HIGH } else { Color::Indexed(67) } }
