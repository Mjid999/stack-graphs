/*--- path: a.js ---*/

export default function foo() { }

/*--- path: index.js ---*/

export { default as default } from "./a.js";

/*--- path: index2.js ---*/

import bar from "./index.js";

/**/ bar;
//   ^ defined: 3, 7, 11