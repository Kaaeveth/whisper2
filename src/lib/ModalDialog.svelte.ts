import type { ButtonColor } from "flowbite-svelte";
import type { Snippet } from "svelte";

export interface ShowModalOptions {
    confirmText: string;
    confirmColor?: ButtonColor;
    abortText: string;
    title: string;
    content?: Snippet;
}

interface ModalDialogState extends ShowModalOptions {
    modalOpen: boolean;
}

/**
 * Global Popup for user confirmation
 */
export default class ModalDialog {
    private static instance?: ModalDialog;

    public static get(): ModalDialog {
        if(!ModalDialog.instance) {
            ModalDialog.instance = new ModalDialog();
        }
        return ModalDialog.instance;
    }

    // Current state and content of the popup
    private _comp: ModalDialogState = $state({
        confirmText: "Confirm",
        abortText: "Abort",
        title: "Confirm",
        modalOpen: false
    });

    // Current promise resolving when the popup closes
    private _currentModalPrompt?: Promise<boolean>;
    private _resolve?: (confirm: boolean) => void;
    private _reject?: () => void;

    get componentState() {
        return this._comp;
    }

    /**
     * Closes an open modal.
     * Does nothing of the modal is open.
     * @param ok Answer of the user (yes/no)
     */
    closeModal(ok: boolean) {
        this._comp.modalOpen = false;
        this._currentModalPrompt = undefined;
        if(this._resolve)
            this._resolve(ok)
        else if (this._reject)
            this._reject();
        this._reject = this._resolve = undefined;
    }

    /**
     * Shows a popup for binary user confirmation.
     * @param options Popup content options
     * @returns A promise resolving when the modal closes
     */
    async showModal(options: ShowModalOptions): Promise<boolean> {
        if(this._currentModalPrompt) {
            if(this._comp.modalOpen)
                await this._currentModalPrompt;
            else
                this.closeModal(false);
        }

        Object.assign(this._comp, options);
        // We need to set optional values explicitly
        // since Object.assign doesn't set undefined values,
        // which may be needed to clean previous state.
        this._comp.confirmColor = options.confirmColor;
        this._comp.content = options.content;

        this._currentModalPrompt = new Promise((res, rej) => {
            this._resolve = res;
            this._reject = rej;
            this._comp.modalOpen = true;
        });
        return this._currentModalPrompt;
    }
}