import { QLValidationError } from "./errors.js";
export const requiredObject = (value, code, name) => {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new QLValidationError(code, `${name} must be an object.`, { value });
  return value;
};
export const requiredString = (value, code, name) => {
  if (typeof value !== "string" || value.length === 0) throw new QLValidationError(code, `${name} must be a non-empty string.`, { value });
  return value;
};
