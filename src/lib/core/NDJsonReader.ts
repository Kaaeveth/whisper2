import Uint8Buffer from "./UInt8Buffer";

/**
 * Deserializes a stream of newline delimited JSON
 * @param body A stream containing newline delimited JSON objects
 */
export default async function* readNdJson<T>(body: ReadableStream<Uint8Array>): AsyncIterable<T> {
    // NDJSON is always delimited by U+000A (newline)
    const DELIMITER = 0xA;
    const buffer = new Uint8Buffer();
    const decoder = new TextDecoder();
    const parseFromBuffer = (idx?: number) => JSON.parse(decoder.decode(buffer.getView(0, idx)));

    const reader = body.getReader();
    try {
        while(true) {
            const {value, done} = await reader.read();
            if(done && buffer.length > 0) {
                yield parseFromBuffer();
                break;
            }

            buffer.append(value!);
            const delimiterIdx = buffer.getView().findIndex((v, _, __) => v == DELIMITER);
            if(delimiterIdx < 0) continue;

            yield parseFromBuffer(delimiterIdx);
            buffer.remove(0, delimiterIdx+1);
        }
    } finally {
        if(!body.locked)
            body.cancel();
    }
}
