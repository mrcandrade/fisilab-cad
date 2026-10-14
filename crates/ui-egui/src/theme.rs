//! Design tokens. Colours were measured from the reference look (dark slate chrome, near-black
//! navy model space) and are CADCraft's own values; every widget reads them from here.
//!
//! Four palettes: [`Tokens::NOITE`] ("UFRGS Noite", the Suíte FisiLab's dark look and the default)
//! and [`Tokens::DIA`] ("UFRGS Dia", its light look), both built from `fisilab_brand::palette`, plus
//! the upstream [`Tokens::DARK`] (CADCraft's original look) and [`Tokens::LIGHT`]. The interface
//! theme changes the chrome (title and tool bar, menus, palettes, tabs, command line, status bar,
//! dialogs); the drawing area keeps its model-space colours in every theme, as drawing colours are
//! not part of the interface theme. [`Tokens::get`] returns the palette of the theme in use, which
//! [`CadApp::logic`](crate::CadApp::logic) resolves once per frame from the user's [`ThemePref`].

use std::cell::Cell;

use egui::{Color32, FontFamily, FontId, Theme, Visuals};
use serde::{Deserialize, Serialize};

/// The interface theme the user chose.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemePref {
    /// The default: "UFRGS Noite", the Suíte FisiLab's dark palette.
    #[default]
    #[serde(rename = "ufrgsNoite")]
    UfrgsNoite,
    /// "UFRGS Dia", the Suíte FisiLab's light palette.
    #[serde(rename = "ufrgsDia")]
    UfrgsDia,
    /// Follow the operating system's light/dark appearance, live (with the upstream palettes).
    System,
    Light,
    /// CADCraft's original look.
    Dark,
}

/// The theme `System` uses when the OS (or browser) reports no appearance.
pub const SYSTEM_FALLBACK: Theme = Theme::Dark;

impl ThemePref {
    pub const ALL: [ThemePref; 5] = [ThemePref::UfrgsNoite, ThemePref::UfrgsDia, ThemePref::System, ThemePref::Light, ThemePref::Dark];

    pub fn as_str(self) -> &'static str {
        match self {
            ThemePref::UfrgsNoite => "ufrgsNoite",
            ThemePref::UfrgsDia => "ufrgsDia",
            ThemePref::System => "system",
            ThemePref::Light => "light",
            ThemePref::Dark => "dark",
        }
    }

    pub fn parse(s: &str) -> Option<ThemePref> {
        ThemePref::ALL.into_iter().find(|t| t.as_str().eq_ignore_ascii_case(s.trim()))
    }

    /// The light/dark appearance to show, given what the OS reports (`ctx.system_theme()`).
    pub fn resolve(self, system: Option<Theme>) -> Theme {
        self.palette(system).theme()
    }

    /// The palette to show, given what the OS reports (`ctx.system_theme()`).
    pub fn palette(self, system: Option<Theme>) -> Palette {
        match self {
            ThemePref::UfrgsNoite => Palette::UfrgsNoite,
            ThemePref::UfrgsDia => Palette::UfrgsDia,
            ThemePref::System => Palette::from(system.unwrap_or(SYSTEM_FALLBACK)),
            ThemePref::Light => Palette::Light,
            ThemePref::Dark => Palette::Dark,
        }
    }
}

/// A concrete colour set: what a [`ThemePref`] resolves to on a given frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Palette {
    UfrgsNoite,
    UfrgsDia,
    Dark,
    Light,
}

impl Palette {
    pub const ALL: [Palette; 4] = [Palette::UfrgsNoite, Palette::UfrgsDia, Palette::Dark, Palette::Light];

    /// egui's light/dark appearance for this palette.
    pub fn theme(self) -> Theme {
        match self {
            Palette::UfrgsNoite | Palette::Dark => Theme::Dark,
            Palette::UfrgsDia | Palette::Light => Theme::Light,
        }
    }

    pub fn tokens(self) -> Tokens {
        match self {
            Palette::UfrgsNoite => Tokens::NOITE,
            Palette::UfrgsDia => Tokens::DIA,
            Palette::Dark => Tokens::DARK,
            Palette::Light => Tokens::LIGHT,
        }
    }

    /// The Suíte FisiLab tones behind a UFRGS palette; `None` for the upstream ones.
    pub fn brand(self) -> Option<&'static fisilab_brand::palette::Tons> {
        match self {
            Palette::UfrgsNoite => Some(&fisilab_brand::palette::NOITE),
            Palette::UfrgsDia => Some(&fisilab_brand::palette::DIA),
            Palette::Dark | Palette::Light => None,
        }
    }

    /// The (dark, light) pair this palette belongs to; both are installed into egui together.
    fn family(self) -> (Palette, Palette) {
        match self {
            Palette::UfrgsNoite | Palette::UfrgsDia => (Palette::UfrgsNoite, Palette::UfrgsDia),
            Palette::Dark | Palette::Light => (Palette::Dark, Palette::Light),
        }
    }
}

/// The upstream palette of an egui appearance.
impl From<Theme> for Palette {
    fn from(theme: Theme) -> Palette {
        match theme {
            Theme::Dark => Palette::Dark,
            Theme::Light => Palette::Light,
        }
    }
}

thread_local! {
    /// The palette in use on the UI thread (set by [`set_active`] each frame).
    static ACTIVE: Cell<Palette> = const { Cell::new(Palette::UfrgsNoite) };
}

/// The light/dark appearance of the palette in use.
pub fn active() -> Theme {
    ACTIVE.with(Cell::get).theme()
}

/// The palette [`Tokens::get`] returns colours for.
pub fn active_palette() -> Palette {
    ACTIVE.with(Cell::get)
}

pub fn set_active(palette: impl Into<Palette>) {
    let palette = palette.into();
    ACTIVE.with(|a| a.set(palette));
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub chrome: Color32,
    pub chrome_dark: Color32,
    pub panel: Color32,
    pub tab_active: Color32,
    pub control: Color32,
    pub control_hover: Color32,
    pub border: Color32,
    pub canvas: Color32,
    pub grid_minor: Color32,
    pub grid_major: Color32,
    pub axis_x: Color32,
    pub axis_y: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub icon: Color32,
    pub icon_accent: Color32,
    pub icon_point: Color32,
    pub accent: Color32,
    pub selection: Color32,
    pub hover: Color32,
    pub grip: Color32,
    pub grip_hot: Color32,
    /// Grips of dynamic block references (AutoCAD's GRIPDYNCOLOR).
    pub grip_dyn: Color32,
    pub snap: Color32,
    /// COMPARE: objects only in this drawing, only in the other one, and the areas of change.
    pub compare_this: Color32,
    pub compare_other: Color32,
    pub compare_area: Color32,
    pub window_stroke: Color32,
    pub crossing_stroke: Color32,
    pub cmd_bg: Color32,
    pub toggle_on: Color32,
    /// Text fields and other inset (extreme) backgrounds.
    pub field: Color32,
    /// Selected text and selected list rows.
    pub select_bg: Color32,
    /// Lines and labels drawn straight onto the drawing area (UCS icon, viewport label, rubber
    /// bands): they follow the model-space background, not the interface theme.
    pub canvas_ink: Color32,
    /// The command-line history lines behind the input bar.
    pub cmd_history: Color32,
    pub cmd_border: Color32,
    /// Command-line keywords (`[Undo/Close]`) and their hover background.
    pub cmd_keyword: Color32,
    pub cmd_keyword_hover: Color32,
    /// The command-line AutoComplete list: background and the highlighted first row.
    pub list_bg: Color32,
    pub list_row: Color32,
    /// Transient warnings (status bar messages).
    pub warn: Color32,
    /// The drawing-area crosshair cursor and its pick box.
    pub crosshair: Color32,
    /// The sheet of paper on a layout tab.
    pub paper: Color32,
    /// Text on an [`Tokens::accent`] fill (primary buttons).
    pub accent_text: Color32,
}

impl Tokens {
    pub const DARK: Tokens = Tokens {
        chrome: Color32::from_rgb(0x3b, 0x44, 0x53),
        chrome_dark: Color32::from_rgb(0x31, 0x38, 0x44),
        panel: Color32::from_rgb(0x3c, 0x44, 0x52),
        tab_active: Color32::from_rgb(0x4e, 0x5a, 0x6e),
        control: Color32::from_rgb(0x50, 0x5a, 0x6d),
        control_hover: Color32::from_rgb(0x5c, 0x67, 0x7c),
        border: Color32::from_rgb(0x23, 0x29, 0x30),
        canvas: Color32::from_rgb(0x22, 0x28, 0x2f),
        grid_minor: Color32::from_rgb(0x29, 0x30, 0x3a),
        grid_major: Color32::from_rgb(0x31, 0x39, 0x45),
        axis_x: Color32::from_rgb(0x56, 0x28, 0x29),
        axis_y: Color32::from_rgb(0x28, 0x4a, 0x2e),
        text: Color32::from_rgb(0xd7, 0xdb, 0xe0),
        text_dim: Color32::from_rgb(0xa3, 0xaa, 0xb4),
        text_faint: Color32::from_rgb(0x87, 0x8d, 0x96),
        icon: Color32::from_rgb(0xcf, 0xd4, 0xdb),
        icon_accent: Color32::from_rgb(0x4f, 0xa3, 0xf7),
        icon_point: Color32::from_rgb(0xf0, 0x7a, 0x3a),
        accent: Color32::from_rgb(0x3d, 0x8b, 0xfd),
        selection: Color32::from_rgb(0x4a, 0x8f, 0xff),
        hover: Color32::from_rgb(0x9c, 0xc6, 0xff),
        grip: Color32::from_rgb(0x2f, 0x6b, 0xff),
        grip_hot: Color32::from_rgb(0xff, 0x3b, 0x3b),
        grip_dyn: Color32::from_rgb(0x26, 0xc6, 0xe8),
        snap: Color32::from_rgb(0x2f, 0xd0, 0x62),
        compare_this: Color32::from_rgb(0x4c, 0xd9, 0x7b),
        compare_other: Color32::from_rgb(0xff, 0x5c, 0x5c),
        compare_area: Color32::from_rgb(0xff, 0xb4, 0x3c),
        window_stroke: Color32::from_rgb(0x6f, 0xa8, 0xff),
        crossing_stroke: Color32::from_rgb(0x7f, 0xe0, 0x7f),
        cmd_bg: Color32::from_rgba_premultiplied(0x34, 0x3c, 0x4a, 0xf0),
        toggle_on: Color32::from_rgb(0x3d, 0x8b, 0xfd),
        field: Color32::from_rgb(0x31, 0x38, 0x44),
        select_bg: Color32::from_rgb(0x3d, 0x8b, 0xfd),
        canvas_ink: Color32::from_rgb(0xa3, 0xaa, 0xb4),
        cmd_history: Color32::from_rgba_unmultiplied_const(0x2a, 0x30, 0x3a, 170),
        cmd_border: Color32::from_rgb(0x55, 0x5f, 0x70),
        cmd_keyword: Color32::from_rgb(0x8f, 0xc1, 0xff),
        cmd_keyword_hover: Color32::from_rgb(0x2f, 0x5e, 0xa8),
        list_bg: Color32::from_rgb(0x2b, 0x31, 0x3b),
        list_row: Color32::from_rgb(0x3a, 0x42, 0x50),
        warn: Color32::from_rgb(0xff, 0xd0, 0x80),
        crosshair: Color32::from_rgb(0xe8, 0xe8, 0xe8),
        paper: Color32::WHITE,
        accent_text: Color32::from_rgb(0xd7, 0xdb, 0xe0),
    };

    /// Light grey chrome with dark text. The drawing-area colours (canvas, grid, axes, selection,
    /// grips, snap markers, selection windows) are the same as [`Tokens::DARK`]'s.
    pub const LIGHT: Tokens = Tokens {
        chrome: Color32::from_rgb(0xe4, 0xe7, 0xeb),
        chrome_dark: Color32::from_rgb(0xd5, 0xd9, 0xdf),
        panel: Color32::from_rgb(0xee, 0xf0, 0xf3),
        tab_active: Color32::from_rgb(0xc8, 0xd3, 0xe2),
        control: Color32::from_rgb(0xdc, 0xe0, 0xe6),
        control_hover: Color32::from_rgb(0xcb, 0xd2, 0xdc),
        border: Color32::from_rgb(0xb3, 0xba, 0xc4),
        text: Color32::from_rgb(0x1d, 0x23, 0x2b),
        text_dim: Color32::from_rgb(0x47, 0x50, 0x5c),
        text_faint: Color32::from_rgb(0x62, 0x6a, 0x76),
        icon: Color32::from_rgb(0x34, 0x3c, 0x48),
        icon_accent: Color32::from_rgb(0x1b, 0x6f, 0xd6),
        icon_point: Color32::from_rgb(0xd3, 0x5a, 0x1c),
        accent: Color32::from_rgb(0x2f, 0x7c, 0xf6),
        cmd_bg: Color32::from_rgba_unmultiplied_const(0xf9, 0xfa, 0xfb, 0xf0),
        toggle_on: Color32::from_rgb(0x2f, 0x7c, 0xf6),
        field: Color32::from_rgb(0xff, 0xff, 0xff),
        select_bg: Color32::from_rgb(0xb6, 0xd3, 0xfb),
        cmd_history: Color32::from_rgba_unmultiplied_const(0xf0, 0xf2, 0xf5, 0xdc),
        cmd_border: Color32::from_rgb(0xa9, 0xb0, 0xba),
        cmd_keyword: Color32::from_rgb(0x15, 0x59, 0xc0),
        cmd_keyword_hover: Color32::from_rgb(0xcf, 0xe0, 0xfa),
        list_bg: Color32::from_rgb(0xfb, 0xfc, 0xfd),
        list_row: Color32::from_rgb(0xe3, 0xe8, 0xef),
        warn: Color32::from_rgb(0x8a, 0x53, 0x00),
        accent_text: Color32::from_rgb(0x1d, 0x23, 0x2b),
        ..Tokens::DARK
    };

    /// "UFRGS Noite": the Suíte FisiLab's dark palette (the default), from
    /// `fisilab_brand::palette::NOITE`. The drawing area keeps [`Tokens::DARK`]'s CAD colours
    /// (selection, grips, snaps, crosshair, paper) on the brand's darkest tone.
    pub const NOITE: Tokens = Tokens::ufrgs(&fisilab_brand::palette::NOITE);

    /// "UFRGS Dia": the Suíte FisiLab's light palette, from `fisilab_brand::palette::DIA`. Like
    /// [`Tokens::LIGHT`], only the chrome turns light: the drawing area is [`Tokens::NOITE`]'s.
    pub const DIA: Tokens = Tokens::ufrgs(&fisilab_brand::palette::DIA);

    /// The chrome of a UFRGS palette from the brand tones `b`. The drawing area is the same for
    /// both: the model space on the dark theme's background, with grid and axes derived from it
    /// as [`Tokens::with_model_background`] does.
    const fn ufrgs(b: &fisilab_brand::palette::Tons) -> Tokens {
        let canvas = fisilab_brand::palette::NOITE.fundo;
        Tokens {
            chrome: b.painel,
            chrome_dark: b.fundo,
            panel: b.painel,
            tab_active: b.ativo,
            control: b.elevado,
            control_hover: b.hover,
            border: b.borda,
            canvas,
            grid_minor: mix_const(canvas, Color32::WHITE, 50),
            grid_major: mix_const(canvas, Color32::WHITE, 110),
            axis_x: mix_const(canvas, Color32::from_rgb(0xd0, 0x30, 0x30), 300),
            axis_y: mix_const(canvas, Color32::from_rgb(0x30, 0xb0, 0x40), 300),
            text: b.texto,
            text_dim: b.texto_fraco,
            text_faint: mix_const(b.texto_fraco, b.painel, 200),
            icon: b.texto,
            icon_accent: b.foco,
            icon_point: b.destaque,
            accent: b.destaque,
            cmd_bg: with_alpha(b.painel, 0xf0),
            toggle_on: b.destaque,
            field: b.elevado,
            select_bg: b.selecao,
            cmd_history: with_alpha(b.fundo, if b.escuro { 170 } else { 0xdc }),
            cmd_border: b.borda,
            cmd_keyword: b.foco,
            cmd_keyword_hover: with_alpha(b.foco, 0x55),
            list_bg: b.elevado,
            list_row: b.hover,
            warn: b.aviso,
            accent_text: b.texto_destaque,
            ..Tokens::DARK
        }
    }

    /// The upstream palette of `theme`.
    pub fn of(theme: Theme) -> Tokens {
        Palette::from(theme).tokens()
    }

    /// The palette in use, with the user's drawing-area colours ([`set_canvas`]).
    pub fn get() -> Tokens {
        CANVAS.with(Cell::get).over(active_palette().tokens())
    }

    /// These tokens with `bg` as the model-space background: the grid, axes, crosshair and the
    /// lines drawn on the drawing area are recomputed to stay visible on it.
    pub fn with_model_background(self, bg: Color32) -> Tokens {
        let light = luminance(bg) > 0.5;
        let ink = if light { Color32::BLACK } else { Color32::WHITE };
        Tokens {
            canvas: bg,
            grid_minor: mix(bg, ink, 0.05),
            grid_major: mix(bg, ink, 0.11),
            axis_x: mix(bg, Color32::from_rgb(0xd0, 0x30, 0x30), 0.3),
            axis_y: mix(bg, Color32::from_rgb(0x30, 0xb0, 0x40), 0.3),
            canvas_ink: if light { Color32::from_rgb(0x47, 0x50, 0x5c) } else { Tokens::DARK.canvas_ink },
            crosshair: if light { Color32::from_rgb(0x20, 0x20, 0x20) } else { Tokens::DARK.crosshair },
            ..self
        }
    }
}

/// The user's drawing-area preferences (Options ▸ Display and Drafting); `None` keeps the theme's
/// colour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Canvas {
    pub model: Option<Color32>,
    pub paper: Option<Color32>,
    /// The side of an object snap marker, in points.
    pub snap_marker: f32,
    /// The colour selected objects are highlighted in.
    pub selection: Option<Color32>,
}

/// The default object snap marker size.
pub const SNAP_MARKER: f32 = 11.0;

impl Default for Canvas {
    fn default() -> Self {
        Canvas { model: None, paper: None, snap_marker: SNAP_MARKER, selection: None }
    }
}

impl Canvas {
    fn over(self, t: Tokens) -> Tokens {
        let t = match self.model {
            Some(bg) => t.with_model_background(bg),
            None => t,
        };
        Tokens { paper: self.paper.unwrap_or(t.paper), selection: self.selection.unwrap_or(t.selection), ..t }
    }
}

thread_local! {
    /// The drawing-area preferences in use on the UI thread (set by [`set_canvas`] each frame).
    static CANVAS: Cell<Canvas> = Cell::new(Canvas::default());
}

pub fn set_canvas(c: Canvas) {
    CANVAS.with(|v| v.set(c));
}

/// Half the side of an object snap marker, in points.
pub fn snap_marker_half() -> f32 {
    CANVAS.with(Cell::get).snap_marker / 2.0
}

/// Relative luminance of `c` (0 black … 1 white).
pub fn luminance(c: Color32) -> f32 {
    (0.299 * f32::from(c.r()) + 0.587 * f32::from(c.g()) + 0.114 * f32::from(c.b())) / 255.0
}

/// `a` moved `permille` thousandths of the way towards `b`, in const context (the palettes).
const fn mix_const(a: Color32, b: Color32, permille: u32) -> Color32 {
    const fn m(x: u8, y: u8, f: u32) -> u8 {
        ((x as u32 * (1000 - f) + y as u32 * f + 500) / 1000) as u8
    }
    Color32::from_rgb(m(a.r(), b.r(), permille), m(a.g(), b.g(), permille), m(a.b(), b.b(), permille))
}

/// `c` with the (unmultiplied) alpha `a`.
const fn with_alpha(c: Color32, a: u8) -> Color32 {
    Color32::from_rgba_unmultiplied_const(c.r(), c.g(), c.b(), a)
}

/// `a` moved the fraction `f` towards `b`.
fn mix(a: Color32, b: Color32, f: f32) -> Color32 {
    let m = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * f).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

pub fn small() -> FontId {
    FontId::new(11.5, FontFamily::Proportional)
}
pub fn body() -> FontId {
    FontId::new(12.5, FontFamily::Proportional)
}
pub fn mono() -> FontId {
    FontId::new(12.0, FontFamily::Monospace)
}

/// Load a system UI font at run time when one is installed (not bundled), else keep egui's;
/// then CJK fonts after the Latin ones, so Latin text keeps its look, in every interface
/// language: file, layout and layer names can be Japanese or Chinese whatever the interface
/// shows (#513). In a Chinese interface (`language` "zh…") the Chinese font comes first, else a
/// Japanese one, so names take their own glyph forms; in an Arabic interface an installed
/// Arabic font follows. A font embedded from craft-fonts comes
/// last, for systems without any (and the web build, which has no system fonts). A Korean
/// interface adds a Hangul font only when the CJK font found doesn't cover Hangul
/// ([`korean_font`]).
pub fn install_fonts(ctx: &egui::Context, language: &str) {
    let mut fonts = egui::FontDefinitions::default();
    let mut added: Vec<(String, std::sync::Arc<egui::FontData>)> = Vec::new();
    #[cfg(not(target_arch = "wasm32"))]
    let chinese = language.starts_with("zh");
    #[cfg(not(target_arch = "wasm32"))]
    {
        let candidates: &[&str] = if cfg!(target_os = "macos") {
            &["/System/Library/Fonts/SFNS.ttf", "/System/Library/Fonts/Helvetica.ttc", "/Library/Fonts/Arial.ttf"]
        } else if cfg!(windows) {
            &["C:\\Windows\\Fonts\\segoeui.ttf", "C:\\Windows\\Fonts\\arial.ttf"]
        } else {
            &[
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                "/usr/share/fonts/TTF/DejaVuSans.ttf",
                "/usr/share/fonts/noto/NotoSans-Regular.ttf",
                "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
            ]
        };
        for path in candidates {
            if let Ok(bytes) = std::fs::read(path) {
                fonts.font_data.insert("system-ui".into(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
                if let Some(f) = fonts.families.get_mut(&FontFamily::Proportional) {
                    f.insert(0, "system-ui".into());
                }
                break;
            }
        }
        let mut paths: Vec<&std::path::Path> = Vec::new();
        if !chinese {
            paths.extend(japanese_font());
        }
        paths.extend(cjk_font());
        paths.dedup();
        added.extend(
            paths
                .into_iter()
                .filter_map(|p| read_capped(p, MAX_UI_FONT_BYTES))
                .enumerate()
                .map(|(i, bytes)| (format!("system-cjk-{i}"), std::sync::Arc::new(egui::FontData::from_owned(bytes)))),
        );
        // العربية: an installed Arabic face after the Latin ones (egui's bundled faces carry no
        // Arabic glyphs), for the Arabic interface.
        if language == "ar"
            && let Some(bytes) = arabic_font().and_then(|p| read_capped(p, MAX_UI_FONT_BYTES))
        {
            added.push(("system-arabic".into(), std::sync::Arc::new(egui::FontData::from_owned(bytes))));
        }
    }
    let mut embedded: Vec<_> = cadcraft_fonts::CRAFT_FONTS
        .iter()
        .filter(|f| f.style == "Regular" && f.scripts.iter().any(|s| matches!(*s, "Jpan" | "Hans" | "Hant")))
        .collect();
    // A sans-serif face reads best in the interface.
    embedded.sort_by_key(|f| !f.family.contains("Gothic") && !f.family.contains("Sans"));
    added.extend(embedded.first().map(|f| (format!("craft-{}", f.stem()), std::sync::Arc::new(egui::FontData::from_static(f.bytes)))));
    // Korean: Noto Sans CJK and most CJK fonts above already cover Hangul; else a Hangul font.
    if language == "ko"
        && !cjk_covers_hangul()
        && let Some(face) = korean_font()
    {
        let mut data = egui::FontData::from_owned(face.bytes.as_ref().clone());
        data.index = face.index;
        added.push(("korean-ui".into(), std::sync::Arc::new(data)));
    }
    for (name, data) in added {
        fonts.font_data.insert(name.clone(), data);
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            if let Some(f) = fonts.families.get_mut(&family) {
                f.push(name.clone());
            }
        }
    }
    ctx.set_fonts(fonts);
}

/// Largest interface font file read or embedded face copied (CJK collections are 15–60 MB).
const MAX_UI_FONT_BYTES: u64 = 96 * 1024 * 1024;

#[cfg(not(target_arch = "wasm32"))]
fn read_capped(path: &std::path::Path, max: u64) -> Option<Vec<u8>> {
    use std::io::Read as _;
    let file = std::fs::File::open(path).ok()?;
    let len = file.metadata().ok()?.len();
    if len == 0 || len > max {
        return None;
    }
    // Enforce the cap during the read too, even if the file grows after metadata was read.
    let mut bytes = Vec::new();
    file.take(max.saturating_add(1)).read_to_end(&mut bytes).ok()?;
    (!bytes.is_empty() && bytes.len() as u64 <= max).then_some(bytes)
}

/// An installed font with Chinese characters, for the Chinese interface; fonts are never bundled
/// (only looked for where the OS or common packages put them). Checked once.
pub fn cjk_font() -> Option<&'static std::path::Path> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static FOUND: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
        FOUND
            .get_or_init(|| {
                let candidates: &[&str] = if cfg!(target_os = "macos") {
                    &["/System/Library/Fonts/PingFang.ttc", "/System/Library/Fonts/STHeiti Light.ttc", "/System/Library/Fonts/Hiragino Sans GB.ttc"]
                } else if cfg!(windows) {
                    &["C:\\Windows\\Fonts\\msyh.ttc", "C:\\Windows\\Fonts\\msyh.ttf", "C:\\Windows\\Fonts\\simsun.ttc"]
                } else {
                    &[
                        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/opentype/noto/NotoSansCJKsc-Regular.otf",
                        "/usr/local/share/fonts/noto/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
                        "/usr/share/fonts/wqy-microhei/wqy-microhei.ttc",
                        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
                        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
                    ]
                };
                candidates.iter().map(std::path::PathBuf::from).find(|p| p.is_file())
            })
            .as_deref()
    }
    #[cfg(target_arch = "wasm32")]
    None
}

/// An installed font with Arabic glyphs, for the العربية interface; fonts are never bundled
/// (only looked for where the OS or common packages put them). Checked once.
pub fn arabic_font() -> Option<&'static std::path::Path> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static FOUND: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
        FOUND
            .get_or_init(|| {
                let candidates: &[&str] = if cfg!(target_os = "macos") {
                    &[
                        "/System/Library/Fonts/GeezaPro.ttc",
                        "/System/Library/Fonts/Supplemental/Geeza Pro.ttf",
                        "/System/Library/Fonts/Supplemental/Al Nile.ttc",
                        "/System/Library/Fonts/Supplemental/Baghdad.ttf",
                    ]
                } else if cfg!(windows) {
                    &[
                        "C:\\Windows\\Fonts\\segoeui.ttf",
                        "C:\\Windows\\Fonts\\tahoma.ttf",
                        "C:\\Windows\\Fonts\\trado.ttf",
                        "C:\\Windows\\Fonts\\arabtype.ttf",
                        "C:\\Windows\\Fonts\\arial.ttf",
                    ]
                } else {
                    &[
                        "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
                        "/usr/share/fonts/noto/NotoSansArabic-Regular.ttf",
                        "/usr/share/fonts/opentype/noto/NotoSansArabic-Regular.ttf",
                        "/usr/share/fonts/truetype/noto/NotoNaskhArabic-Regular.ttf",
                        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                        "/usr/share/fonts/TTF/DejaVuSans.ttf",
                        "/usr/share/fonts/truetype/amiri/Amiri-Regular.ttf",
                        "/usr/share/fonts/truetype/kacst/KacstOne.ttf",
                    ]
                };
                candidates.iter().map(std::path::PathBuf::from).find(|p| p.is_file())
            })
            .as_deref()
    }
    #[cfg(target_arch = "wasm32")]
    None
}

/// An installed Japanese font, for Japanese names in a non-Chinese interface. Checked once.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn japanese_font() -> Option<&'static std::path::Path> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static FOUND: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
        FOUND
            .get_or_init(|| {
                let candidates: &[&str] = if cfg!(target_os = "macos") {
                    &["/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc", "/System/Library/Fonts/Hiragino Sans GB.ttc"]
                } else if cfg!(windows) {
                    &["C:\\Windows\\Fonts\\YuGothR.ttc", "C:\\Windows\\Fonts\\meiryo.ttc", "C:\\Windows\\Fonts\\msgothic.ttc"]
                } else {
                    &[
                        "/usr/share/fonts/opentype/ipafont-gothic/ipagp.ttf",
                        "/usr/share/fonts/truetype/fonts-japanese-gothic.ttf",
                        "/usr/share/fonts/ipa-pgothic/ipagp.ttf",
                        "/usr/share/fonts/vl-gothic/VL-PGothic-Regular.ttf",
                    ]
                };
                candidates.iter().map(std::path::PathBuf::from).find(|p| p.is_file())
            })
            .as_deref()
    }
    #[cfg(target_arch = "wasm32")]
    None
}

/// The Hangul letters the Korean interface shows: its catalog and its own name.
fn hangul_letters() -> &'static std::collections::BTreeSet<char> {
    static LETTERS: std::sync::OnceLock<std::collections::BTreeSet<char>> = std::sync::OnceLock::new();
    LETTERS.get_or_init(|| {
        include_str!("i18n/ko.tsv")
            .lines()
            .filter_map(|line| line.split('\t').nth(2))
            .flat_map(str::chars)
            .chain("한국어".chars())
            .filter(|c| matches!(c, '\u{1100}'..='\u{11ff}' | '\u{3130}'..='\u{318f}' | '\u{a960}'..='\u{a97f}' | '\u{ac00}'..='\u{d7ff}'))
            .collect()
    })
}

/// Whether `face` has every Hangul letter the Korean interface shows (rejects malformed fonts
/// and Chinese- or Japanese-only faces).
fn covers_hangul(face: &cadcraft_fonts::ttf::Face) -> bool {
    hangul_letters().iter().all(|c| face.has_glyph(*c))
}

/// The first face of a font file (the one egui uses) when it covers the Korean interface.
fn korean_face(bytes: std::sync::Arc<Vec<u8>>) -> Option<cadcraft_fonts::ttf::Face> {
    cadcraft_fonts::ttf::faces(&bytes).into_iter().find(covers_hangul)
}

/// Whether the CJK fonts [`install_fonts`] loads in every language already show Hangul (Noto Sans
/// CJK, Microsoft YaHei, PingFang do). Only asked about Korean, so other interfaces never read the
/// font twice. Checked once.
pub fn cjk_covers_hangul() -> bool {
    static COVERS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *COVERS.get_or_init(|| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // egui uses a collection's first face, and each letter from the first font with it.
            let first_covers = |p: &std::path::Path| {
                read_capped(p, MAX_UI_FONT_BYTES)
                    .and_then(|bytes| cadcraft_fonts::ttf::faces(&std::sync::Arc::new(bytes)).into_iter().next())
                    .is_some_and(|face| covers_hangul(&face))
            };
            if cjk_font().is_some_and(first_covers) {
                return true;
            }
        }
        cadcraft_fonts::CRAFT_FONTS
            .iter()
            .filter(|f| f.style == "Regular" && f.scripts.iter().any(|s| matches!(*s, "Jpan" | "Hans" | "Hant")))
            .any(|f| cadcraft_fonts::ttf::faces(&std::sync::Arc::new(f.bytes.to_vec())).first().is_some_and(covers_hangul))
    })
}

/// A Hangul font for the Korean interface when the CJK fonts don't cover it: a Korean face from
/// the optional craft-fonts input, else one installed where the OS or common packages put it.
/// Nothing is bundled or downloaded; without one the interface stays English. Only looked for
/// when Korean is asked about. Checked once; collections are scanned with the fonts crate's
/// face-count cap and reads are capped.
pub fn korean_font() -> Option<&'static cadcraft_fonts::ttf::Face> {
    static FOUND: std::sync::OnceLock<Option<cadcraft_fonts::ttf::Face>> = std::sync::OnceLock::new();
    FOUND
        .get_or_init(|| {
            for font in cadcraft_fonts::CRAFT_FONTS.iter().filter(|f| f.scripts.iter().any(|s| matches!(*s, "Hang" | "Kore"))) {
                if font.bytes.len() as u64 > MAX_UI_FONT_BYTES {
                    continue;
                }
                if let Some(face) = korean_face(std::sync::Arc::new(font.bytes.to_vec())) {
                    return Some(face);
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let candidates: &[&str] = if cfg!(target_os = "macos") {
                    &[
                        "/System/Library/Fonts/AppleSDGothicNeo.ttc",
                        "/System/Library/Fonts/Supplemental/AppleGothic.ttf",
                        "/Library/Fonts/NanumGothic.ttf",
                    ]
                } else if cfg!(windows) {
                    &["C:\\Windows\\Fonts\\malgun.ttf", "C:\\Windows\\Fonts\\gulim.ttc"]
                } else {
                    &[
                        "/usr/share/fonts/truetype/nanum/NanumGothic.ttf",
                        "/usr/share/fonts/nanum/NanumGothic.ttf",
                        "/usr/share/fonts/opentype/noto/NotoSansCJKkr-Regular.otf",
                        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
                        "/usr/share/fonts/google-noto-cjk/NotoSansCJK-Regular.ttc",
                        "/usr/local/share/fonts/nanum/NanumGothic.ttf",
                        "/usr/local/share/fonts/noto/NotoSansCJK-Regular.ttc",
                    ]
                };
                for path in candidates {
                    if let Some(face) =
                        read_capped(std::path::Path::new(path), MAX_UI_FONT_BYTES).and_then(|bytes| korean_face(std::sync::Arc::new(bytes)))
                    {
                        return Some(face);
                    }
                }
            }
            None
        })
        .as_ref()
}

/// Whether the interface in `language` shows Hangul (the Korean name in the language picker):
/// in every language when the CJK fonts cover it, in a Korean interface with [`korean_font`].
pub fn shows_hangul(language: &str) -> bool {
    cjk_covers_hangul() || (language == "ko" && korean_font().is_some())
}

/// Whether the interface can show `language` without missing-glyph boxes: Chinese needs an
/// installed CJK font, Korean a font with Hangul, العربية an installed Arabic font.
pub fn can_show(language: &str) -> bool {
    match language {
        "ko" => shows_hangul("ko"),
        "ar" => arabic_font().is_some(),
        l if l.starts_with("zh") => cjk_font().is_some(),
        _ => true,
    }
}

/// egui's visuals for `palette`, from its tokens.
fn visuals(palette: Palette) -> Visuals {
    let theme = palette.theme();
    let t = palette.tokens();
    let mut v = match theme {
        Theme::Dark => Visuals::dark(),
        Theme::Light => Visuals::light(),
    };
    v.panel_fill = t.panel;
    v.window_fill = t.chrome;
    v.extreme_bg_color = t.field;
    v.faint_bg_color = t.chrome_dark;
    v.override_text_color = Some(t.text);
    v.widgets.noninteractive.bg_fill = t.panel;
    v.widgets.noninteractive.fg_stroke.color = t.text;
    v.widgets.noninteractive.bg_stroke.color = t.border;
    v.widgets.inactive.bg_fill = t.control;
    v.widgets.inactive.weak_bg_fill = t.control;
    v.widgets.inactive.fg_stroke.color = t.text;
    v.widgets.hovered.bg_fill = t.control_hover;
    v.widgets.hovered.weak_bg_fill = t.control_hover;
    v.widgets.active.bg_fill = t.accent;
    v.widgets.active.weak_bg_fill = t.accent;
    v.selection.bg_fill = t.select_bg;
    v.window_stroke = egui::Stroke::new(1.0, t.border);
    let shadow = Color32::from_black_alpha(if theme == Theme::Dark { 90 } else { 40 });
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 12, spread: 0, color: shadow };
    v.window_corner_radius = egui::CornerRadius::same(6);
    v.menu_corner_radius = egui::CornerRadius::same(5);
    if let Some(b) = palette.brand() {
        // The UFRGS palettes: links, warnings, errors and selected text from the brand tones, and
        // shadows in the brand's darkest tone instead of plain black.
        v.hyperlink_color = b.foco;
        v.warn_fg_color = b.aviso;
        v.error_fg_color = b.erro;
        v.selection.stroke.color = b.texto;
        v.widgets.active.fg_stroke.color = b.texto_destaque;
        let deep = fisilab_brand::palette::NOITE.fundo;
        let shadow = Color32::from_rgba_unmultiplied(deep.r(), deep.g(), deep.b(), if b.escuro { 150 } else { 60 });
        v.popup_shadow.color = shadow;
        v.window_shadow.color = shadow;
    }
    v
}

/// Install the styles of both themes of `palette`'s family (UFRGS Noite and Dia, or the upstream
/// Dark and Light) and show `palette` (egui's own widgets, popups and menus follow it; our own
/// painting follows [`Tokens::get`]). `follow_system` is true for the [`ThemePref::System`]
/// choice, whose palette was resolved from the OS report.
pub fn apply(ctx: &egui::Context, palette: impl Into<Palette>, follow_system: bool) {
    let palette = palette.into();
    let theme = palette.theme();
    set_active(palette);
    let (dark, light) = palette.family();
    ctx.set_visuals_of(Theme::Dark, visuals(dark));
    ctx.set_visuals_of(Theme::Light, visuals(light));
    // A System choice keeps egui's own System preference: a concrete Light/Dark would pin the native
    // window appearance (winit then stops reporting OS changes) and the app would never follow again.
    ctx.set_theme(if follow_system { egui::ThemePreference::System } else { egui::ThemePreference::from(theme) });
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 3.0);
        s.spacing.interact_size.y = 20.0;
        s.text_styles.insert(egui::TextStyle::Body, body());
        s.text_styles.insert(egui::TextStyle::Button, body());
        s.text_styles.insert(egui::TextStyle::Small, small());
        s.text_styles.insert(egui::TextStyle::Monospace, mono());
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn korean_font_rejects_invalid_or_glyphless_files() {
        for bytes in [vec![], vec![0xff; 32], vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]] {
            assert!(super::korean_face(std::sync::Arc::new(bytes)).is_none());
        }
    }

    use super::{Palette, ThemePref, Tokens};
    use egui::Color32;

    /// WCAG contrast ratio of two opaque colours.
    fn contrast(a: Color32, b: Color32) -> f32 {
        let lum = |c: Color32| {
            let f = |x: u8| {
                let s = f32::from(x) / 255.0;
                if s <= 0.039_28 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
            };
            0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
        };
        let (x, y) = (lum(a), lum(b));
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    }

    #[test]
    fn ufrgs_noite_is_the_default_and_the_ufrgs_themes_come_first() {
        assert_eq!(ThemePref::default(), ThemePref::UfrgsNoite);
        assert_eq!(&ThemePref::ALL[..2], &[ThemePref::UfrgsNoite, ThemePref::UfrgsDia]);
        assert_eq!(ThemePref::default().palette(Some(egui::Theme::Light)), Palette::UfrgsNoite, "a fixed choice ignores the OS");
        assert_eq!(Palette::UfrgsNoite.theme(), egui::Theme::Dark);
        assert_eq!(Palette::UfrgsDia.theme(), egui::Theme::Light);
    }

    #[test]
    fn theme_names_round_trip() {
        for t in ThemePref::ALL {
            assert_eq!(ThemePref::parse(t.as_str()), Some(t));
            assert_eq!(serde_json::to_value(t).ok().and_then(|v| v.as_str().map(str::to_owned)).as_deref(), Some(t.as_str()));
        }
        assert_eq!(ThemePref::parse("ufrgsnoite"), Some(ThemePref::UfrgsNoite));
        assert_eq!(ThemePref::parse(" UFRGSDIA "), Some(ThemePref::UfrgsDia));
        assert_eq!(ThemePref::parse("sepia"), None);
    }

    #[test]
    fn ufrgs_palettes_come_from_the_brand() {
        let (n, d) = (&fisilab_brand::palette::NOITE, &fisilab_brand::palette::DIA);
        assert_eq!((Tokens::NOITE.panel, Tokens::NOITE.accent, Tokens::NOITE.text), (n.painel, n.destaque, n.texto));
        assert_eq!((Tokens::DIA.panel, Tokens::DIA.accent, Tokens::DIA.text), (d.painel, d.destaque, d.texto));
        // The drawing area keeps one model-space look in both, as upstream's Light does.
        assert_eq!(Tokens::DIA.canvas, Tokens::NOITE.canvas);
        assert_eq!(Tokens::NOITE.canvas, n.fundo);
        assert_eq!(Tokens::NOITE.with_model_background(n.fundo).grid_major, Tokens::NOITE.grid_major, "grid derived from the background");
    }

    #[test]
    fn every_theme_keeps_its_text_readable() {
        // Only the suite's own palettes: the upstream ones keep their original colours.
        for p in Palette::ALL.into_iter().filter(|p| p.brand().is_some()) {
            let t = p.tokens();
            for bg in [t.chrome, t.chrome_dark, t.panel, t.control, t.field, t.list_bg] {
                assert!(contrast(t.text, bg) >= 4.5, "{p:?}: text {:?} on {bg:?}", t.text);
                assert!(contrast(t.text_dim, bg) >= 3.0, "{p:?}: dim text {:?} on {bg:?}", t.text_dim);
            }
            assert!(contrast(t.text_faint, t.panel) >= 2.5, "{p:?}: faint text");
            assert!(contrast(t.accent_text, t.accent) >= 4.5, "{p:?}: text on accent");
            assert!(contrast(t.cmd_keyword, t.panel) >= 3.0, "{p:?}: command-line keywords");
        }
    }

    #[test]
    fn applying_a_palette_installs_its_family() {
        let ctx = egui::Context::default();
        super::apply(&ctx, Palette::UfrgsDia, false);
        assert_eq!(super::active_palette(), Palette::UfrgsDia);
        assert_eq!(Tokens::get(), Tokens::DIA);
        assert_eq!(ctx.theme(), egui::Theme::Light);
        assert_eq!(ctx.style_of(egui::Theme::Dark).visuals.panel_fill, Tokens::NOITE.panel);
        assert_eq!(ctx.style_of(egui::Theme::Light).visuals.panel_fill, Tokens::DIA.panel);
        super::apply(&ctx, egui::Theme::Dark, false);
        assert_eq!(Tokens::get(), Tokens::DARK);
        assert_eq!(ctx.style_of(egui::Theme::Light).visuals.panel_fill, Tokens::LIGHT.panel);
    }
}
