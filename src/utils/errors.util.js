class AppError extends Error {
  constructor({ errorMsg = "", shortErrMsg = "" }) {
    this.errorMsg = errorMsg;
    this.shortErrMsg = shortErrMsg;
  }
}

export default AppError;
