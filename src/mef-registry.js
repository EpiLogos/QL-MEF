import { MEF_SQUARE_A } from "./mef-square-a.js";
import { MEF_SQUARE_B } from "./mef-square-b.js";
import { MEF_SQUARE_C } from "./mef-square-c.js";
export const MEF_MANIFOLD_VERSION = "1.0.0";
export const MEF_MANIFOLD = Object.freeze(MEF_SQUARE_A.concat(MEF_SQUARE_B, MEF_SQUARE_C));
