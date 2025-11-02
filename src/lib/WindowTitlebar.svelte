<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { onMount } from 'svelte';

    type ClickHandler = () => boolean;
    interface Props {
        title: string;
        onclose?: ClickHandler;
        onminimize?: ClickHandler;
        onmaximize?: ClickHandler;
    }

    let props: Props = $props();
    const thisWindow = getCurrentWindow();
    let isMaximized = $state(false);

    onMount(() => {
        thisWindow.isMaximized().then(m => isMaximized = m);
        const unlisten = thisWindow.onResized(async () => {
            isMaximized = await thisWindow.isMaximized();
        });
        return () => unlisten.then(unlisten => unlisten());
    });

    function onClose() {
        (props.onclose ?? thisWindow.close.bind(thisWindow))();
    }

    function onMaximize() {
        if(isMaximized)
            thisWindow.unmaximize();
        else
            (props.onmaximize ?? thisWindow.maximize.bind(thisWindow))();
    }

    function onMinimize() {
        (props.onminimize ?? thisWindow.minimize.bind(thisWindow))();
    }
</script>

<div data-tauri-drag-region class="w-full h-8 flex justify-between border-b-1 dark:border-gray-600 select-none">
    <span class="dark:text-gray-200 font-bold pt-1 pl-2">
        {props.title}
    </span>
    <div class="inline">
        <!-- Minimize button -->
        <button class="h-full w-12 hover:cursor-pointer hover:bg-gray-500" aria-label="Minimize" tabindex="-1" onclick={() => onMinimize()}>
            <svg viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 m-auto stroke-2 dark:stroke-gray-300 stroke-black fill-transparent">
      		    <path fill-rule="evenodd" d="M 4.9191571,9.9865342 H 15.107774" clip-rule="evenodd"></path>
      		</svg>
        </button>

        <!-- Maximize button -->
        <button class="h-full w-12 hover:cursor-pointer hover:bg-gray-500" aria-label="Maximize" tabindex="-1" onclick={() => onMaximize()}>
            {#if isMaximized}
                <svg viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 m-auto stroke-1 dark:stroke-gray-300 stroke-black fill-transparent">
          		    <path fill-rule="evenodd" d="M 7.0687349,4.043615 A 0.2406889,0.2406889 0 0 0 6.8284243,4.2844435 V 6.5363194 H 4.3093066 A 0.2406889,0.2406889 0 0 0 4.068996,6.7766299 v 8.6662011 a 0.2406889,0.2406889 0 0 0 0.2403106,0.24031 h 8.6662004 a 0.2406889,0.2406889 0 0 0 0.24031,-0.24031 v -2.252395 h 2.519636 a 0.2406889,0.2406889 0 0 0 0.239793,-0.239792 V 4.2844435 A 0.2406889,0.2406889 0 0 0 15.735453,4.043615 Z M 7.3090455,4.5242362 H 15.494624 V 12.709815 H 13.215817 V 6.7766299 A 0.2406889,0.2406889 0 0 0 12.975507,6.5363194 H 7.3090455 Z M 4.5496172,7.0169405 H 12.735196 V 15.202519 H 4.5496172 Z" clip-rule="evenodd"></path>
          		</svg>
            {:else}
                <svg viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 m-auto stroke-2 dark:stroke-gray-300 stroke-black fill-transparent">
          		    <path fill-rule="evenodd" d="M 4.4102168,4.3826919 H 15.609889 V 15.582364 H 4.4102168 Z" clip-rule="evenodd"></path>
          		</svg>
            {/if}
        </button>

        <!-- Close button -->
        <button class="h-full w-12 hover:bg-red-500 hover:cursor-pointer" aria-label="Close" tabindex="-1" onclick={() => onClose()}>
    		<svg viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 m-auto dark:fill-gray-300">
    		    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
    		</svg>
        </button>
    </div>
</div>
