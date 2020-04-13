# What Input?

**A global utility for tracking the current input method (mouse, keyboard or touch).**

## What Input is now v5

Now with more information and less opinion!

What Input adds data attributes to the `window` based on the type of input being used. It also exposes a simple API that can be used for scripting interactions.

## How it works

What Input uses event bubbling on the `window` to watch for mouse, keyboard and touch events (via `mousedown`, `keydown` and `touchstart`). It then sets or updates a `data-whatinput` attribute.

Pointer Events are supported but note that `pen` inputs are remapped to `touch`.

What Input also exposes a tiny API that allows the developer to ask for the current input, set custom ignore keys, and set and remove custom callback functions.

_What Input does not make assumptions about the input environment before the page is interacted with._ However, the `mousemove` and `pointermove` events are used to set a `data-whatintent="mouse"` attribute to indicate that a mouse is being used _indirectly_.

## Demo

Check out the demo to see What Input in action.

https://ten1seven.github.io/what-input

### Interacting with Forms

Since interacting with a form _always_ requires use of the keyboard, What Input uses the `data-whatintent` attribute to display a "buffered" version of input events while form `<input>`s, `<select>`s, and `<textarea>`s are being interacted with (i.e. mouse user's `data-whatintent` will be preserved as `mouse` while typing).

## Installing

Download the file directly.

Install via Yarn:

```shell
yarn add what-input
```

Install via NPM:

```shell
npm install what-input
```

## Usage

Include the script directly in your project.

```html
<script src="path/to/what-input.js"></script>
```

Or require with a script loader.

```javascript
import 'what-input'

// or

import whatInput from 'what-input'

// or

require('what-input')

// or

var whatInput = require('what-input')

// or

requirejs.config({
  paths: {
    whatInput: 'path/to/what-input'
  }
})

require(['whatInput'], function() {})
```

What Input will start doing its thing while you do yours.

### Basic Styling

```css
/*
 * only suppress the focus ring once what-input has successfully started
 */

/* suppress focus ring on form controls for mouse users */
[data-whatintent='mouse'] *:focus {
  outline: none;
}
```

**Note:** If you remove outlines with `outline: none;`, be sure to provide clear visual `:focus` styles so the user can see which element they are on at any time for greater accessibility. Visit [W3C's WCAG 2.0 2.4.7 Guideline](https://www.w3.org/TR/UNDERSTANDING-WCAG20/navigation-mechanisms-focus-visible.html) to learn more.

### Scripting

#### Current Input

Ask What Input what the current input method is. This works best if asked after the events What Input is bound to (`mousedown`, `keydown` and `touchstart`).

```javascript
whatInput.ask() // returns `mouse`, `keyboard` or `touch`

myButton.addEventListener('click', () => {
  if (whatInput.ask() === 'mouse') {
    // do mousy things
  } else if (whatInput.ask() === 'keyboard') {
    // do keyboard things
  }
})
```

If it's necessary to know if `mousemove` is being used, use the `'intent'` option. For example:

```javascript
/*
 * nothing has happened but the mouse has moved
 */

whatInput.ask() // returns `initial` because the page has not been directly interacted with
whatInput.ask('intent') // returns `mouse` because mouse movement was detected

/*
 * the keyboard has been used, then the mouse was moved
 */

whatInput.ask() // returns `keyboard` because the keyboard was the last direct page interaction
whatInput.ask('intent') // returns `mouse` because mouse movement was the most recent action detected
```

### Current Element

Ask What Input the currently focused DOM element.

```javascript
whatInput.element() // returns a string, like `input` or null
```

#### Ignore Keys

Set a custom array of [keycodes](http://keycode.info/) that will be ignored (will not switch over to `keyboard`) when pressed. _A custom list will overwrite the default values._

```javascript
/*
 * default ignored keys:
 * 16, // shift
 * 17, // control
 * 18, // alt
 * 91, // Windows key / left Apple cmd
 * 93  // Windows menu / right Apple cmd
 */

whatInput.ignoreKeys([1, 2, 3])
```

#### Specific Keys

Set a custom array of [keycodes](http://keycode.info/) that will trigger the keyboard pressed intent (will not switch to `keyboard` unless these keys are pressed). _This overrides ignoreKeys._

```javascript
// only listen to tab keyboard press
whatInput.specificKeys([9])
```

#### Custom Callbacks

Fire a function when the input or intent changes.

```javascript
// create a function to be fired
var myFunction = function(type) {
  console.log(type)
}

// fire `myFunction` when the intent changes
whatInput.registerOnChange(myFunction, 'intent')

// fire `myFunction` when the input changes
whatInput.registerOnChange(myFunction, 'input')

// remove custom event
whatInput.unRegisterOnChange(myFunction)
```

## Compatibility

What Input works in all modern browsers.

## Changelog

### v5.2.3

- **Fixed:** `activeElement` is null bug in IE is fixed (thanks @EasterPeanut).
- **Fixed:** Mousewheel event detection works correctly again.

### v5.2.1

- **Fixed:** iOS was occasionally reporting `mouse` because of event execution order.
- **Added:** `touchend` to input map
- **Added:** Allows buttons inside forms to be treated like other form inputs.
- **Added:** iTouch intent indicator in demo page (it worked all along, you just couldn't see it).

### v5.1.4

- **Fixed:** Increase buffering time by 20ms to fix iOS reporting mousedown
- **Fixed:** Adds `touchend` to input map

### v5.1.3

- **Added:** Sourcemap for the minified version.

### v5.1.2

- **Added:** `specificKeys` functionality to allow overriding of keyboard keys list. Fix via [bk3](https://github.com/bk3).

### v5.1.1

- **Fixed:** Browsers with cookies turned off would throw an error with session storage. Fix via [yuheiy](https://github.com/yuheiy).

### v5.1.0

- **Added:** Session variable stores last used input and intent so subsiquent page loads don't have to wait for interactions to set the correct input and intent state.
- **Removed:** IE8 support.

### v5.0.7

- **Fixed:** `unRegisterOnChange` failed to unregister items at index 0.

### v5.0.5

- **Fixed:** Fail gracefully in non-DOM environments.

### v5.0.3

- **Fixed:** Event buffer for touch was not working correctly.

### Changes from v4

- **Added:** A the ability to add and remove custom callback function when the input or intent changes with `whatInput.registerOnChange` and `whatInput.unRegisterOnChange`.
- **Added:** A `data-whatelement` attribute exposes any currently focused DOM element (i.e. `data-whatelement="a"` or `data-whatelement="input"`).
- **Added:** A `data-whatclasses` attribute exposes any currently focused element's classes as a comma-separated list (i.e. `data-whatclasses="class1,class2"`).
- **Added:** An API option to provide a custom array of keycodes that will be ignored.
- **Changed:** Typing in form fields is no longer filtered out. The `data-whatinput` attribute immediately reflects the current input. The `data-whatintent` attribute now takes on the role of remembering mouse input prior to typing in or clicking on a form field.
- **Changed:** If you use the Tab key to move from one input to another one - the `data-whatinput` attribute reflects the current input (switches to "keyboard").
- **Removed:** `whatInput.types()` API option.
- **Removed:** Bower support.
- **Fixed:** Using mouse modifier keys (`shift`, `control`, `alt`, `cmd`) no longer toggles back to keyboard.

## Acknowledgments

Special thanks to [Viget](http://viget.com/) for their encouragement and commitment to open source projects. Visit [code.viget.com](http://code.viget.com/) to see more projects from [Viget](http://viget.com).

Thanks to [mAAdhaTTah](https://github.com/mAAdhaTTah) for the initial conversion to Webpack.

What Input is written and maintained by [@ten1seven](https://github.com/ten1seven).

## License

What Input is freely available under the [MIT License](http://opensource.org/licenses/MIT).
