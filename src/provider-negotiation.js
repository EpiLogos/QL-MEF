import { inspectQLProvider } from "./provider-inspection.js";
const list=(value)=>Array.isArray(value)?value:[];
const refs=(items)=>new Set(list(items).map((item)=>typeof item==="string"?item:`${item.id}@${item.version}`));
export function negotiateQLProvider(provider,demand={}) {
  const inspected=inspectQLProvider(provider);
  if (inspected.state==="absent"||inspected.state==="incompatible") return Object.freeze({...inspected,satisfied:false,missing:Object.freeze({operations:[],forms:[],lenses:[],extensions:[]})});
  const c=inspected.capabilities, operations=new Set(c.operations), forms=refs(c.supportedForms), lenses=refs(c.supportedLenses), extensions=new Set(c.extensionNamespaces);
  const missing=Object.freeze({
    operations:Object.freeze(list(demand.operations).filter((x)=>!operations.has(x))),
    forms:Object.freeze(list(demand.forms).filter((x)=>!forms.has(typeof x==="string"?x:`${x.id}@${x.version}`))),
    lenses:Object.freeze(list(demand.lenses).filter((x)=>!lenses.has(typeof x==="string"?x:`${x.id}@${x.version}`))),
    extensions:Object.freeze(list(demand.extensions).filter((x)=>!extensions.has(x)))
  });
  const satisfied=Object.values(missing).every((items)=>items.length===0);
  return Object.freeze({state:satisfied?inspected.state:"incompatible",capabilities:c,warnings:inspected.warnings,satisfied,missing});
}
