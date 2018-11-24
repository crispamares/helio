use std::convert::Into;
use std::f32; 
use std::hash::{Hash, Hasher};
use crate::utils::InDelta;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32
}

impl Color {
    pub fn to_string(&self) -> String {
        format!("rgba({:},{:},{:},{:})", self.r, self.g, self.b, self.a)
    }

    pub fn rgba(color: &Option<Self>) -> String {
        match color { 
            Some(c) => c.to_string(), 
            None => "none".into() 
        }
    }

    pub fn to_hex(&self) -> String {
        let alpha: u8 = (self.a * 255.0).round() as u8;
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, alpha)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r 
        && self.g == other.g
        && self.b == other.b
        && (self.a.in_delta(other.a))
    }
}
impl Eq for Color {}
impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.g.hash(state);
        self.b.hash(state);
        let alpha: u8 = (self.a * 255.0).round() as u8;
        alpha.hash(state);
    }
}

///
/// Exports the CSS4 color list
///
pub const ALICEBLUE: Color = Color{r:240, g:248, b:255, a:1.0};
pub const ANTIQUEWHITE: Color = Color{r:250, g:235, b:215, a:1.0};
pub const AQUA: Color = Color{r:0, g:255, b:255, a:1.0};
pub const AQUAMARINE: Color = Color{r:127, g:255, b:212, a:1.0};
pub const AZURE: Color = Color{r:240, g:255, b:255, a:1.0};
pub const BEIGE: Color = Color{r:245, g:245, b:220, a:1.0};
pub const BISQUE: Color = Color{r:255, g:228, b:196, a:1.0};
pub const BLACK: Color = Color{r:0, g:0, b:0, a:1.0};
pub const BLANCHEDALMOND: Color = Color{r:255, g:235, b:205, a:1.0};
pub const BLUE: Color = Color{r:0, g:0, b:255, a:1.0};
pub const BLUEVIOLET: Color = Color{r:138, g:43, b:226, a:1.0};
pub const BROWN: Color = Color{r:165, g:42, b:42, a:1.0};
pub const BURLYWOOD: Color = Color{r:222, g:184, b:135, a:1.0};
pub const CADETBLUE: Color = Color{r:95, g:158, b:160, a:1.0};
pub const CHARTREUSE: Color = Color{r:127, g:255, b:0, a:1.0};
pub const CHOCOLATE: Color = Color{r:210, g:105, b:30, a:1.0};
pub const CORAL: Color = Color{r:255, g:127, b:80, a:1.0};
pub const CORNFLOWERBLUE: Color = Color{r:100, g:149, b:237, a:1.0};
pub const CORNSILK: Color = Color{r:255, g:248, b:220, a:1.0};
pub const CRIMSON: Color = Color{r:220, g:20, b:60, a:1.0};
pub const CYAN: Color = Color{r:0, g:255, b:255, a:1.0};
pub const DARKBLUE: Color = Color{r:0, g:0, b:139, a:1.0};
pub const DARKCYAN: Color = Color{r:0, g:139, b:139, a:1.0};
pub const DARKGOLDENROD: Color = Color{r:184, g:134, b:11, a:1.0};
pub const DARKGRAY: Color = Color{r:169, g:169, b:169, a:1.0};
pub const DARKGREEN: Color = Color{r:0, g:100, b:0, a:1.0};
pub const DARKGREY: Color = Color{r:169, g:169, b:169, a:1.0};
pub const DARKKHAKI: Color = Color{r:189, g:183, b:107, a:1.0};
pub const DARKMAGENTA: Color = Color{r:139, g:0, b:139, a:1.0};
pub const DARKOLIVEGREEN: Color = Color{r:85, g:107, b:47, a:1.0};
pub const DARKORANGE: Color = Color{r:255, g:140, b:0, a:1.0};
pub const DARKORCHID: Color = Color{r:153, g:50, b:204, a:1.0};
pub const DARKRED: Color = Color{r:139, g:0, b:0, a:1.0};
pub const DARKSALMON: Color = Color{r:233, g:150, b:122, a:1.0};
pub const DARKSEAGREEN: Color = Color{r:143, g:188, b:143, a:1.0};
pub const DARKSLATEBLUE: Color = Color{r:72, g:61, b:139, a:1.0};
pub const DARKSLATEGRAY: Color = Color{r:47, g:79, b:79, a:1.0};
pub const DARKSLATEGREY: Color = Color{r:47, g:79, b:79, a:1.0};
pub const DARKTURQUOISE: Color = Color{r:0, g:206, b:209, a:1.0};
pub const DARKVIOLET: Color = Color{r:148, g:0, b:211, a:1.0};
pub const DEEPPINK: Color = Color{r:255, g:20, b:147, a:1.0};
pub const DEEPSKYBLUE: Color = Color{r:0, g:191, b:255, a:1.0};
pub const DIMGRAY: Color = Color{r:105, g:105, b:105, a:1.0};
pub const DIMGREY: Color = Color{r:105, g:105, b:105, a:1.0};
pub const DODGERBLUE: Color = Color{r:30, g:144, b:255, a:1.0};
pub const FIREBRICK: Color = Color{r:178, g:34, b:34, a:1.0};
pub const FLORALWHITE: Color = Color{r:255, g:250, b:240, a:1.0};
pub const FORESTGREEN: Color = Color{r:34, g:139, b:34, a:1.0};
pub const FUCHSIA: Color = Color{r:255, g:0, b:255, a:1.0};
pub const GAINSBORO: Color = Color{r:220, g:220, b:220, a:1.0};
pub const GHOSTWHITE: Color = Color{r:248, g:248, b:255, a:1.0};
pub const GOLD: Color = Color{r:255, g:215, b:0, a:1.0};
pub const GOLDENROD: Color = Color{r:218, g:165, b:32, a:1.0};
pub const GRAY: Color = Color{r:128, g:128, b:128, a:1.0};
pub const GREEN: Color = Color{r:0, g:128, b:0, a:1.0};
pub const GREENYELLOW: Color = Color{r:173, g:255, b:47, a:1.0};
pub const GREY: Color = Color{r:128, g:128, b:128, a:1.0};
pub const HONEYDEW: Color = Color{r:240, g:255, b:240, a:1.0};
pub const HOTPINK: Color = Color{r:255, g:105, b:180, a:1.0};
pub const INDIANRED: Color = Color{r:205, g:92, b:92, a:1.0};
pub const INDIGO: Color = Color{r:75, g:0, b:130, a:1.0};
pub const IVORY: Color = Color{r:255, g:255, b:240, a:1.0};
pub const KHAKI: Color = Color{r:240, g:230, b:140, a:1.0};
pub const LAVENDER: Color = Color{r:230, g:230, b:250, a:1.0};
pub const LAVENDERBLUSH: Color = Color{r:255, g:240, b:245, a:1.0};
pub const LAWNGREEN: Color = Color{r:124, g:252, b:0, a:1.0};
pub const LEMONCHIFFON: Color = Color{r:255, g:250, b:205, a:1.0};
pub const LIGHTBLUE: Color = Color{r:173, g:216, b:230, a:1.0};
pub const LIGHTCORAL: Color = Color{r:240, g:128, b:128, a:1.0};
pub const LIGHTCYAN: Color = Color{r:224, g:255, b:255, a:1.0};
pub const LIGHTGOLDENRODYELLOW: Color = Color{r:250, g:250, b:210, a:1.0};
pub const LIGHTGRAY: Color = Color{r:211, g:211, b:211, a:1.0};
pub const LIGHTGREEN: Color = Color{r:144, g:238, b:144, a:1.0};
pub const LIGHTGREY: Color = Color{r:211, g:211, b:211, a:1.0};
pub const LIGHTPINK: Color = Color{r:255, g:182, b:193, a:1.0};
pub const LIGHTSALMON: Color = Color{r:255, g:160, b:122, a:1.0};
pub const LIGHTSEAGREEN: Color = Color{r:32, g:178, b:170, a:1.0};
pub const LIGHTSKYBLUE: Color = Color{r:135, g:206, b:250, a:1.0};
pub const LIGHTSLATEGRAY: Color = Color{r:119, g:136, b:153, a:1.0};
pub const LIGHTSLATEGREY: Color = Color{r:119, g:136, b:153, a:1.0};
pub const LIGHTSTEELBLUE: Color = Color{r:176, g:196, b:222, a:1.0};
pub const LIGHTYELLOW: Color = Color{r:255, g:255, b:224, a:1.0};
pub const LIME: Color = Color{r:0, g:255, b:0, a:1.0};
pub const LIMEGREEN: Color = Color{r:50, g:205, b:50, a:1.0};
pub const LINEN: Color = Color{r:250, g:240, b:230, a:1.0};
pub const MAGENTA: Color = Color{r:255, g:0, b:255, a:1.0};
pub const MAROON: Color = Color{r:128, g:0, b:0, a:1.0};
pub const MEDIUMAQUAMARINE: Color = Color{r:102, g:205, b:170, a:1.0};
pub const MEDIUMBLUE: Color = Color{r:0, g:0, b:205, a:1.0};
pub const MEDIUMORCHID: Color = Color{r:186, g:85, b:211, a:1.0};
pub const MEDIUMPURPLE: Color = Color{r:147, g:112, b:219, a:1.0};
pub const MEDIUMSEAGREEN: Color = Color{r:60, g:179, b:113, a:1.0};
pub const MEDIUMSLATEBLUE: Color = Color{r:123, g:104, b:238, a:1.0};
pub const MEDIUMSPRINGGREEN: Color = Color{r:0, g:250, b:154, a:1.0};
pub const MEDIUMTURQUOISE: Color = Color{r:72, g:209, b:204, a:1.0};
pub const MEDIUMVIOLETRED: Color = Color{r:199, g:21, b:133, a:1.0};
pub const MIDNIGHTBLUE: Color = Color{r:25, g:25, b:112, a:1.0};
pub const MINTCREAM: Color = Color{r:245, g:255, b:250, a:1.0};
pub const MISTYROSE: Color = Color{r:255, g:228, b:225, a:1.0};
pub const MOCCASIN: Color = Color{r:255, g:228, b:181, a:1.0};
pub const NAVAJOWHITE: Color = Color{r:255, g:222, b:173, a:1.0};
pub const NAVY: Color = Color{r:0, g:0, b:128, a:1.0};
pub const OLDLACE: Color = Color{r:253, g:245, b:230, a:1.0};
pub const OLIVE: Color = Color{r:128, g:128, b:0, a:1.0};
pub const OLIVEDRAB: Color = Color{r:107, g:142, b:35, a:1.0};
pub const ORANGE: Color = Color{r:255, g:165, b:0, a:1.0};
pub const ORANGERED: Color = Color{r:255, g:69, b:0, a:1.0};
pub const ORCHID: Color = Color{r:218, g:112, b:214, a:1.0};
pub const PALEGOLDENROD: Color = Color{r:238, g:232, b:170, a:1.0};
pub const PALEGREEN: Color = Color{r:152, g:251, b:152, a:1.0};
pub const PALETURQUOISE: Color = Color{r:175, g:238, b:238, a:1.0};
pub const PALEVIOLETRED: Color = Color{r:219, g:112, b:147, a:1.0};
pub const PAPAYAWHIP: Color = Color{r:255, g:239, b:213, a:1.0};
pub const PEACHPUFF: Color = Color{r:255, g:218, b:185, a:1.0};
pub const PERU: Color = Color{r:205, g:133, b:63, a:1.0};
pub const PINK: Color = Color{r:255, g:192, b:203, a:1.0};
pub const PLUM: Color = Color{r:221, g:160, b:221, a:1.0};
pub const POWDERBLUE: Color = Color{r:176, g:224, b:230, a:1.0};
pub const PURPLE: Color = Color{r:128, g:0, b:128, a:1.0};
pub const REBECCAPURPLE: Color = Color{r:102, g:51, b:153, a:1.0};
pub const RED: Color = Color{r:255, g:0, b:0, a:1.0};
pub const ROSYBROWN: Color = Color{r:188, g:143, b:143, a:1.0};
pub const ROYALBLUE: Color = Color{r:65, g:105, b:225, a:1.0};
pub const SADDLEBROWN: Color = Color{r:139, g:69, b:19, a:1.0};
pub const SALMON: Color = Color{r:250, g:128, b:114, a:1.0};
pub const SANDYBROWN: Color = Color{r:244, g:164, b:96, a:1.0};
pub const SEAGREEN: Color = Color{r:46, g:139, b:87, a:1.0};
pub const SEASHELL: Color = Color{r:255, g:245, b:238, a:1.0};
pub const SIENNA: Color = Color{r:160, g:82, b:45, a:1.0};
pub const SILVER: Color = Color{r:192, g:192, b:192, a:1.0};
pub const SKYBLUE: Color = Color{r:135, g:206, b:235, a:1.0};
pub const SLATEBLUE: Color = Color{r:106, g:90, b:205, a:1.0};
pub const SLATEGRAY: Color = Color{r:112, g:128, b:144, a:1.0};
pub const SLATEGREY: Color = Color{r:112, g:128, b:144, a:1.0};
pub const SNOW: Color = Color{r:255, g:250, b:250, a:1.0};
pub const SPRINGGREEN: Color = Color{r:0, g:255, b:127, a:1.0};
pub const STEELBLUE: Color = Color{r:70, g:130, b:180, a:1.0};
pub const TAN: Color = Color{r:210, g:180, b:140, a:1.0};
pub const TEAL: Color = Color{r:0, g:128, b:128, a:1.0};
pub const THISTLE: Color = Color{r:216, g:191, b:216, a:1.0};
pub const TOMATO: Color = Color{r:255, g:99, b:71, a:1.0};
pub const TURQUOISE: Color = Color{r:64, g:224, b:208, a:1.0};
pub const VIOLET: Color = Color{r:238, g:130, b:238, a:1.0};
pub const WHEAT: Color = Color{r:245, g:222, b:179, a:1.0};
pub const WHITE: Color = Color{r:255, g:255, b:255, a:1.0};
pub const WHITESMOKE: Color = Color{r:245, g:245, b:245, a:1.0};
pub const YELLOW: Color = Color{r:255, g:255, b:0, a:1.0};
pub const YELLOWGREEN: Color = Color{r:154, g:205, b:50, a:1.0}; 



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hexadecimal_works() {
        assert_eq!(BLUE.to_hex(), "#0000ffff");
        assert_eq!(WHITE.to_hex(), "#ffffffff");
        assert_eq!(BLACK.to_hex(), "#000000ff");
        let trans = Color{r: 255, g: 100, b: 23, a: 0.5};
        assert_eq!(trans.to_hex(), "#ff641780");
    }
}