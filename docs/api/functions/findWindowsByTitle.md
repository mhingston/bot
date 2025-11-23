[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / findWindowsByTitle

# Function: findWindowsByTitle()

> **findWindowsByTitle**(`title`): `WindowInfo`[]

Defined in: [index.ts:699](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L699)

Find windows by title using case-insensitive partial matching

**Note:** Currently searches only the active window due to API limitations of the underlying library.
Future versions may support searching all windows.

## Parameters

### title

`string`

Title text to search for (case-insensitive partial match)

## Returns

`WindowInfo`[]

Array of matching WindowInfo objects

## Example

```typescript
import { findWindowsByTitle } from "@tego/botjs";

// Find Chrome windows
const chromeWindows = findWindowsByTitle('chrome');
chromeWindows.forEach(win => console.log(win.title));

// Find Visual Studio Code windows
const vscodeWindows = findWindowsByTitle('Visual Studio Code');
```
