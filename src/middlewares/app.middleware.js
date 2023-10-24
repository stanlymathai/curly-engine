const express = require('express');
const cors = require('cors');

const helmet = require('helmet');
const rateLimit = require('express-rate-limit');

const RATE_LIMIT_WINDOW_MS = 15 * 60 * 1000; // 15 minutes
const RATE_LIMIT_MAX_REQUESTS = 100; // Max number of requests per window

const limiter = rateLimit({
  windowMs: RATE_LIMIT_WINDOW_MS,
  max: RATE_LIMIT_MAX_REQUESTS,
  message: { error: 'Please try again later.' },
  headers: true, // Enables X-RateLimit headers
});

module.exports = (app) => {
  app.disable('x-powered-by');
  app.use(helmet());
  app.use(cors());
  app.use(limiter);
  app.use(express.json({ limit: '5mb' }));
  app.use(express.urlencoded({ extended: false }));
};
