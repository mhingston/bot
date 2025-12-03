/**
 * GUI Form Example
 *
 * Demonstrates creating a login form with text inputs,
 * checkboxes, and buttons using the widget builder pattern.
 */

import {
  button,
  checkbox,
  GuiApp,
  hbox,
  label,
  panel,
  textInput,
  vbox,
} from "@tego/botjs";

async function main() {
  console.log("GUI Form Example\n");

  // Create the GUI application
  const app = new GuiApp();

  // Create a form window
  const form = app.createWindow({
    title: "Login Form",
    width: 350,
    height: 280,
    alwaysOnTop: true,
  });

  // Build the form UI
  form.setContent(
    panel(
      vbox([
        // Title
        label("Please enter your credentials"),

        // Username field
        hbox([
          label("Username:").withStyle({ minWidth: 80 }),
          textInput().withId("username").withPlaceholder("Enter username"),
        ]).withSpacing(8),

        // Password field
        hbox([
          label("Password:").withStyle({ minWidth: 80 }),
          textInput().withId("password").withPassword(true),
        ]).withSpacing(8),

        // Remember me checkbox
        checkbox("Remember me", false).withId("remember"),

        // Buttons
        hbox([
          button("Cancel").withId("cancel"),
          button("Login").withId("login").withStyle({
            backgroundColor: "#007AFF",
            textColor: "#FFFFFF",
          }),
        ]).withSpacing(8),
      ]).withSpacing(12),
    ).withStyle({ padding: 16 }),
  );

  // Show the form
  form.show();

  console.log("Form shown. Close the window to exit.");
  console.log("Note: Event callbacks are not yet implemented in this version.");

  // Run the event loop
  app.run();

  console.log("Form closed!");
}

main().catch(console.error);
