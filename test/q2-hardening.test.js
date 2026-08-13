import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import { MEF_MANIFOLD, makeQLReading, makeQLTarget } from "../src/index.js";
const provider={id:"fixture-q2",version:"0.2.0-q2"};
test("all Q2 schemas define every required field", async()=>{ for(const f of ["target","provenance","reading","relation","synthesis"]){ const schema=JSON.parse(await readFile(new URL(`../schemas/v1/q2-${f}.schema.json`,import.meta.url),"utf8")); assert.equal(schema.additionalProperties,false); for(const key of schema.required) assert.ok(key in schema.properties,`${f} schema missing ${key}`); } });
test("one client subject identity survives all twelve lens refractions",()=>{ const ref={ref:"factory:claim:all-lenses"}; const target=makeQLTarget({subjectRef:ref}); const provenance={provider,operation:"refract",mode:"semantic-stochastic"}; for(const lens of MEF_MANIFOLD){ const reading=makeQLReading({id:`r-${lens.id}`,target,lens:{id:lens.id,version:1},reading:{},provenance}); assert.equal(reading.target.subjectRef,ref); } });
