import * as React from "react";

import { Layout, Header, Content, H1, H2 } from "./components";

export function App({ data }) {
  return (
    <Layout>
      <Header>
        <H1> Posts </H1>
      </Header>
      <Content>
        <div className="posts">
          {data.posts.map(post =>
            <div key={post.id} className="post">
              <H2> {post.title} </H2>
              <div className="postContent"> {post.content} </div>
            </div>
          )}
        </div>
      </Content>
    </Layout>
  );
}
