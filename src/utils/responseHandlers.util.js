const HTTP_STATUS = require('./httpStatus.util');

/**
 * Creates a structured error response.
 * @param {Object} error - The error object.
 * @returns {Object} A structured error object.
 */
function createErrorResponse(error) {
  return {
    message: error.message || 'An unexpected error occurred',
    ...(process.env.NODE_ENV !== 'production' && { originalError: error }),
  };
}

/**
 * Send a 400 Bad Request response.
 * The server could not understand the request due to invalid syntax.
 * @param {Response} res - Express response object.
 * @param {Object} error - The error object.
 * @returns {Response} Express response object.
 */
function sendBadRequest(res, error) {
  return res.status(HTTP_STATUS.BAD_REQUEST).json({
    success: false,
    error: createErrorResponse(error),
  });
}

/**
 * Send a 500 Internal Server Error response.
 * The server has encountered a situation it doesn't know how to handle.
 * @param {Response} res - Express response object.
 * @param {Object} error - The error object.
 * @returns {Response} Express response object.
 */
function sendInternalServerError(res, error) {
  return res.status(HTTP_STATUS.INTERNAL_SERVER_ERROR).json({
    success: false,
    error: createErrorResponse(error),
  });
}

/**
 * Send a 200 OK response.
 * Standard response for successful HTTP requests.
 * @param {Response} res - Express response object.
 * @param {Array} data - The payload to send in the response.
 * @returns {Response} Express response object.
 */
function sendSuccessResponse(res, data = []) {
  return res.status(HTTP_STATUS.SUCCESS).json({
    success: true,
    data: data,
  });
}

/**
 * Send a 403 Forbidden response.
 * Indicates that the request is understood, but it has been refused
 * or access is not allowed.
 * @param {Response} res - Express response object
 * @param {string} [message] - Optional custom message
 * @returns {Response} Express response object
 */
function sendForbidden(res, message = 'Forbidden') {
  return res.status(HTTP_STATUS.FORBIDDEN).json({
    success: false,
    error: { message },
  });
}

/**
 * Send a 404 Not Found response.
 * Indicates that there is no resource behind the URI.
 * @param {Response} res - Express response object
 * @param {string} [message] - Optional custom message
 * @returns {Response} Express response object
 */
function sendNotFound(res, message = 'Not Found') {
  return res.status(HTTP_STATUS.NOT_FOUND).json({
    success: false,
    error: { message },
  });
}

/**
 * Send a 405 Method Not Allowed response.
 * Indicates that the method specified in the request is known by the server
 * but is not supported by the target resource.
 * @param {Response} res - Express response object
 * @param {string} [message] - Optional custom message
 * @returns {Response} Express response object
 */
function sendMethodNotAllowed(res, message = 'Method Not Allowed') {
  return res.status(HTTP_STATUS.METHOD_NOT_ALLOWED).json({
    success: false,
    error: { message },
  });
}

/**
 * Send a 409 Conflict response.
 * Indicates a request conflict with current state of the target resource.
 * @param {Response} res - Express response object
 * @param {string} [message] - Optional custom message
 * @returns {Response} Express response object
 */
function sendConflict(res, message = 'Conflict') {
  return res.status(HTTP_STATUS.CONFLICT).json({
    success: false,
    error: { message },
  });
}

/**
 * Send a 422 Unprocessable Entity response.
 * Indicates that the server understands the content type of the request entity,
 * but the server was unable to process the contained instructions.
 * @param {Response} res - Express response object
 * @param {Object} error - Detailed error information
 * @returns {Response} Express response object
 */
function sendUnprocessableEntity(res, error) {
  return res.status(HTTP_STATUS.UNPROCESSABLE_ENTITY).json({
    success: false,
    error: createErrorResponse(error),
  });
}

module.exports = {
  sendConflict,
  sendNotFound,
  sendForbidden,
  sendBadRequest,
  sendSuccessResponse,
  sendMethodNotAllowed,
  sendUnprocessableEntity,
  sendInternalServerError,
};
