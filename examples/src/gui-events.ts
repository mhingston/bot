/**
 * GUI Events Example
 *
 * Demonstrates event handling for widget interactions using the polling API.
 * This example shows how to receive button clicks, text changes,
 * checkbox toggles, and slider value changes.
 */

import {
  button,
  checkbox,
  GuiApp,
  hbox,
  label,
  panel,
  slider,
  textInput,
  vbox,
} from "@tego/botjs";

async function main() {
  console.log("GUI Events Example\n");

  // Create the GUI application
  const app = new GuiApp();

  // Create an event demo window
  const win = app.createWindow({
    title: "Event Demo",
    width: 400,
    height: 350,
    alwaysOnTop: true,
  });

  // Build the UI with various interactive widgets
  win.setContent(
    panel(
      vbox([
        label("Widget Event Demo").withStyle({ fontSize: 16 }),

        // Text input section
        label("Text Input:"),
        textInput().withId("text-input").withPlaceholder("Type here..."),

        // Checkbox section
        checkbox("Enable feature", false).withId("feature-checkbox"),

        // Slider section
        label("Volume:"),
        slider(50, 0, 100).withId("volume-slider").withStep(5),

        // Button section
        hbox([
          button("Click Me").withId("click-btn"),
          button("Submit").withId("submit-btn").withStyle({
            backgroundColor: "#007AFF",
            textColor: "#FFFFFF",
          }),
          button("Close").withId("close-btn").withStyle({
            backgroundColor: "#FF3B30",
            textColor: "#FFFFFF",
          }),
        ]).withSpacing(8),
      ]).withSpacing(12),
    ).withStyle({ padding: 16 }),
  );

  // Show the window
  console.log("Showing window...");
  win.show();

  console.log("Window shown. Interact with widgets to see events.");
  console.log("Click 'Close' button to exit.\n");

  // Initialize the event loop for non-blocking mode
  app.init();

  // Track whether we should exit
  let shouldExit = false;

  // Event handler function
  function handleEvent(event: {
    eventType: string;
    widgetId: string;
    value?: string | null;
    checked?: boolean | null;
    numberValue?: number | null;
  }) {
    console.log(`Event: ${event.eventType}`);
    console.log(`  Widget ID: ${event.widgetId}`);

    switch (event.eventType) {
      case "button_click":
        console.log(`  Button "${event.widgetId}" was clicked!`);
        if (event.widgetId === "close-btn") {
          console.log("\nClosing window...");
          shouldExit = true;
          win.close();
          app.exit();
        }
        break;

      case "text_changed":
        console.log(`  New text: "${event.value}"`);
        break;

      case "text_submit":
        console.log(`  Text submitted: "${event.value}"`);
        break;

      case "checkbox_changed":
        console.log(
          `  Checkbox is now: ${event.checked ? "checked" : "unchecked"}`,
        );
        break;

      case "slider_changed":
        console.log(`  Slider value: ${event.numberValue}`);
        break;

      default:
        console.log(`  (unhandled event type)`);
    }
    console.log("");
  }

  // Event loop using setImmediate for non-blocking operation
  function pump() {
    // Pump the GUI event loop once
    const running = app.runOnce();

    // Poll for any pending events
    const events = app.pollEvents();
    for (const event of events) {
      handleEvent(event);
    }

    // Continue pumping if still running and not exiting
    if (running && !shouldExit) {
      setImmediate(pump);
    } else {
      console.log("Event loop finished. Application closed!");
    }
  }

  console.log("Starting event loop...");
  pump();
}

main().catch(console.error);
