use imgui::{Context, Direction, Style, StyleColor};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ImStyleToml {
    alpha: Option<f32>,
    disabledAlpha: Option<f32>,
    windowPadding: Option<[f32; 2]>,
    windowRounding: Option<f32>,
    windowBorderSize: Option<f32>,
    windowMinSize: Option<[f32; 2]>,
    windowTitleAlign: Option<[f32; 2]>,
    windowMenuButtonPosition: Option<String>,
    childRounding: Option<f32>,
    childBorderSize: Option<f32>,
    popupRounding: Option<f32>,
    popupBorderSize: Option<f32>,
    framePadding: Option<[f32; 2]>,
    frameRounding: Option<f32>,
    frameBorderSize: Option<f32>,
    itemSpacing: Option<[f32; 2]>,
    itemInnerSpacing: Option<[f32; 2]>,
    cellPadding: Option<[f32; 2]>,
    touchExtraPadding: Option<[f32; 2]>,
    indentSpacing: Option<f32>,
    columnsMinSpacing: Option<f32>,
    scrollbarSize: Option<f32>,
    scrollbarRounding: Option<f32>,
    grabMinSize: Option<f32>,
    grabRounding: Option<f32>,
    logSliderDeadzone: Option<f32>,
    tabRounding: Option<f32>,
    tabBorderSize: Option<f32>,
    tabMinWidthForCloseButton: Option<f32>,
    colorButtonPosition: Option<String>,
    buttonTextAlign: Option<[f32; 2]>,
    selectableTextAlign: Option<[f32; 2]>,
    displayWindowPadding: Option<[f32; 2]>,
    displaySafeAreaPadding: Option<[f32; 2]>,
    mouseCursorScale: Option<f32>,
    antiAliasedLines: Option<bool>,
    antiAliasedLinesUseTex: Option<bool>,
    antiAliasedFill: Option<bool>,
    curveTessellationTol: Option<f32>,
    circleTessellationMaxError: Option<f32>,
    colors: Option<HashMap<String, [u8; 4]>>,
}

pub struct ImStyle {
    pub style: Style,
}

impl ImStyle {
    /// Create a new Style filled with your default values (matching your Nim initImGuiStyle)
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_toml(toml_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parsed: ImStyleToml = toml::from_str(toml_str)?;

        let mut s = Self::new();

        if let Some(alpha) = parsed.alpha {
            s.style.alpha = alpha;
        }
        if let Some(disabled) = parsed.disabledAlpha {
            s.style.disabled_alpha = disabled;
        }
        if let Some(pad) = parsed.windowPadding {
            s.style.window_padding = pad;
        }
        if let Some(round) = parsed.windowRounding {
            s.style.window_rounding = round;
        }
        if let Some(border) = parsed.windowBorderSize {
            s.style.window_border_size = border;
        }
        if let Some(min_size) = parsed.windowMinSize {
            s.style.window_min_size = min_size;
        }
        if let Some(title_align) = parsed.windowTitleAlign {
            s.style.window_title_align = title_align;
        }
        if let Some(pos) = parsed.windowMenuButtonPosition {
            s.style.window_menu_button_position = match pos.as_str() {
                "Left" | "left" => Direction::Left,
                "Right" | "right" => Direction::Right,
                _ => Direction::Left,
            };
        }
        if let Some(round) = parsed.childRounding {
            s.style.child_rounding = round;
        }
        if let Some(border) = parsed.childBorderSize {
            s.style.child_border_size = border;
        }
        if let Some(round) = parsed.popupRounding {
            s.style.popup_rounding = round;
        }
        if let Some(border) = parsed.popupBorderSize {
            s.style.popup_border_size = border;
        }
        if let Some(pad) = parsed.framePadding {
            s.style.frame_padding = pad;
        }
        if let Some(round) = parsed.frameRounding {
            s.style.frame_rounding = round;
        }
        if let Some(border) = parsed.frameBorderSize {
            s.style.frame_border_size = border;
        }
        if let Some(spacing) = parsed.itemSpacing {
            s.style.item_spacing = spacing;
        }
        if let Some(inner) = parsed.itemInnerSpacing {
            s.style.item_inner_spacing = inner;
        }
        if let Some(pad) = parsed.cellPadding {
            s.style.cell_padding = pad;
        }
        if let Some(pad) = parsed.touchExtraPadding {
            s.style.touch_extra_padding = pad;
        }
        if let Some(indent) = parsed.indentSpacing {
            s.style.indent_spacing = indent;
        }
        if let Some(min) = parsed.columnsMinSpacing {
            s.style.columns_min_spacing = min;
        }
        if let Some(size) = parsed.scrollbarSize {
            s.style.scrollbar_size = size;
        }
        if let Some(round) = parsed.scrollbarRounding {
            s.style.scrollbar_rounding = round;
        }
        if let Some(size) = parsed.grabMinSize {
            s.style.grab_min_size = size;
        }
        if let Some(round) = parsed.grabRounding {
            s.style.grab_rounding = round;
        }
        if let Some(deadzone) = parsed.logSliderDeadzone {
            s.style.log_slider_deadzone = deadzone;
        }
        if let Some(round) = parsed.tabRounding {
            s.style.tab_rounding = round;
        }
        if let Some(border) = parsed.tabBorderSize {
            s.style.tab_border_size = border;
        }
        if let Some(min_width) = parsed.tabMinWidthForCloseButton {
            s.style.tab_min_width_for_close_button = min_width;
        }
        if let Some(pos) = parsed.colorButtonPosition {
            s.style.color_button_position = match pos.as_str() {
                "Left" | "left" => Direction::Left,
                "Right" | "right" => Direction::Right,
                _ => Direction::Right,
            };
        }
        if let Some(align) = parsed.buttonTextAlign {
            s.style.button_text_align = align;
        }
        if let Some(align) = parsed.selectableTextAlign {
            s.style.selectable_text_align = align;
        }
        if let Some(pad) = parsed.displayWindowPadding {
            s.style.display_window_padding = pad;
        }
        if let Some(pad) = parsed.displaySafeAreaPadding {
            s.style.display_safe_area_padding = pad;
        }
        if let Some(scale) = parsed.mouseCursorScale {
            s.style.mouse_cursor_scale = scale;
        }
        if let Some(a) = parsed.antiAliasedLines {
            s.style.anti_aliased_lines = a;
        }
        if let Some(a) = parsed.antiAliasedLinesUseTex {
            s.style.anti_aliased_lines_use_tex = a;
        }
        if let Some(a) = parsed.antiAliasedFill {
            s.style.anti_aliased_fill = a;
        }
        if let Some(val) = parsed.curveTessellationTol {
            s.style.curve_tessellation_tol = val;
        }
        if let Some(val) = parsed.circleTessellationMaxError {
            s.style.circle_tesselation_max_error = val;
        }

        // Fill colors
        if let Some(colors_map) = parsed.colors {
            for (name, rgba) in colors_map {
                if let Some(idx) = style_color_from_name(&name) {
                    s.style.colors[idx as usize] = [
                        rgba[0] as f32 / 255.0,
                        rgba[1] as f32 / 255.0,
                        rgba[2] as f32 / 255.0,
                        rgba[3] as f32 / 255.0,
                    ];
                }
            }
        }

        Ok(s)
    }

    /// Set the provided style as the current context style.
    pub fn set_current(&self, ctx: &mut Context) {
        *ctx.style_mut() = self.style;
    }
}

impl Default for ImStyle {
    fn default() -> Self {
        Self {
            style: Style {
                alpha: 1.0,
                disabled_alpha: 0.60,
                window_padding: [8.0, 8.0],
                window_rounding: 0.0,
                window_border_size: 1.0,
                window_min_size: [32.0, 32.0],
                window_title_align: [0.0, 0.5],
                window_menu_button_position: Direction::Left,

                child_rounding: 0.0,
                child_border_size: 1.0,

                popup_rounding: 0.0,
                popup_border_size: 1.0,

                frame_padding: [4.0, 3.0],
                frame_rounding: 0.0,
                frame_border_size: 0.0,

                item_spacing: [8.0, 4.0],
                item_inner_spacing: [4.0, 4.0],

                cell_padding: [4.0, 2.0],

                touch_extra_padding: [0.0, 0.0],

                indent_spacing: 21.0,
                columns_min_spacing: 6.0,

                scrollbar_size: 14.0,
                scrollbar_rounding: 9.0,

                grab_min_size: 10.0,
                grab_rounding: 0.0,

                log_slider_deadzone: 4.0,

                tab_rounding: 4.0,
                tab_border_size: 0.0,
                tab_min_width_for_close_button: 0.0,

                color_button_position: Direction::Right,
                button_text_align: [0.5, 0.5],
                selectable_text_align: [0.0, 0.0],

                display_window_padding: [19.0, 19.0],
                display_safe_area_padding: [3.0, 3.0],

                mouse_cursor_scale: 1.0,

                anti_aliased_lines: true,
                anti_aliased_lines_use_tex: true,
                anti_aliased_fill: true,

                curve_tessellation_tol: 1.25,
                circle_tesselation_max_error: 0.30,

                colors: [[0.0_f32; 4]; 53],
            },
        }
    }
}

/// Helper to convert string to StyleColor enum
fn style_color_from_name(name: &str) -> Option<StyleColor> {
    use imgui::StyleColor::*;
    match name {
        "Text" => Some(Text),
        "TextDisabled" => Some(TextDisabled),
        "WindowBg" => Some(WindowBg),
        "ChildBg" => Some(ChildBg),
        "PopupBg" => Some(PopupBg),
        "Border" => Some(Border),
        "BorderShadow" => Some(BorderShadow),
        "FrameBg" => Some(FrameBg),
        "FrameBgHovered" => Some(FrameBgHovered),
        "FrameBgActive" => Some(FrameBgActive),
        "TitleBg" => Some(TitleBg),
        "TitleBgActive" => Some(TitleBgActive),
        "TitleBgCollapsed" => Some(TitleBgCollapsed),
        "MenuBarBg" => Some(MenuBarBg),
        "ScrollbarBg" => Some(ScrollbarBg),
        "ScrollbarGrab" => Some(ScrollbarGrab),
        "ScrollbarGrabHovered" => Some(ScrollbarGrabHovered),
        "ScrollbarGrabActive" => Some(ScrollbarGrabActive),
        "CheckMark" => Some(CheckMark),
        "SliderGrab" => Some(SliderGrab),
        "SliderGrabActive" => Some(SliderGrabActive),
        "Button" => Some(Button),
        "ButtonHovered" => Some(ButtonHovered),
        "ButtonActive" => Some(ButtonActive),
        "Header" => Some(Header),
        "HeaderHovered" => Some(HeaderHovered),
        "HeaderActive" => Some(HeaderActive),
        "Separator" => Some(Separator),
        "SeparatorHovered" => Some(SeparatorHovered),
        "SeparatorActive" => Some(SeparatorActive),
        "ResizeGrip" => Some(ResizeGrip),
        "ResizeGripHovered" => Some(ResizeGripHovered),
        "ResizeGripActive" => Some(ResizeGripActive),
        "Tab" => Some(Tab),
        "TabHovered" => Some(TabHovered),
        "TabActive" => Some(TabActive),
        "TabUnfocused" => Some(TabUnfocused),
        "TabUnfocusedActive" => Some(TabUnfocusedActive),
        "PlotLines" => Some(PlotLines),
        "PlotLinesHovered" => Some(PlotLinesHovered),
        "PlotHistogram" => Some(PlotHistogram),
        "PlotHistogramHovered" => Some(PlotHistogramHovered),
        "TableHeaderBg" => Some(TableHeaderBg),
        "TableBorderStrong" => Some(TableBorderStrong),
        "TableBorderLight" => Some(TableBorderLight),
        "TableRowBg" => Some(TableRowBg),
        "TableRowBgAlt" => Some(TableRowBgAlt),
        "TextSelectedBg" => Some(TextSelectedBg),
        "DragDropTarget" => Some(DragDropTarget),
        "NavHighlight" => Some(NavHighlight),
        "NavWindowingHighlight" => Some(NavWindowingHighlight),
        "NavWindowingDimBg" => Some(NavWindowingDimBg),
        "ModalWindowDimBg" => Some(ModalWindowDimBg),
        _ => None,
    }
}
