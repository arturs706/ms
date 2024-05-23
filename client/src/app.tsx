import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start";
import { Suspense } from "solid-js";
import "./css/app.css";
import Nav from "./components/Nav";


export default function App() {
  return (
    <Router
      root={props => (
        <MetaProvider>
          <Title>Dashboard</Title>
          <Nav/>
          <Suspense>{props.children}</Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
