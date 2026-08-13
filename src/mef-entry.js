export const mefLens = (id, name, index, face, square, complement, mobius, sublenses) => Object.freeze({
  id, name, version: 1, index, face, square, complement, mobius,
  sublenses: Object.freeze(sublenses.map((name, position) => Object.freeze({ position, name })))
});
