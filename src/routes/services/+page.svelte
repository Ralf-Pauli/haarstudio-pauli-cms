<script lang="ts">
    import Toolbar from "@/components/services/Toolbar.svelte";
    import {save} from "$utils";
    import type {Category} from "$utils/interfaces";
    import DataTable from "@/components/services/data-tables/DataTable.svelte";
    import {categoryColumns} from "@/components/services/data-tables/columns/category-columns";

    let { data, table } = $props();
    let categories: Category[] = $state(data.categories);

    async function saveCategories() {
        const result = await save(categories);
        console.log(result);
    }
</script>

<div>
    <Toolbar onSave={saveCategories}/>
    <DataTable bind:data={categories} columns={categoryColumns} bind:table type="category" />
</div>