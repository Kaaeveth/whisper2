<script lang="ts">
    import { A, Heading, P } from "flowbite-svelte";
    import { getVersion } from "@tauri-apps/api/app";
</script>

<div class="w-full">
    <Heading tag="h2">Whisper2</Heading>
    <P class="my-2">
        Small and simple Windows desktop chat client for large language models written in Rust and Svelte.<br>
    </P>
    <P class="my-2">
        <span>
            {#await getVersion() then version}
                v{version}
            {/await}
        </span>
        <br>
        <A href="https://github.com/Kaaeveth/whisper2" rel="noopener noreferrer" target="_blank">Github</A>
    </P>
    <Heading tag="h3">License</Heading>
    <P class="mt-2">
<pre>
MIT License

Copyright (c) 2025 Dominik Strutz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
</pre>
    </P>
</div>
