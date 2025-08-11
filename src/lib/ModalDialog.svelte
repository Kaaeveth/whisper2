<script lang="ts" module>
    import { Button, Heading, Modal, type ButtonColor } from "flowbite-svelte";
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

    // Current state and content of the popup
    let popupState: ModalDialogState = $state({
        confirmText: "Confirm",
        abortText: "Abort",
        title: "Confirm",
        modalOpen: false
    });

    // Current promise resolving when the popup closes
    let currentModalPrompt: Promise<boolean> | undefined = undefined;
    let resolvePopupClose: ((confirm: boolean) => void) | undefined = undefined;
    let rejectPopupClose: (() => void) | undefined = undefined;

    /**
     * Closes an open modal.
     * Does nothing of the modal is open.
     * @param ok Answer of the user (yes/no)
     */
    export function closeModal(ok: boolean) {
        popupState.modalOpen = false;
        currentModalPrompt = undefined;
        if(resolvePopupClose)
            resolvePopupClose(ok)
        else if (rejectPopupClose)
            rejectPopupClose();
        rejectPopupClose = rejectPopupClose = undefined;
    }

    /**
     * Shows a popup for binary user confirmation.
     * @param options Popup content options
     * @returns A promise resolving when the modal closes
     */
    export async function showModal(options: ShowModalOptions): Promise<boolean> {
        if(currentModalPrompt) {
            if(popupState.modalOpen)
                await currentModalPrompt;
            else
                closeModal(false);
        }

        Object.assign(popupState, options);
        // We need to set optional values explicitly
        // since Object.assign doesn't set undefined values,
        // which may be needed to clean previous state.
        popupState.confirmColor = options.confirmColor;
        popupState.content = options.content;

        currentModalPrompt = new Promise((res, rej) => {
            resolvePopupClose = res;
            rejectPopupClose = rej;
            popupState.modalOpen = true;
        });
        return currentModalPrompt;
    }
</script>

<Modal bind:open={popupState.modalOpen}>
    <Heading tag="h3">{popupState.title}</Heading>
    {@render popupState.content?.()}

    {#snippet footer()}
        <Button
            color={popupState.confirmColor ?? "primary"}
            onclick={() => closeModal(true)}>
            {popupState.confirmText}
        </Button>
        <Button color="alternative" onclick={() => closeModal(false)}>
            {popupState.abortText}
        </Button>
    {/snippet}
</Modal>
