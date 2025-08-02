<script lang="ts" module>
    import { fly } from 'svelte/transition';

    export interface Notification {
        content: string;
        level: "info"|"warn"|"error";
        duration: number // Visibility duration in ms
    }
    interface KeyedNotification extends Notification {
        key: number;
    }

    const MSG_COLORS: {[level in Notification["level"]]: string} = {
        "info": "bg-blue-100 border-blue-400 text-blue-500",
        "warn": "bg-orange-100 border-orange-400 text-orange-500",
        "error": "bg-red-200 border-red-400 text-red-500"
    };
    const MAX_VISIBLE_NOTIFICATIONS = 3;
    const MAX_NOTIFICATIONS = 16;

    const notifications: Array<KeyedNotification|undefined> = $state(new Array(MAX_NOTIFICATIONS));
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
        setTimeout(() => {
            notifications[notificationIdx] = undefined;
            if (notificationIdx < nextNotificationIdx) {
                nextNotificationIdx = notificationIdx;
            }
        }, msg.duration);
    }

    export function showInfo(msg: string) {
        show({
            content: msg,
            level: "info",
            duration: 4000
        });
    }

    export function showWarning(msg: string) {
        show({
            content: msg,
            level: "warn",
            duration: 5000
        });
        console.warn(msg);
    }

    export function showError(msg: string) {
        show({
            content: msg,
            level: "error",
            duration: 6000
        });
        console.error(msg);
    }
</script>

{#snippet snackbarMsg(msg: Notification)}
    <div
        transition:fly={{duration: 500, x: 500, opacity: 100}}
        class={`p-4 mb-2 max-h-24 rounded-lg border-2 ${MSG_COLORS[msg.level]}`}
    >
        <p class="wrap-anywhere overflow-ellipsis">{msg.content}</p>
    </div>
{/snippet}

<div class="absolute z-50 right-4 top-4 w-96 h-96">
    {#each displayingNotifications as msg (msg.key)}
        {@render snackbarMsg(msg)}
    {/each}
</div>
