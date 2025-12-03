//! GUI module - napi bindings for aumate widget system
//!
//! This module provides JavaScript bindings for the aumate GUI widget system,
//! allowing creation of declarative UIs from JavaScript/TypeScript.

// Include all GUI code inline to avoid napi re-export issues
// Allow dead code for methods that will be used when GuiApp/Window classes are added
#![allow(dead_code)]

use aumate::gui::widget::WidgetDef;
use napi::bindgen_prelude::*;
use napi_derive::napi;

// ============================================================================
// Widget Style Types
// ============================================================================

/// Widget style configuration
#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct JsWidgetStyle {
    /// Margin on all sides
    pub margin: Option<f64>,
    /// Padding on all sides
    pub padding: Option<f64>,
    /// Minimum width
    pub min_width: Option<f64>,
    /// Minimum height
    pub min_height: Option<f64>,
    /// Maximum width
    pub max_width: Option<f64>,
    /// Maximum height
    pub max_height: Option<f64>,
    /// Background color as hex string (e.g., "#FF0000" or "#FF0000FF")
    pub background_color: Option<String>,
    /// Text color as hex string
    pub text_color: Option<String>,
    /// Font size in points
    pub font_size: Option<f64>,
    /// Text alignment: "left", "center", "right"
    pub text_align: Option<String>,
}

impl JsWidgetStyle {
    fn to_aumate(&self) -> aumate::gui::widget::WidgetStyle {
        use aumate::gui::widget::{Spacing, TextAlign, WidgetStyle};

        let mut style = WidgetStyle::default();

        if let Some(m) = self.margin {
            style.margin = Spacing::all(m as f32);
        }
        if let Some(p) = self.padding {
            style.padding = Spacing::all(p as f32);
        }
        if let Some(w) = self.min_width {
            style.min_width = Some(w as f32);
        }
        if let Some(h) = self.min_height {
            style.min_height = Some(h as f32);
        }
        if let Some(w) = self.max_width {
            style.max_width = Some(w as f32);
        }
        if let Some(h) = self.max_height {
            style.max_height = Some(h as f32);
        }
        if let Some(ref color) = self.background_color {
            style.background_color = parse_hex_color(color);
        }
        if let Some(ref color) = self.text_color {
            style.text_color = parse_hex_color(color);
        }
        if let Some(size) = self.font_size {
            style.font_size = Some(size as f32);
        }
        if let Some(ref align) = self.text_align {
            style.text_align = Some(match align.as_str() {
                "center" => TextAlign::Center,
                "right" => TextAlign::Right,
                _ => TextAlign::Left,
            });
        }

        style
    }
}

/// Parse hex color string to RGBA array
fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let hex = hex.trim_start_matches('#');
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some([r, g, b, a])
        }
        _ => None,
    }
}

// ============================================================================
// Widget Class
// ============================================================================

/// A widget that can be composed into UIs
#[napi]
#[derive(Debug, Clone)]
pub struct Widget {
    inner: WidgetDef,
}

#[napi]
impl Widget {
    /// Set the widget ID
    #[napi]
    pub fn with_id(&self, id: String) -> Widget {
        Widget { inner: self.inner.clone().with_id(id) }
    }

    /// Set visibility
    #[napi]
    pub fn with_visible(&self, visible: bool) -> Widget {
        Widget { inner: self.inner.clone().with_visible(visible) }
    }

    /// Set enabled state
    #[napi]
    pub fn with_enabled(&self, enabled: bool) -> Widget {
        Widget { inner: self.inner.clone().with_enabled(enabled) }
    }

    /// Set tooltip
    #[napi]
    pub fn with_tooltip(&self, tooltip: String) -> Widget {
        Widget { inner: self.inner.clone().with_tooltip(tooltip) }
    }

    /// Set style
    #[napi]
    pub fn with_style(&self, style: JsWidgetStyle) -> Widget {
        Widget { inner: self.inner.clone().with_style(style.to_aumate()) }
    }

    /// Set spacing (for layout widgets)
    #[napi]
    pub fn with_spacing(&self, spacing: f64) -> Widget {
        Widget { inner: self.inner.clone().with_spacing(spacing as f32) }
    }

    /// Set placeholder (for text input)
    #[napi]
    pub fn with_placeholder(&self, placeholder: String) -> Widget {
        Widget { inner: self.inner.clone().with_placeholder(placeholder) }
    }

    /// Set password mode (for text input)
    #[napi]
    pub fn with_password(&self, password: bool) -> Widget {
        Widget { inner: self.inner.clone().with_password(password) }
    }

    /// Set step (for slider)
    #[napi]
    pub fn with_step(&self, step: f64) -> Widget {
        Widget { inner: self.inner.clone().with_step(step as f32) }
    }

    /// Set max height (for scroll area)
    #[napi]
    pub fn with_max_height(&self, height: f64) -> Widget {
        Widget { inner: self.inner.clone().with_max_height(height as f32) }
    }

    /// Set collapsed state (for group)
    #[napi]
    pub fn with_collapsed(&self, collapsed: bool) -> Widget {
        Widget { inner: self.inner.clone().with_collapsed(collapsed) }
    }

    /// Get the inner WidgetDef (for internal use)
    pub(crate) fn into_inner(self) -> WidgetDef {
        self.inner
    }
}

impl From<WidgetDef> for Widget {
    fn from(def: WidgetDef) -> Self {
        Widget { inner: def }
    }
}

// ============================================================================
// Basic Widget Constructors
// ============================================================================

/// Create a text label widget
#[napi]
pub fn label(text: String) -> Widget {
    Widget::from(WidgetDef::label(text))
}

/// Create a button widget
#[napi]
pub fn button(text: String) -> Widget {
    Widget::from(WidgetDef::button(text))
}

/// Create a text input widget
#[napi]
pub fn text_input() -> Widget {
    Widget::from(WidgetDef::text_input())
}

/// Create a text input with initial value
#[napi]
pub fn text_input_with_value(value: String) -> Widget {
    Widget::from(WidgetDef::text_input_with_value(value))
}

/// Create a checkbox widget
#[napi]
pub fn checkbox(label_text: String, checked: bool) -> Widget {
    Widget::from(WidgetDef::checkbox(label_text, checked))
}

/// Create a slider widget
#[napi]
pub fn slider(value: f64, min: f64, max: f64) -> Widget {
    Widget::from(WidgetDef::slider(value as f32, min as f32, max as f32))
}

/// Create a progress bar widget
#[napi]
pub fn progress_bar(value: f64) -> Widget {
    Widget::from(WidgetDef::progress_bar(value as f32))
}

/// Create a horizontal separator
#[napi]
pub fn separator() -> Widget {
    Widget::from(WidgetDef::separator())
}

/// Create a spacer
#[napi]
pub fn spacer(size: f64) -> Widget {
    Widget::from(WidgetDef::spacer(size as f32))
}

// ============================================================================
// Layout Widget Constructors
// ============================================================================

/// Create a horizontal box layout
#[napi]
pub fn hbox(children: Vec<&Widget>) -> Widget {
    let child_defs: Vec<WidgetDef> = children.into_iter().map(|w| w.inner.clone()).collect();
    Widget::from(WidgetDef::hbox(child_defs))
}

/// Create a vertical box layout
#[napi]
pub fn vbox(children: Vec<&Widget>) -> Widget {
    let child_defs: Vec<WidgetDef> = children.into_iter().map(|w| w.inner.clone()).collect();
    Widget::from(WidgetDef::vbox(child_defs))
}

/// Create a grid layout
/// Each inner array represents a row of widgets
#[napi]
pub fn grid(rows: Vec<Vec<&Widget>>) -> Widget {
    let grid_defs: Vec<Vec<WidgetDef>> =
        rows.into_iter().map(|row| row.into_iter().map(|w| w.inner.clone()).collect()).collect();
    Widget::from(WidgetDef::grid(grid_defs))
}

// ============================================================================
// Container Widget Constructors
// ============================================================================

/// Create a panel container
#[napi]
pub fn panel(child: &Widget) -> Widget {
    Widget::from(WidgetDef::panel(child.inner.clone()))
}

/// Create a scroll area container
#[napi]
pub fn scroll_area(child: &Widget) -> Widget {
    Widget::from(WidgetDef::scroll_area(child.inner.clone()))
}

/// Create a collapsible group
#[napi]
pub fn group(title: String, child: &Widget) -> Widget {
    Widget::from(WidgetDef::group(title, child.inner.clone()))
}

// ============================================================================
// Image Widget Constructor
// ============================================================================

/// Create an image widget from RGBA data
#[napi]
pub fn image(data: Buffer, width: u32, height: u32) -> Widget {
    Widget::from(WidgetDef::image(data.to_vec(), width, height))
}
