[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / updateScreenMetrics

# Function: updateScreenMetrics()

> **updateScreenMetrics**(): `void`

Defined in: [index.ts:537](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L537)

Update screen metrics (refresh monitor information)
Call this after display configuration changes

## Returns

`void`

## Example

```typescript
import { updateScreenMetrics, getScreenSize } from "@tego/botjs";

// After connecting/disconnecting monitors
updateScreenMetrics();
const newSize = getScreenSize();
console.log(`Updated screen size: ${newSize.width}x${newSize.height}`);
```
