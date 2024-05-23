import { Title } from "@solidjs/meta";
import { HttpStatusCode } from "@solidjs/start";
import styles from "../css/notfound.module.css";

export default function NotFound() {
  return (
    <main class={styles.main}>
      <Title>Not Found</Title>
      <HttpStatusCode code={404} />
      <h1>Page Not Found</h1>
      <p>
        The page you are looking for does not exist. Go back to{" "}
        <a href="/">Dashboard</a>.
      </p>
    </main>
  );
}
