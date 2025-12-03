/**
 * GUI Notification Example
 *
 * Demonstrates creating a notification-style window that
 * automatically closes after a few seconds.
 */

import { GuiApp, label, vbox } from "@tego/botjs";

async function main() {
  console.log("GUI Notification Example\n");

  // Create the GUI application
  const app = new GuiApp();

  // Create a notification window (no decorations, always on top)
  const notification = app.createWindow({
    title: "Notification",
    width: 280,
    height: 100,
    alwaysOnTop: true,
    x: 100,
    y: 100,
  });

  // Set notification content
  notification.setContent(
    vbox([
      label("Download complete!").withStyle({ fontSize: 16 }),
      label("file.zip has been saved to Downloads").withStyle({
        fontSize: 12,
        textColor: "#888888",
      }),
    ])
      .withSpacing(8)
      .withStyle({ padding: 16 }),
  );

  // Show the notification
  notification.show();

  console.log("Notification shown. It will auto-close in 5 seconds...");

  // Auto-close after 5 seconds
  setTimeout(() => {
    console.log("Closing notification...");
    notification.close();
    app.exit();
  }, 5000);

  // Run the event loop
  app.run();

  console.log("Done!");
}

main().catch(console.error);
