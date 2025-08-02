import { showError, showInfo, showWarning, type Notification } from "./Snackbar.svelte";

const UNIT_SUFFIX = ["Bytes", "KB", "MB", "GB"];

export function formatByteSize(size: number): string {
    let prefixCount = 0;
    let decimals = 0;
    while(size > 1000) {
        decimals = size % 1000;
        size = Math.trunc(size / 1000);
        if(++prefixCount >= UNIT_SUFFIX.length-1)
            break;
    }
    decimals = Math.trunc(decimals / 100);
    return `${size}.${decimals} ${UNIT_SUFFIX[prefixCount]}`;
}

export function handleError(e: any, level: Notification["level"] = "error") {
    let showFn;
    switch(level) {
        case 'info':
            showFn = showInfo
            break;
        case 'warn':
            showFn = showWarning;
            break;
        default:
            console.error("Unknown level - Using 'error'");
        case 'error':
            showFn = showError;
            break;
    }

    if (e instanceof Error) {
        showFn(e.message);
    } else if (typeof e === "string") {
        showFn(e);
    } else {
        showFn("Unknown error");
    }
}
