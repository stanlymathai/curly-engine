const express = require('express');

const middlewares = require('./middlewares/app.middleware');
const errorHandler = require('./utils/errorHandler.util');

// Loads .env file contents for local development.
if (!process.env.APP_ENV) require('dotenv').config();

// db connection.
const db = require('./configs/db.config');
db.establishConnection();

// Creating an Express app.
const app = express();

// middlewares
middlewares(app);

// Routes
// app.use(process.env.API_ENDPOINT + '/', require('./routes'));

// error handler
errorHandler(app);

module.exports = app;
