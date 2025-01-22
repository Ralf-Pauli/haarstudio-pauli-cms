<script lang="ts">

    import {Button} from "@/components/ui/button";
    import {Trash2} from "lucide-svelte";
    import type {Entity} from "$utils/interfaces";
    import type {Row} from "@tanstack/table-core"

    let {row} : {row: Row<Entity>}  = $props();

    function onDelete(id: number) {

    }
</script>

<Button variant="ghost" onclick={() => onDelete(row.original.id)}>
    <Trash2/>
</Button>