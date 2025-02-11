<script lang="ts">
    import Toolbar from "@/components/services/Toolbar.svelte";
    import {saveCategories} from "$utils";
    import type {Category} from "$utils/interfaces";
    import DataTable from "@/components/services/data-tables/DataTable.svelte";
    import {categoryColumns} from "@/components/services/data-tables/columns/category-columns";

    let {data, table} = $props();
    let categories: Category[] = $state(data.categories);
</script>

<div class="flex flex-col gap-8">
    <div class="text-xl font-bold">
        <h1 class="inline border-b-2 border-b-primary">Services</h1>
    </div>
    <div>
        <Toolbar onSave={() => saveCategories(categories)} config={{save: true, resetAll:true}}/>
        <DataTable bind:data={categories} columns={categoryColumns} bind:table type="category"/>
    </div>
</div>