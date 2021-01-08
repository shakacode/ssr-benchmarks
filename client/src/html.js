export function html({data, markup}) {
  return (
    `
      <!DOCTYPE html>
      <html>
        <head>
          <base href="/" />
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <title>SSR</title>
          <link href="/assets/app.css" rel="stylesheet">
        </head>
        <body>
          <script>
            window.__DATA__ = JSON.parse(${data});
          </script>
          <div id="root">
            ${markup}
          </div>
          <script src="/assets/app.js"></script>
        </body>
      </html>
    `
  );
}
