const mongoose = require('mongoose');
const dbConfig = require('./env.config/db.env');

function mongoConnUrl(config) {
  const { username, password, host, name } = config;
  return `mongodb+srv://${username}:${password}@${host}.mongodb.net/${name}?retryWrites=true&w=majority`;
}

mongoose.Promise = global.Promise;
const MONGO_CONN_URL = mongoConnUrl(dbConfig);
const RETRY_BASE_INTERVAL = 2000;
const MAX_RETRIES = 5;

async function establishConnection(attempt = 1) {
  try {
    const { connection_timeout, socket_timeout } = dbConfig;
    await mongoose.connect(MONGO_CONN_URL, {
      connectTimeoutMS: connection_timeout,
      socketTimeoutMS: socket_timeout,

      useUnifiedTopology: true,
      useNewUrlParser: true,
    });
    console.log('DB connection successful');
  } catch (err) {
    console.error(
      `Failed to connect to db on attempt ${attempt}: ${err.message}`
    );

    if (attempt < MAX_RETRIES) {
      let exponentialBackoff = RETRY_BASE_INTERVAL * Math.pow(2, attempt);
      let jitter = Math.random() * exponentialBackoff * 0.5;
      let sleepTime = exponentialBackoff + jitter;

      console.log(`Retrying in ${sleepTime / 1000} seconds...`);
      setTimeout(() => establishConnection(attempt + 1), sleepTime);
    } else gracefulShutdown(new Error('Max connection retries reached.'));
  }
}

mongoose.connection.on('error', (err) => {
  console.error('DB connection error:', err);
  gracefulShutdown(err);
});

mongoose.connection.on('disconnected', () => {
  console.warn('DB connection lost.');
});

mongoose.connection.on('reconnected', () => {
  console.info('DB reconnected!');
});

mongoose.connection.on('reconnectFailed', () => {
  console.error('DB reconnection failed. Exiting...');
  gracefulShutdown(new Error('DB reconnection failed.'));
});

// Graceful shutdown
function gracefulShutdown(err) {
  console.error('Encountered an error:', err.message);
  console.info('Initiating graceful shutdown...');

  mongoose.connection
    .close()
    .then(() => {
      console.log('MongoDB connection closed due to app termination.');
      process.exit(1); // Exit with error
    })
    .catch((error) => {
      console.error('Error while closing MongoDB connection:', error);
      process.exit(1);
    });
}

// Shutdown process for signals
async function handleShutdown(signal) {
  console.log(`Received ${signal}. Closing MongoDB connection...`);
  try {
    await mongoose.connection.close();
    console.log('MongoDB connection closed due to app termination.');
    process.exit(0);
  } catch (error) {
    console.error('Error while closing MongoDB connection:', error);
    process.exit(1);
  }
}

// Listen for shutdown signals
process.on('SIGINT', handleShutdown.bind(null, 'SIGINT')); // interrupt signal
process.on('SIGTERM', handleShutdown.bind(null, 'SIGTERM')); // Termination signal

module.exports = { establishConnection };
