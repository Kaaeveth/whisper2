class Uint8Buffer {
  private innerBuffer: ArrayBuffer;
  private buffer: Uint8Array;
  private _length: number;

  constructor(initialSize: number = 1024, maxSize: number = 65536) {
    this.innerBuffer = new ArrayBuffer(initialSize, {
      maxByteLength: maxSize
    });
    this.buffer = new Uint8Array(this.innerBuffer);
    this._length = 0;
  }

  append(data: Uint8Array) {
    // Need to expand buffer
    if (this._length + data.length > this.innerBuffer.byteLength) {
      const newSize = Math.max(this.innerBuffer.byteLength * 2, this._length + data.length);
      this.innerBuffer.resize(newSize);
    }
    this.buffer.set(data, this._length);
    this._length += data.length;
  }

  /**
   * Drops the data between start and end indezes.
   * The data on the right side of the removed data will be 
   * moved left to fill up the removed space.
   * The length of the buffer will be reduced accordingly.
   * However, the capacity will remain the same.
   * @param start start index of the data to be removed or 0, if not given
   * @param end end index of the data to be removed or the end of the buffer, if not given
   */
  remove(start?: number, end?: number) {
    this.buffer.set(this.buffer.subarray(end), start);
    this._length -= (end ?? this._length) - (start ?? 0);
    this.buffer.fill(0, this._length);
  }

  /**
   * Returns a view of the buffer.
   * @param start optional start index (inclusive) or 0 if not given
   * @param end optional end index (exclusive) or the end of buffer if not given
   * @returns View of the buffer
   */
  getView(start?: number, end?: number): Uint8Array {
    return this.buffer.subarray(start ?? 0, end ?? this._length);
  }

  get length(): number {
    return this._length;
  }
}
