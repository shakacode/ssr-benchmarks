import * as React from "react";

export function Layout({ children }) {
  return (
    <div className="container">
      {children}
      <Footer />
    </div>
  );
}

export function Header({ children }) {
  return (
    <div className="header">
      {children}
    </div>
  );
}

export function Content({ children }) {
  return (
    <div className="content">
      {children}
    </div>
  );
}

export function Footer() {
  return (
    <div className="footer">
      (c)
    </div>
  );
}

export function H1({ children }) {
  return (
    <h1 className="h h1">
      {children}
    </h1>
  );
}

export function H2({ children }) {
  return (
    <h2 className="h h2">
      {children}
    </h2>
  );
}
