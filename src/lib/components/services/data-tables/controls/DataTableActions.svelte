<script lang="ts">
    import Ellipsis from "lucide-svelte/icons/ellipsis";
    import { Button } from "@/components/ui/button";
    import * as DropdownMenu from "@/components/ui/dropdown-menu";

    let { id }: { id: number } = $props();
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger>
        {#snippet child({ props })}
            <Button
                    {...props}
                    variant="ghost"
                    size="icon"
                    class="relative size-8 p-0"
            >
                <span class="sr-only">Open menu</span>
                <Ellipsis class="size-4" />
            </Button>
        {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
        <DropdownMenu.Item onclick={() => console.log("Delete")}>Delete Entry</DropdownMenu.Item>
        <DropdownMenu.Item onclick={() => console.log("Add")}>Add Service</DropdownMenu.Item>
    </DropdownMenu.Content>
</DropdownMenu.Root>