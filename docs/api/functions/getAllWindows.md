[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getAllWindows

# Function: getAllWindows()

> **getAllWindows**(): `WindowInfo`[]

Defined in: [index.ts:674](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L674)

Get a list of all visible windows

**Note:** Currently returns only the active window due to API limitations of the underlying library.
Future versions may support enumerating all windows.

## Returns

`WindowInfo`[]

Array of WindowInfo objects

## Example

```typescript
import { getAllWindows } from "@tego/botjs";

const windows = getAllWindows();
console.log(`Found ${windows.length} windows`);
windows.forEach(win => {
  console.log(`- ${win.title}`);
});
```
