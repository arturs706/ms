import { onMount } from "solid-js";
import styles from "../css/animation.module.css";
import { TimelineMax } from "gsap";

export default function Animation() {
    onMount(() => {
    const spanCont = document.querySelectorAll(".span");
    var tl = new TimelineMax({ 
        repeat: -1
      }); 
      
      tl.staggerFromTo('.span', 1, {x:0}, {x:80}, -0.10);
    });
  
  return (
    <main class={styles.container}>
        <span class={`${styles.span} span`}></span>
        <span class={`${styles.span} span`}></span>
        <span class={`${styles.span} span`}></span>
        <span class={`${styles.span} span`}></span>
    </main>
  );
}
