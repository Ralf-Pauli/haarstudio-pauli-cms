<script lang="ts">
    import Toolbar from "@/components/services/Toolbar.svelte";
    import {save, saveCategories} from "$utils";
    import type {Category} from "$utils/interfaces";
    import DataTable from "@/components/services/data-tables/DataTable.svelte";
    import {categoryColumns} from "@/components/services/data-tables/columns/category-columns";

    let { data, table } = $props();
    let categories: Category[] = $state(data.categories);
</script>

<div>
    <Toolbar onSave={() => saveCategories(categories)}/>
    <DataTable bind:data={categories} columns={categoryColumns} bind:table type="category" />
</div>