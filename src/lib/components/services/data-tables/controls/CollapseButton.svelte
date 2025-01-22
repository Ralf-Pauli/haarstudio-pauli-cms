<script lang="ts">
    import {Button} from "@/components/ui/button";
    import {ChevronDown, ChevronUp} from "lucide-svelte";
    import type {Category} from "\$utils/interfaces";

    let {row} = $props();
</script>

<Button variant="ghost" class="h-8 w-8 p-0" onclick={() => row.toggleExpanded()}>
    {#if row.getIsExpanded()}
        <ChevronUp/>
    {:else}
        <ChevronDown/>
    {/if}
</Button>