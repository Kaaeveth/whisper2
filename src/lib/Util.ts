const UNIT_SUFFIX = ["Bytes", "KB", "MB", "GB"];

export function formatByteSize(size: number): string {
    let prefixCount = 0;
    while(size > 1000) {
        size = Math.trunc(size / 1000);
        if(++prefixCount >= UNIT_SUFFIX.length-1)
            break;
    }
    return `${size} ${UNIT_SUFFIX[prefixCount]}`;
}