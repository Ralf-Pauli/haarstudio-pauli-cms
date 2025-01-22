<script lang="ts">
    import type {Row} from "@tanstack/table-core"
    import CollapseButton from "@/components/services/data-tables/controls/CollapseButton.svelte";
    import DeleteButton from "@/components/services/data-tables/controls/DeleteButton.svelte";
    import type {Entity} from "$utils/interfaces";

    let { row }= $props();
</script>

<div>
    <CollapseButton {row} />
</div>