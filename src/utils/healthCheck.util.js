const http = require('http');

const HOST = 'localhost';
const PORT = process.env.PORT || '8080';
const HEALTH_PATH = '/api/v1/health';
const TIMEOUT = 2000; // 2 seconds
const SUCCESS_CODE = 200;
const MAX_RETRIES = 3;
const METHOD = 'GET';

let retries = 0;

const options = {
  host: HOST,
  port: PORT,
  method: METHOD,
  timeout: TIMEOUT,
  path: HEALTH_PATH,
};

function retryOrExit() {
  if (retries < MAX_RETRIES) {
    retries++;
    console.log(`Retrying (${retries}/${MAX_RETRIES})...`);
    setTimeout(checkServiceHealth, TIMEOUT);
  } else {
    console.log('Max retries reached.');
    process.exit(1);
  }
}

function checkServiceHealth() {
  const request = http.request(options, (res) => {
    console.log(`STATUS: ${res.statusCode}`);
    res.resume();
    if (res.statusCode === SUCCESS_CODE) {
      process.exit(0);
    } else {
      retryOrExit();
    }
  });

  request.on('error', function (err) {
    console.log('ERROR:', err.message);
    retryOrExit();
  });

  request.on('timeout', function () {
    console.log('ERROR: Request timed out');
    request.destroy();
    retryOrExit();
  });

  request.end();
}

checkServiceHealth();
