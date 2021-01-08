import "./styles.css";

import * as React from "react";
import ReactDom from "react-dom";

import { App } from "./app";

ReactDom.hydrate(
  <App data={window.__DATA__.data} />,
  document.querySelector("#root"),
)
