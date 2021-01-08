const express = require("express");
const body = require("body-parser");
const cookies = require("cookie-parser");
const http = require("http");
const ssr = require("./assets/ssr.js");

const app = express();

app.use(body.json());
app.use(body.urlencoded({ extended: true }));

app.use(cookies());

app.use("/assets", express.static("assets"));

app.use(
  "/",
  (req, res, next) => {
    res.header("Cache-Control", "private, no-cache, no-store, must-revalidate");
    res.header("Expires", "-1");
    res.header("Pragma", "no-cache");
    next();
  },
  async (req, res, next) => {
    try {
      const data = await fetchData();
      const html = ssr.render({data});
      res.send(html);
    } catch (err) {
      console.error(err);
      res.status(500).end();
    }
  }
);

app.set("port", process.env.NODE_SERVER_PORT);

app.listen(app.get("port"), function () {
  console.log(`Express server is running on port ${this.address().port}`);
});

function fetchData() {
  return new Promise (resolve => {
    const query = JSON.stringify({
      query:
        `
          fragment PostFragment on Post {
            id
            title
            content
          }

          query GetAllPosts {
            posts {
              ...PostFragment
            }
          }
        `,
      variables: null,
    });

    const options = {
      hostname: "127.0.0.1",
      port: process.env.RUST_SERVER_PORT,
      path: process.env.GRAPHQL_PATH,
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Content-Length": query.length
      }
    };

    const req =
      http.request(options, res => {
        let data = "";
        res.setEncoding("utf8");
        res.on("data", chunk => data = data + chunk);
        res.on("end", () => resolve(data));
      });

    req.on("error", error => {
      console.error(error);
    });

    req.write(query);
    req.end();
  });
};
