const { analyze } = require('../pkg/ssvm_nodejs_starter_lib.js');

const http = require('http');
const url = require('url');
const hostname = '0.0.0.0';
const port = 3000;

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url,true).query;
  if (!queryObject['phrase']) {
    res.end(`pass phrase as parameter`);
  } else {
    res.end("Score:"+analyze(queryObject['phrase']));
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});