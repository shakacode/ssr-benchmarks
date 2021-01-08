import * as React from "react";
import ReactDomServer from "react-dom/server";

import { App } from "./app";
import { html } from "./html";

export function render({url, jsonData, hydrationData}) {
  return html({
    data: hydrationData,
    markup: ReactDomServer.renderToString(<App data={jsonData} />),
  });
}
