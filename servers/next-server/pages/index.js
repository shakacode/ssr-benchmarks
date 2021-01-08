const { App } = require("../assets/app");

export default function Home({ data }) {
  return <App data={data} />;
}

export async function getServerSideProps() {
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

  const res = await fetch(
    `http://127.0.0.1:${process.env.RUST_SERVER_PORT}${process.env.GRAPHQL_PATH}`,
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Content-Length": query.length
      },
      body: query,
    },
  );
  const data = await res.json();

  return { props: { ...data } }
}
