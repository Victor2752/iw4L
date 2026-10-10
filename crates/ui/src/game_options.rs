//! Options IW4L adds to Black Ops' own in-game options menus, as rows built
//! like the game's: the mouse sensitivity typed as a number, below the
//! game's slider (Controls → Look), and the frames-per-second counter
//! (Graphics). They are IW4L's additions, not Black Ops' options.

use std::sync::Arc;

use asset_game::{MenuDef, MenuEditField, MenuEvent, MenuItem, SessionHudMenus};
use bevy::prelude::*;
use frame::ClientSet;

const SENSITIVITY_ROW: &str = "iw4l_options_look_sensitivity_value";
const FPS_ROW: &str = "iw4l_options_graphics_show_fps";

/// The items of one Black Ops options row: the bar, the highlight bar, the
/// label, the highlighted label and the control.
const ROW_ITEMS: usize = 5;
const HIGHLIGHT_ITEMS: [usize; 2] = [1, 3];
const LABEL_ITEMS: [usize; 2] = [2, 3];

/// The menu engine's decimal field item type: digits, a point and a sign.
const ITEM_TYPE_DECIMALFIELD: i32 = 17;

/// Black Ops expression function indices (`expressions/functions.txt` + 24).
const FN_LOCALVARINT: i32 = 65;
const FN_LOCALVARSTRING: i32 = 68;
const FN_MENUHASFOCUS: i32 = 309;
const OP_NOT: i32 = 8;
const OP_EQUALS: i32 = 13;
const OP_AND: i32 = 15;

pub(crate) fn register(app: &mut App) {
    app.add_systems(Update, add_iw4l_options.in_set(ClientSet::Ui));
}

fn add_iw4l_options(menus: Option<ResMut<SessionHudMenus>>) {
    let Some(mut menus) = menus else {
        return;
    };
    if !menus.is_changed() {
        return;
    }
    let Some(catalog) = menus.0.as_deref() else {
        return;
    };
    let patched = catalog
        .get("options_look")
        .is_none_or(|menu| menu.items.iter().any(|item| item.name == SENSITIVITY_ROW));
    if patched {
        return;
    }
    let mut catalog = catalog.clone();
    if let Some(menu) = catalog.menus.get_mut("options_look") {
        add_sensitivity_value(menu);
    }
    if let Some(menu) = catalog.menus.get_mut("options_graphics") {
        add_show_fps(menu);
    }
    menus.0 = Some(Arc::new(catalog));
}

/// A row below the sensitivity slider whose number is typed: Enter edits it,
/// Enter again sets `sensitivity`.
fn add_sensitivity_value(menu: &mut MenuDef) {
    let Some(slider) = menu
        .items
        .iter()
        .position(|item| item.dvar.eq_ignore_ascii_case("sensitivity") && item.slider.is_some())
    else {
        return;
    };
    // The row above is a choice; its value column is where the number goes.
    let style = slider
        .checked_sub(ROW_ITEMS)
        .and_then(|above| menu.items.get(above))
        .filter(|item| !item.choices.is_empty())
        .cloned();
    let Some(mut row) = clone_row(
        menu,
        slider,
        SENSITIVITY_ROW,
        "MOUSE SENSITIVITY (VALUE)",
        RowSlot::Next,
    ) else {
        return;
    };
    if let Some(control) = row.last_mut() {
        control.item_type = ITEM_TYPE_DECIMALFIELD;
        control.slider = None;
        control.edit_field = Some(MenuEditField {
            max_chars: 6,
            masked: false,
            starts_empty: true,
        });
        if let Some(style) = style {
            control.font_enum = style.font_enum;
            control.text_align_mode = style.text_align_mode;
            control.text_align_x = style.text_align_x;
            control.text_align_y = style.text_align_y;
            control.text_scale = style.text_scale;
        }
    }
    menu.items.extend(row);
}

/// A yes/no row for the FPS counter in the field of view row's place, which
/// Black Ops shows only in multiplayer; the counter's row shows when that
/// row does not.
fn add_show_fps(menu: &mut MenuDef) {
    let Some(template) = menu
        .items
        .iter()
        .position(|item| item.choices.len() == 2 && item.dvar.eq_ignore_ascii_case("sm_enable"))
    else {
        return;
    };
    let slot = match menu
        .items
        .iter()
        .position(|item| item.dvar.eq_ignore_ascii_case("cg_fov_default"))
    {
        Some(fov) => RowSlot::InPlaceOf(fov),
        None => RowSlot::Next,
    };
    let Some(mut row) = clone_row(menu, template, FPS_ROW, "SHOW FPS", slot) else {
        return;
    };
    if let Some(control) = row.last_mut() {
        control.dvar = "cg_drawFPS".into();
    }
    menu.items.extend(row);
}

/// Where a copied row goes.
enum RowSlot {
    /// One row step below the lowest row, always shown.
    Next,
    /// On the row whose control is this item, shown while that row is not.
    InPlaceOf(usize),
}

/// The row whose control is `control`, copied as a new row numbered after
/// the highest, with `label` as its text and `name` on its items.
fn clone_row(
    menu: &MenuDef,
    control: usize,
    name: &str,
    label: &str,
    slot: RowSlot,
) -> Option<Vec<MenuItem>> {
    let first = control.checked_sub(ROW_ITEMS - 1)?;
    let template = menu.items.get(first..=control)?;
    let template_name = template.last()?.name.clone();
    let from = row_number(&template_name)?;
    let prefix = template_name.strip_suffix(&from.to_string())?;
    let rows: Vec<&MenuItem> = menu
        .items
        .iter()
        .filter(|item| item.name.starts_with(prefix) && row_number(&item.name).is_some())
        .collect();
    let to = rows
        .iter()
        .filter_map(|item| row_number(&item.name))
        .max()?
        + 1;
    let lowest = rows.iter().map(|item| item.rect.y).fold(f32::MIN, f32::max);
    let step = control
        .checked_sub(ROW_ITEMS)
        .and_then(|above| menu.items.get(above))
        .map(|above| template[ROW_ITEMS - 1].rect.y - above.rect.y)
        .filter(|step| *step > 0.0)?;
    let (y, unless) = match slot {
        RowSlot::Next => (lowest + step, None),
        RowSlot::InPlaceOf(other) => {
            let other = menu.items.get(other)?;
            (other.rect.y, other.vis_exp.strip_prefix("t5 "))
        }
    };
    let mut row = template.to_vec();
    for (index, item) in row.iter_mut().enumerate() {
        item.rect.y = y;
        item.float_exp.clear();
        let shown = if HIGHLIGHT_ITEMS.contains(&index) {
            Some(highlight_visible(to))
        } else {
            None
        };
        item.vis_exp = match (shown, unless) {
            (Some(shown), Some(unless)) => format!("{shown} {unless} #{OP_NOT} #{OP_AND}"),
            (None, Some(unless)) => format!("t5 {unless} #{OP_NOT}"),
            (Some(shown), None) => shown,
            (None, None) => String::new(),
        };
        if LABEL_ITEMS.contains(&index) {
            item.text_exp.clear();
            item.text_key = label.to_owned();
            item.text_literal = true;
        }
        if item.name == template_name {
            item.name = name.to_owned();
        }
        let rename = |text: &str| {
            text.replace(
                &format!("ui_highlight {from} "),
                &format!("ui_highlight {to} "),
            )
            .replace(&template_name, name)
        };
        let handlers = &mut item.handlers;
        for events in [
            &mut handlers.action,
            &mut handlers.accept,
            &mut handlers.focus,
            &mut handlers.leave_focus,
            &mut handlers.mouse_enter,
            &mut handlers.mouse_exit,
        ] {
            for event in events.iter_mut() {
                match event {
                    MenuEvent::Script(text) => *text = rename(text),
                    MenuEvent::T5Script(link) => link.action = rename(&link.action),
                    _ => {}
                }
            }
        }
    }
    Some(row)
}

/// The number after a row item's last `_`.
fn row_number(name: &str) -> Option<i32> {
    name.rsplit_once('_')?.1.parse().ok()
}

/// Black Ops' row highlight test: `menuhasfocus() && localvarint("ui_highlight")
/// == row && localvarstring("ui_choicegroup") == ""`, as a `t5` program.
fn highlight_visible(row: i32) -> String {
    let text = |value: &str| {
        let hex: String = value.bytes().map(|b| format!("{b:02x}")).collect();
        format!("s:{hex}")
    };
    format!(
        "t5 #{FN_MENUHASFOCUS} {} #{FN_LOCALVARINT} i:{row} #{OP_EQUALS} #{OP_AND} {} #{FN_LOCALVARSTRING} {} #{OP_EQUALS} #{OP_AND}",
        text("ui_highlight"),
        text("ui_choicegroup"),
        text(""),
    )
}
