<script lang="ts" module>
    import { info, warn, error } from '@tauri-apps/plugin-log';
    import { CloseOutline } from 'flowbite-svelte-icons';
    import { fly } from 'svelte/transition';

    export interface Notification {
        content: string;
        level: "info"|"warn"|"error";
        duration: number // Visibility duration in ms
    }
    interface KeyedNotification extends Notification {
        key: number;
    }

    const MSG_COLORS: {
        [level in Notification["level"]]: {
            container: string;
            progressbar: string;
        }
    } = {
        "info": {
            container: "bg-blue-100 border-blue-400 text-blue-500",
            progressbar: "bg-blue-600"
        },
        "warn": {
            container: "bg-orange-100 border-orange-400 text-orange-500",
            progressbar: "br-orange-600"
        },
        "error": {
            container: "bg-red-200 border-red-400 text-red-500",
            progressbar: "bg-red-600"
        }
    };
    const MAX_VISIBLE_NOTIFICATIONS = 3;
    const MAX_NOTIFICATIONS = 16;

    const notifications: Array<KeyedNotification|undefined> = $state(new Array(MAX_NOTIFICATIONS));
    notifications.fill(undefined, 0, MAX_NOTIFICATIONS);
    const displayingNotifications: KeyedNotification[] = $derived(
      (notifications.filter(e => e) as KeyedNotification[]).slice(0, MAX_VISIBLE_NOTIFICATIONS).reverse()
    );
    let nextNotificationIdx = 0;

    export function show(msg: Notification) {
        if (msg.duration < 1) {
            console.error("Snackbar duration must be >= 1");
            return;
        }
        if (nextNotificationIdx >= MAX_NOTIFICATIONS) {
            console.warn("Too many notifications queued - Dropping");
            return;
        }

        const notificationIdx = nextNotificationIdx++;
        notifications[notificationIdx] = {...msg, key: notificationIdx};
    }

    function closeNotification(id: number) {
        notifications[id] = undefined;
        if (id < nextNotificationIdx) {
            nextNotificationIdx = id;
        }
    }

    function onNotificationMounted(elem: HTMLElement, id: KeyedNotification) {
        setTimeout(() => closeNotification(id.key), id.duration);
    }

    export function showInfo(msg: string) {
        show({
            content: msg,
            level: "info",
            duration: 3000
        });
        info(msg);
    }

    export function showWarning(msg: string) {
        show({
            content: msg,
            level: "warn",
            duration: 4000
        });
        warn(msg);
    }

    export function showError(msg: string) {
        show({
            content: msg,
            level: "error",
            duration: 4000
        });
        error(msg);
    }
</script>

{#snippet snackbarMsg(msg: KeyedNotification)}
    {@const colors = MSG_COLORS[msg.level]}
    <div
        transition:fly={{duration: 300, x: 500, opacity: 100}}
        class={`mb-2 max-h-24 rounded-lg border-2 ${colors.container}`}
        use:onNotificationMounted={msg}
    >
        <div class="flex justify-between p-4">
            <p class="wrap-anywhere overflow-ellipsis">{msg.content}</p>
            <CloseOutline size="md" class="my-auto cursor-pointer" onclick={() => closeNotification(msg.key)}></CloseOutline>
        </div>
        <div style={`animation: progressbar ${msg.duration}ms linear forwards;`} class={`w-full h-1 ${colors.progressbar}`}></div>
    </div>
{/snippet}

<div class="absolute right-4 top-4 w-96 h-0 overflow-y-visible">
    {#each displayingNotifications as msg (msg.key)}
        {@render snackbarMsg(msg)}
    {/each}
</div>
