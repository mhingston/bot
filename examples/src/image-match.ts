/**
 * Image Template Matching Examples
 *
 * Demonstrates the image template matching API for finding UI elements on screen.
 *
 * Usage: pnpm ex:run image-match
 *
 * Note: This example captures the screen and uses the captured region as a template,
 * which should always find a match. In real usage, you would load a template image
 * from a file.
 */

import * as fs from "node:fs";
import * as path from "node:path";
import {
  captureScreenRegion,
  findAllOnScreen,
  findOnScreen,
  getMatchBounds,
  getMatchCenter,
  imageResource,
  imageResourceFromBuffer,
} from "@tego/botjs";

async function main() {
  console.log("🎯 @tego/botjs Image Template Matching Examples\n");

  // ============================================
  // Example 1: Using a captured screen region as template
  // ============================================
  console.log("📸 Example 1: Self-matching with captured region");
  console.log("  Capturing a 100x100 region from top-left corner...");

  // Capture a small region from the screen
  const capture = await captureScreenRegion(0, 0, 100, 100);
  console.log(
    `  Captured ${capture.width}x${capture.height} region (${capture.image.length} bytes)`,
  );

  // Create an ImageResource from the captured buffer
  const capturedTemplate = imageResourceFromBuffer(capture.image);

  // Try to find this template on screen (should find it at origin)
  console.log("  Searching for captured region on screen...");
  const selfMatch = await findOnScreen(capturedTemplate, {
    confidence: 0.9,
    searchMultipleScales: false, // Exact size match
  });

  if (selfMatch) {
    console.log(
      `  ✅ Found match at (${selfMatch.x}, ${selfMatch.y}) with ${(selfMatch.confidence * 100).toFixed(1)}% confidence`,
    );
    const center = getMatchCenter(selfMatch);
    console.log(`  Center point: (${center.x}, ${center.y})`);
    const bounds = getMatchBounds(selfMatch);
    console.log(
      `  Bounds: (${bounds.left}, ${bounds.top}) to (${bounds.right}, ${bounds.bottom})`,
    );
  } else {
    console.log("  ❌ No match found (this is unexpected for self-matching)");
  }

  // ============================================
  // Example 2: Find all matches (multi-scale)
  // ============================================
  console.log("\n🔍 Example 2: Find all matches with multi-scale search");

  // Capture a smaller unique region
  const smallCapture = await captureScreenRegion(50, 50, 50, 50);
  const smallTemplate = imageResourceFromBuffer(smallCapture.image);

  // Search with lower confidence and multiple scales
  const allMatches = await findAllOnScreen(smallTemplate, {
    confidence: 0.5, // Lower threshold
    searchMultipleScales: true,
    scaleSteps: [1.0, 0.9, 0.8],
    limit: 5,
  });

  console.log(`  Found ${allMatches.length} matches:`);
  for (let i = 0; i < Math.min(allMatches.length, 3); i++) {
    const match = allMatches[i];
    console.log(
      `    ${i + 1}. Position: (${match.x}, ${match.y}), Confidence: ${(match.confidence * 100).toFixed(1)}%, Scale: ${match.scale}`,
    );
  }

  // ============================================
  // Example 3: Load template from file (if exists)
  // ============================================
  console.log("\n📁 Example 3: Loading template from file");

  // Check if a sample template exists
  const samplePath = path.join(
    process.cwd(),
    "examples",
    "sample-template.png",
  );
  if (fs.existsSync(samplePath)) {
    console.log(`  Loading template from: ${samplePath}`);
    const fileTemplate = await imageResource(samplePath);
    console.log(`  Template loaded: ${fileTemplate.buffer.length} bytes`);

    const fileMatch = await findOnScreen(fileTemplate, { confidence: 0.8 });
    if (fileMatch) {
      console.log(
        `  ✅ Found at (${fileMatch.x}, ${fileMatch.y}) with ${(fileMatch.confidence * 100).toFixed(1)}% confidence`,
      );
    } else {
      console.log("  ❌ Template not found on screen");
    }
  } else {
    console.log(`  ⚠️ Sample template not found at: ${samplePath}`);
    console.log(
      "  To test file loading, save a PNG image and update the path.",
    );
  }

  // ============================================
  // Example 4: Configuration options
  // ============================================
  console.log("\n⚙️ Example 4: Configuration options demo");
  console.log("  Available configuration options:");
  console.log("    - searchMultipleScales: boolean (default: true)");
  console.log("    - useGrayscale: boolean (default: false)");
  console.log(
    "    - scaleSteps: number[] (default: [1.0, 0.9, 0.8, 0.7, 0.6, 0.5])",
  );
  console.log("    - confidence: number 0.0-1.0 (default: 0.8)");
  console.log("    - limit: number (default: 100)");

  // High-precision search example
  console.log("\n  Testing high-precision search (confidence: 0.95)...");
  const preciseMatch = await findOnScreen(capturedTemplate, {
    confidence: 0.95,
    searchMultipleScales: false,
  });
  console.log(
    `  High precision result: ${preciseMatch ? `Found at (${preciseMatch.x}, ${preciseMatch.y})` : "No match"}`,
  );

  // ============================================
  // Usage Tips
  // ============================================
  console.log("\n💡 Usage Tips:");
  console.log("  1. Use imageResource() to load templates from PNG/JPG files");
  console.log("  2. Use imageResourceFromBuffer() for in-memory image data");
  console.log(
    "  3. Lower confidence = more matches but potential false positives",
  );
  console.log("  4. Use getMatchCenter() to get click target coordinates");
  console.log("  5. Use waitFor() to wait for an element to appear");
  console.log("  6. Use waitForGone() to wait for an element to disappear");

  console.log("\n✅ Image template matching examples completed!");
}

main().catch(console.error);
