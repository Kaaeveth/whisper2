import { SvelteURL } from "svelte/reactivity";
import BackendImpl from "./Backend";

export default class OllamaBackend extends BackendImpl {
    readonly name: string = "Ollama";
    readonly apiUrl: URL = new SvelteURL("http://localhost:11434/api");
}
