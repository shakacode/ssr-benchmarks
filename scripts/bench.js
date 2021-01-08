const autocannon = require("autocannon");

const nextServer = { label: "Next server", port: process.env.NEXT_SERVER_PORT };
const nodeServer = { label: "Node server", port: process.env.NODE_SERVER_PORT };
const rustServer = { label: "Rust server", port: process.env.RUST_SERVER_PORT };

const connections = parseInt(process.env.CONNECTIONS, 10);

if (!process.env.CONNECTIONS) {
  console.error("Number of concurrent connections is not provided.");
  process.exit(1);
} else if (!Number.isInteger(connections) || connections <= 0) {
  console.error("Number of concurrent connections must be a positive integer.");
  process.exit(1);
}

function bench({label, port}, next) {
  console.log(`=== ${label}:`);

  const instance = autocannon({
    url: `http://localhost:${port}`,
    duration: 10,
    connections: connections,
    headers: {
      "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
      "Accept-Encoding": "gzip, deflate",
      "Accept-Language": "en-us",
      "Cookie": "WMF-Last-Access=08-Jan-2021; enwikimwuser-sessionId=041fb3ba99e057d356e5; WMF-Last-Access-Global=08-Jan-2021",
      "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Safari/605.1.15",
    },
  }, (err, res) => {
    if (err) {
      console.error("Error:", err);
      return process.exit(1);
    }
    console.log("\n");
    if (next) next();
  });

  autocannon.track(instance);

  process.once("SIGINT", () => instance.stop())
}

bench(
  nextServer,
  () => bench(
    nodeServer,
    () => bench(
      rustServer,
    ),
  )
)
