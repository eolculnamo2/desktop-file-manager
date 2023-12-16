import "./styles.css";
import App from "./App.svelte";
// TODO use all and make theme configurable
import "carbon-components-svelte/css/g90.css";
const app = new App({
    target: document.getElementById("app")!,
});

export default app;
