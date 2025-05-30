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