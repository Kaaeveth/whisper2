export default class ToggableElement {
    private _open = $state(true);

    constructor(open: boolean) {
        this._open = open;
    }

    public set open(open: boolean) {this._open = open;}
    public get open() {return this._open;}

    public toggle() {
        this.open = !this.open;
    }
}
