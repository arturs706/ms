import styles from "../css/nav.module.css";
import { A } from "@solidjs/router";

export default function Nav() {
  return (
    <div class={styles.nav}>
        <h3>Dashboard</h3>
        <A href="/">Home</A>
        <A href="/lettings">Lettings</A>
        <A href="/sales">Sales</A>
        <A href="/admin">Admin</A>
    </div>
  );
}
