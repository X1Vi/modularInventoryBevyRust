use bevy::{input::mouse, prelude::*};
use once_cell::sync::Lazy;
use std::cell::LazyCell;

use rand::Rng; // Import the Rng trait for random number generation
use std::sync::Mutex;
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const SOFT_BLUE_YELLOW: Color = Color::srgb(0.85, 0.85, 0.55);

#[derive(Component)] // Added for demonstration purposes; optional
pub struct InventorySlot<T: AsRef<str>> {
    item_name: T, // Accepts both &'static str and String
    item_quantity: u32,
    can_stack: bool,
    is_empty: bool,
    is_selected: bool,
    texture_path: T, // Accepts both &'static str and String
}

// Make the new function a const fn
impl<T: AsRef<str>> InventorySlot<T> {
    const fn new(
        item_name: T,
        texture_path: T,
        item_quantity: u32,
        can_stack: bool,
        is_empty: bool,
        is_selected: bool,
    ) -> Self {
        InventorySlot {
            item_name,
            texture_path,
            item_quantity,
            can_stack,
            is_empty,
            is_selected,
        }
    }
}

trait InventorySlotFunctions {
    fn clear(&mut self);
}

impl InventorySlotFunctions for InventorySlot<String> {
    fn clear(&mut self) {
        self.item_name = String::new();
        self.is_empty = true;
        self.is_selected = true;
        self.item_quantity = 0;
        self.can_stack = false;
        self.texture_path = String::new();
    }
}

static mut CURRENT_INVENTORY_SLOT: Option<&mut InventorySlot<String>> = None;

// Now you can create the constant object using the const constructor
const HEALTH_POTION_INVENTORY_SLOT_ITEM: InventorySlot<&'static str> = InventorySlot::new(
    "Health Potion",                              // Using string literal
    "Keeney_Dungeon_Asssets/Tiles/tile_0113.png", // Using string literal
    1,                                            // item_quantity
    true,                                         // can_stack
    false,                                        // is_empty
    false,                                        // is_selected
);

const WEIRD_DAGGER_INVENTORY_SLOT_ITEM: InventorySlot<&'static str> = InventorySlot::new(
    "Weird Dagger",                               // Using string literal
    "Keeney_Dungeon_Asssets/Tiles/tile_0130.png", // Using string literal
    1,                                            // item_quantity
    false,                                        // can_stack
    false,                                        // is_empty
    false,                                        // is_selected
);

const DAGGER_INVENTORY_SLOT_ITEM: InventorySlot<&'static str> = InventorySlot::new(
    "Dagger",                                     // Using string literal
    "Keeney_Dungeon_Asssets/Tiles/tile_0131.png", // Using string literal
    1,                                            // item_quantity
    false,                                        // can_stack
    false,                                        // is_empty
    false,                                        // is_selected
);

const INVENTORY_SLOTS: [InventorySlot<&'static str>; 3] = [
    HEALTH_POTION_INVENTORY_SLOT_ITEM,
    WEIRD_DAGGER_INVENTORY_SLOT_ITEM,
    DAGGER_INVENTORY_SLOT_ITEM,
];

static MOUSE_SLOT: Lazy<Mutex<InventorySlot<String>>> = Lazy::new(|| {
    Mutex::new(InventorySlot {
        item_name: String::new(),
        texture_path: String::new(),
        item_quantity: 0,
        can_stack: false,
        is_empty: true,
        is_selected: false,
    })
});
// Function to return a random inventory slot
fn get_random_inventory_slot() -> &'static InventorySlot<&'static str> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..INVENTORY_SLOTS.len()); // Generate a random index
    return &INVENTORY_SLOTS[index]; // Return a reference to the random inventory slot
}

const BUTTON_LIMIT: u16 = 12;

pub fn spawn_custom_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the UI camera
    commands.spawn(Camera2dBundle::default());

    // Root UI container
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Second container to hold the grid of buttons
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        height: Val::Percent(90.0),
                        display: Display::Grid,
                        grid_template_columns: vec![
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                        ],
                        grid_template_rows: vec![GridTrack::flex(1.0); 3],
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    background_color: SOFT_BLUE_YELLOW.into(),
                    border_radius: BorderRadius::all(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    // Generate buttons in a 3x3 grid
                    for i in 0..BUTTON_LIMIT {
                        let texture_path = format!("Keeney_Dungeon_Asssets/Tiles/tile_0113.png");
                        let texture_handle = asset_server.load(texture_path.clone());

                        let inventory_slot = get_random_inventory_slot();

                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Percent(90.0),
                                    height: Val::Percent(95.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_direction: FlexDirection::Column, // Stack texture and text vertically
                                    ..default()
                                },
                                border_color: BorderColor(Color::BLACK),
                                border_radius: BorderRadius::all(Val::Px(15.0)),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .insert(InventorySlot {
                                can_stack: inventory_slot.can_stack,
                                is_empty: inventory_slot.is_empty,
                                is_selected: inventory_slot.is_selected,
                                item_name: inventory_slot.item_name.to_string(), // Convert to String
                                item_quantity: inventory_slot.item_quantity,
                                texture_path: inventory_slot.texture_path.to_string(),
                            })
                            .with_children(|parent| {
                                // Add the texture (image) at the top
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(80.0),
                                        height: Val::Px(80.0),
                                        margin: UiRect::bottom(Val::Px(5.0)), // Add spacing between image and text
                                        ..default()
                                    },
                                    image: UiImage::new(texture_handle),
                                    ..default()
                                });

                                // Add the text below the image
                                parent.spawn(TextBundle {
                                    style: Style {
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                    text: Text::from_section(
                                        format!("Item {i}"),
                                        TextStyle {
                                            font: asset_server.load("Fonts/Kenney Pixel.ttf"),
                                            font_size: 16.0,
                                            color: Color::srgb(0.9, 0.9, 0.9),
                                        },
                                    ),
                                    ..default()
                                });
                            });
                    }
                });
        });
}

// Update system to also check the text components
pub fn update_button_information(
    mut button_query: Query<
        (
            &mut InventorySlot<String>,
            &Button,
            &Children,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
    mut texture_query: Query<&mut UiImage>,
    asset_server: Res<AssetServer>,
) {
    for (mut inventory_slot, _button, children, interaction, mut bg_color, mut border_color) in
        button_query.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                println!("Button was clicked!");

                *bg_color = BackgroundColor(Color::rgb(0.1, 0.1, 0.1).into()); // Dark black color
                *border_color = BorderColor(Color::rgb(0.15, 0.15, 0.15).into()); // Dark gray color

                let mut mouse_slot = MOUSE_SLOT.lock().unwrap();

                if !mouse_slot.is_selected {
                    // First click - pick up item
                    if !inventory_slot.is_empty {
                        transfer_to_mouse_slot(&mut mouse_slot, &mut inventory_slot);
                    }
                } else {
                    // Second click - handle placement/stacking/swapping
                    handle_mouse_slot_selection(&mut inventory_slot, &mut mouse_slot);
                }
            }
            Interaction::Hovered => {
                println!("Button is hovered.");
                *bg_color = BackgroundColor(Color::rgb(0.2, 0.2, 0.2).into()); // Dark gray with a yellowish tint
                *border_color = BorderColor(Color::rgb(0.25, 0.25, 0.1).into());
                // Slightly lighter dark gray
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15).into()); // Dark gray
                *border_color = BorderColor(Color::rgb(0.1, 0.1, 0.1).into()); // Darker gray
            }
        }

        // Update children components.
        for &child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                if let Some(section) = text.sections.first_mut() {
                    section.value = format!(
                        "{}: {}",
                        inventory_slot.item_name, inventory_slot.item_quantity
                    );
                }
            }

            if let Ok(mut ui_image) = texture_query.get_mut(child) {
                ui_image.texture = asset_server.load(&inventory_slot.texture_path);
            }
        }
    }
}

fn transfer_to_mouse_slot(
    mouse_slot: &mut InventorySlot<String>,
    inventory_slot: &mut InventorySlot<String>,
) {
    // Store the mouse slot's current state before overwriting
    let old_mouse_item = mouse_slot.item_name.clone();
    let old_mouse_quantity = mouse_slot.item_quantity;
    let old_mouse_texture = mouse_slot.texture_path.clone();
    let old_mouse_can_stack = mouse_slot.can_stack;
    let old_mouse_is_empty = mouse_slot.is_empty;

    // Transfer inventory slot data to mouse slot
    mouse_slot.item_name = inventory_slot.item_name.clone();
    mouse_slot.texture_path = inventory_slot.texture_path.clone();
    mouse_slot.item_quantity = inventory_slot.item_quantity;
    mouse_slot.can_stack = inventory_slot.can_stack;
    mouse_slot.is_empty = inventory_slot.is_empty;
    mouse_slot.is_selected = true;

    // Transfer old mouse slot data to inventory slot
    inventory_slot.item_name = old_mouse_item;
    inventory_slot.texture_path = old_mouse_texture;
    inventory_slot.item_quantity = old_mouse_quantity;
    inventory_slot.can_stack = old_mouse_can_stack;
    inventory_slot.is_empty = old_mouse_is_empty;

    println!(
        "Swapped - Mouse Slot: {} ({}), Inventory Slot: {} ({})",
        mouse_slot.item_name,
        mouse_slot.item_quantity,
        inventory_slot.item_name,
        inventory_slot.item_quantity
    );
}

fn handle_mouse_slot_selection(
    inventory_slot: &mut InventorySlot<String>,
    mouse_slot: &mut InventorySlot<String>,
) {
    if mouse_slot.is_empty {
        return;
    }

    if !inventory_slot.is_empty
        && mouse_slot.item_name == inventory_slot.item_name
        && mouse_slot.can_stack
    {
        // Stack items of the same type
        let total_quantity = mouse_slot.item_quantity + inventory_slot.item_quantity;

        // If there's a stack size limit, you might want to handle it here
        // For example: let max_stack = 64;
        inventory_slot.item_quantity = total_quantity;
        mouse_slot.clear();
        mouse_slot.is_selected = false;

        println!(
            "Stacked items - New quantity: {}",
            inventory_slot.item_quantity
        );
    } else if inventory_slot.is_empty {
        // Transfer to empty slot
        transfer_item(inventory_slot, mouse_slot);
    } else {
        // Swap items between slots
        let temp_name = inventory_slot.item_name.clone();
        let temp_texture = inventory_slot.texture_path.clone();
        let temp_quantity = inventory_slot.item_quantity;
        let temp_can_stack = inventory_slot.can_stack;

        inventory_slot.item_name = mouse_slot.item_name.clone();
        inventory_slot.texture_path = mouse_slot.texture_path.clone();
        inventory_slot.item_quantity = mouse_slot.item_quantity;
        inventory_slot.can_stack = mouse_slot.can_stack;
        inventory_slot.is_empty = false;

        mouse_slot.item_name = temp_name;
        mouse_slot.texture_path = temp_texture;
        mouse_slot.item_quantity = temp_quantity;
        mouse_slot.can_stack = temp_can_stack;
        mouse_slot.is_empty = false;

        println!("Swapped different items between slots");
    }
}

fn transfer_item(
    inventory_slot: &mut InventorySlot<String>,
    mouse_slot: &mut InventorySlot<String>,
) {
    if mouse_slot.is_empty {
        return;
    }

    inventory_slot.item_name = mouse_slot.item_name.clone();
    inventory_slot.texture_path = mouse_slot.texture_path.clone();
    inventory_slot.item_quantity = mouse_slot.item_quantity;
    inventory_slot.can_stack = mouse_slot.can_stack;
    inventory_slot.is_empty = false;

    mouse_slot.clear();
    mouse_slot.is_selected = false;

    println!(
        "Transferred item to inventory: {} ({})",
        inventory_slot.item_name, inventory_slot.item_quantity
    );
}
