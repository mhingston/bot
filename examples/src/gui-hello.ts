/**
 * GUI Hello World Example
 *
 * Demonstrates basic window creation with widget content.
 * Creates a simple window with a label and a close button.
 */

import { button, GuiApp, label, vbox } from "@tego/botjs";

async function main() {
  console.log("GUI Hello World Example\n");

  // Create the GUI application (spawns the GUI thread)
  const app = new GuiApp();

  // Create a window with configuration
  const win = app.createWindow({
    title: "Hello World",
    width: 300,
    height: 150,
    alwaysOnTop: true,
  });

  // Set the widget content using the builder pattern
  win.setContent(
    vbox([
      label("Hello from @tego/botjs!"),
      label("This is a GUI window with widgets."),
      button("Close").withId("close-btn"),
    ]).withSpacing(16),
  );

  // Show the window
  win.show();

  console.log("Window shown. Close the window to exit.");

  // Run the event loop (blocks until all windows are closed)
  app.run();

  console.log("Done!");
}

main().catch(console.error);
