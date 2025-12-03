/**
 * GUI Widgets Example
 *
 * Demonstrates all the new advanced widgets:
 * - Dropdown (select)
 * - RadioGroup
 * - TextArea
 * - Tabs
 */

import {
  button,
  dropdown,
  GuiApp,
  hbox,
  label,
  panel,
  radioGroup,
  tabs,
  textArea,
  vbox,
} from "@tego/botjs";

async function main() {
  console.log("GUI Widgets Example\n");

  // Create the GUI application
  const app = new GuiApp();

  // Create a widget demo window
  const win = app.createWindow({
    title: "Widget Gallery",
    width: 500,
    height: 500,
    alwaysOnTop: true,
  });

  // Build the UI showcasing all widgets
  win.setContent(
    panel(
      tabs([
        {
          label: "Dropdown",
          content: vbox([
            label("Select your favorite color:"),
            dropdown(["Red", "Green", "Blue", "Yellow", "Purple"])
              .withId("color-dropdown")
              .withPlaceholder("Choose a color"),

            label("Select your country:"),
            dropdown(["USA", "UK", "Canada", "Australia", "Japan"])
              .withId("country-dropdown")
              .withSelected(2), // Canada selected by default
          ]).withSpacing(8),
        },
        {
          label: "Radio Group",
          content: vbox([
            label("Select payment method:"),
            radioGroup(["Credit Card", "PayPal", "Bank Transfer"]).withId(
              "payment-radio",
            ),

            label("Select size (horizontal):"),
            radioGroup(["S", "M", "L", "XL"])
              .withId("size-radio")
              .withHorizontal(true),
          ]).withSpacing(12),
        },
        {
          label: "Text Area",
          content: vbox([
            label("Enter your feedback:"),
            textArea()
              .withId("feedback-area")
              .withPlaceholder("Type your feedback here...")
              .withRows(6),

            label("Bio (pre-filled):"),
            textArea().withId("bio-area").withRows(4),
          ]).withSpacing(8),
        },
        {
          label: "Combined",
          content: vbox([
            label("Combined Example"),
            hbox([
              vbox([
                label("Theme:"),
                radioGroup(["Light", "Dark", "System"]).withId("theme-radio"),
              ]).withSpacing(4),
              vbox([
                label("Language:"),
                dropdown(["English", "Spanish", "French", "German"]).withId(
                  "lang-dropdown",
                ),
              ]).withSpacing(4),
            ]).withSpacing(20),

            label("Notes:"),
            textArea().withId("notes-area").withRows(3),

            button("Submit").withId("submit-btn").withStyle({
              backgroundColor: "#007AFF",
              textColor: "#FFFFFF",
            }),
          ]).withSpacing(8),
        },
      ]).withId("main-tabs"),
    ).withStyle({ padding: 12 }),
  );

  // Register event callback
  win.onEvent((event) => {
    console.log(`Event: ${event.eventType}`);
    console.log(`  Widget ID: ${event.widgetId}`);

    switch (event.eventType) {
      case "selection_changed":
        console.log(`  Selected index: ${event.numberValue}`);
        console.log(`  Selected value: "${event.value}"`);
        break;

      case "radio_changed":
        console.log(`  Selected index: ${event.numberValue}`);
        console.log(`  Selected value: "${event.value}"`);
        break;

      case "tab_changed":
        console.log(`  Active tab index: ${event.numberValue}`);
        console.log(`  Tab label: "${event.value}"`);
        break;

      case "text_changed":
        console.log(`  Text: "${event.value?.substring(0, 50)}..."`);
        break;

      case "button_click":
        console.log(`  Button clicked: ${event.widgetId}`);
        if (event.widgetId === "submit-btn") {
          console.log("\n=== Form Submitted ===\n");
        }
        break;

      default:
        console.log(
          `  Event value: ${event.value || event.numberValue || "(none)"}`,
        );
    }
    console.log("");
  });

  // Show the window
  console.log("Showing window...");
  win.show();

  console.log("Window shown. Interact with widgets to see events.");
  console.log("Press Ctrl+C to exit.\n");
  console.log("Starting event loop...");

  // Run the event loop
  app.run();

  console.log("Event loop finished. Application closed!");
}

main().catch(console.error);
