export class QLValidationError extends Error {
  constructor(code, message, details = {}) {
    super(message);
    this.name = "QLValidationError";
    this.code = code;
    this.details = details;
  }
}

export class QLUnsupportedError extends Error {
  constructor(code, message, details = {}) {
    super(message);
    this.name = "QLUnsupportedError";
    this.code = code;
    this.details = details;
  }
}
