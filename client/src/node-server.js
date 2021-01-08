import * as React from "react";
import ReactDomServer from "react-dom/server";
import serialize from "serialize-javascript";

import { App } from "./app";
import { html } from "./html";

export function render({data}) {
  const json = JSON.parse(data);

  return html({
    data: serialize(data),
    markup: ReactDomServer.renderToString(<App data={json.data} />),
  });
}
