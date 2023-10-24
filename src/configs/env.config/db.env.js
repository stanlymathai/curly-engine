module.exports = {
  name: process.env.DB_NAME,
  host: process.env.DB_HOST,

  username: process.env.DB_USERNAME,
  password: process.env.DB_PASSWORD,

  socket_timeout: process.env.DB_SOCKET_TIMEOUT,
  connection_timeout: process.env.DB_CONNECTION_TIMEOUT,
};
