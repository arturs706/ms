import { Title } from "@solidjs/meta";
import styles from "../css/home.module.css";
import { createSignal } from "solid-js";

export default function Home() {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [responseHeaders, setResponseHeaders] = createSignal("");

  const handleLogin = async (e: any) => {
    e.preventDefault();
    const data = {
      username: username(),
      passwd: password()
    };

    try {
      const response = await fetch("http://localhost:10001/api/v1/users/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
      });

      if (!response.ok) {
        throw new Error("Login failed");
      }

      // Get response headers
      const headers = response.headers;
      console.log("Response headers:", headers.forEach((value, name) => console.log(name, value)));
      setResponseHeaders(JSON.stringify(headers));

      // Handle successful login response here
      const responseData = await response.json();
      console.log("Login successful:", responseData);
    } catch (error) {
      console.error("Error logging in:", error);
    }
  };

  return (
    <main>
      <Title>Dashboard</Title>
      <div class={styles.main}>
        <h3>Dashboard</h3>
        <form onSubmit={handleLogin}>
          <input
            type="text"
            placeholder="Username"
            value={username()}
            onInput={(e) => setUsername(e.target.value)}
          />
          <input
            type="password"
            placeholder="Password"
            value={password()}
            onInput={(e) => setPassword(e.target.value)}
          />
          <button type="submit">Login</button>
        </form>
        <div>Response Headers: {responseHeaders()}</div>
      </div>
    </main>
  );
}
