<script lang="ts">
    import {Toolbar} from "bits-ui";
    import {Separator} from "@/components/ui/separator";
    import {Plus, Redo, RotateCcw, Save, Trash2, Undo} from "lucide-svelte";

    let props = $props();
    const onSave = props.onSave ?? (async () => ({status: "ok"}));
    const config = props.config ?? {};
</script>

<Toolbar.Root class="flex h-12 min-w-max items-center rounded-md border border-border px-[4px] shadow-mini">
    <div class="flex items-center">
        <!-- Save -->
        {#if config.save}
            <Toolbar.Button
                    onclick={onSave}
                    class="inline-flex items-center justify-center rounded-md px-3 py-2 text-sm font-medium text-foreground/80 transition-all hover:bg-muted active:scale-98 active:bg-dark-10">
                <Save class="mr-2 size-6"/>
                <span>Save</span>
            </Toolbar.Button>
        {/if}

        <!-- Rest All -->
        {#if config.resetAll}
            <Toolbar.Button
                    onclick={onSave}
                    class="inline-flex items-center justify-center rounded-md px-3 py-2 text-sm  font-medium text-foreground/80 transition-all hover:bg-muted active:scale-98 active:bg-dark-10">
                <Trash2 class="mr-2 size-6"/>
                <span>Rest All</span>
            </Toolbar.Button>
        {/if}

    </div>
</Toolbar.Root>