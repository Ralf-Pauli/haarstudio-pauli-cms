<script lang="ts" generics="TData, TValue">
    import {
        type ColumnDef,
        type SortingState,
        type ColumnFiltersState,
        type VisibilityState,
        type RowSelectionState,
        type ExpandedState,
        type TableOptions,
        getCoreRowModel,
        getSortedRowModel,
        getFilteredRowModel,
    } from "@tanstack/table-core";
    import {
        createSvelteTable,
        FlexRender,
    } from "@/components/ui/data-table";
    import * as Table from "@/components/ui/table";
    import * as DropdownMenu from "@/components/ui/dropdown-menu";
    import {Input} from "@/components/ui/input";
    import {Button} from "@/components/ui/button";
    import {serviceColumns} from "@/components/services/data-tables/columns/service-columns"
    import {subServiceColumns} from "@/components/services/data-tables/columns/sub-service-columns"
    import DataTable from "./DataTable.svelte"
    import {Plus} from "lucide-svelte";
    import AddEntryDialog from "@/components/services/AddEntryDialog.svelte";
    import {get, writable} from "svelte/store";
    import DeleteEntryButton from "@/components/services/DeleteEntryButton.svelte";


    type DataTableProps<TData, TValue> = {
        columns: ColumnDef<TData, TValue>[];
        data: TData[];
    };

    let {data = $bindable(), tableData = $bindable(), columns, table = $bindable(), type, parentRow} = $props();

    let sorting = $state<SortingState>([]);
    let columnFilters = $state<ColumnFiltersState>([]);
    let columnVisibility = $state<VisibilityState>({});
    let rowSelection = $state<RowSelectionState>({});
    let expanded = $state<ExpandedState>({});

    const options = writable<TableOptions<unknown>>({
        get data() {
            return data;
        },
        columns,
        getCoreRowModel: getCoreRowModel(),
        getSortedRowModel: getSortedRowModel(),
        getFilteredRowModel: getFilteredRowModel(),
        getSubRows: (row) => {
            if (!row.services && !row.sub_services) {
                return [];
            } else if (row.sub_services) {
                return row.sub_services
            } else {
                return row.services;
            }
        },
        onSortingChange: (updater) => {
            if (typeof updater === "function") {
                sorting = updater(sorting);
            } else {
                sorting = updater;
            }
        },
        onColumnFiltersChange: (updater) => {
            if (typeof updater === "function") {
                columnFilters = updater(columnFilters);
            } else {
                columnFilters = updater;
            }
        },
        onColumnVisibilityChange: (updater) => {
            if (typeof updater === "function") {
                columnVisibility = updater(columnVisibility);
            } else {
                columnVisibility = updater;
            }
        },
        onRowSelectionChange: (updater) => {
            if (typeof updater === "function") {
                rowSelection = updater(rowSelection);
            } else {
                rowSelection = updater;
            }
        },
        onExpandedChange: (updater) => {
            if (typeof updater === "function") {
                expanded = updater(expanded);
            } else {
                expanded = updater;
            }
        },
        state: {
            get sorting() {
                return sorting;
            },
            get columnFilters() {
                return columnFilters;
            },
            get columnVisibility() {
                return columnVisibility;
            },
            get rowSelection() {
                return rowSelection;
            },
            get expanded() {
                return expanded;
            },
        }
    });

    table = createSvelteTable(get(options));
</script>

<div class="mt-3">
    <div class="flex items-center gap-4 py-4">
        <Input
                placeholder="Filter names..."
                class="max-w-sm"
                value={(table.getColumn("name")?.getFilterValue() as string) ?? ""}
                onchange={(e) => {
                    table.getColumn("name")?.setFilterValue(e.currentTarget.value);
                }}
                oninput={(e) => {
                    table.getColumn("name")?.setFilterValue(e.currentTarget.value);
                }}
        />

        <AddEntryDialog bind:data {type} {parentRow} />
        <DeleteEntryButton bind:data entries={table.getSelectedRowModel().rows} {type} {tableData} />

        <DropdownMenu.Root>
            <DropdownMenu.Trigger>
                {#snippet child({props})}
                    <Button {...props} variant="outline" class="ml-auto">Columns</Button>
                {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content align="end">
                {#each table
                    .getAllColumns()
                    .filter((col) => col.getCanHide()) as column (column.id)}
                    <DropdownMenu.CheckboxItem
                            class="capitalize"
                            bind:checked={() => column.getIsVisible(), (v) => column.toggleVisibility(!!v)}
                    >
                        {column.id}
                    </DropdownMenu.CheckboxItem>
                {/each}
            </DropdownMenu.Content>
        </DropdownMenu.Root>
    </div>
    <div class="rounded-md border">
        <Table.Root>
            <Table.Header>
                {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
                    <Table.Row>
                        {#each headerGroup.headers as header (header.id)}
                            <Table.Head>
                                {#if !header.isPlaceholder}
                                    <FlexRender
                                            content={header.column.columnDef.header}
                                            context={header.getContext()}
                                    />
                                {/if}
                            </Table.Head>
                        {/each}
                    </Table.Row>
                {/each}
            </Table.Header>
            <Table.Body>
                {#each table.getRowModel().rows as row (row.id)}
                    <Table.Row data-state={row.getIsSelected() && "selected"}>
                        {#each row.getVisibleCells() as cell (cell.id)}
                            <Table.Cell>
                                <FlexRender
                                        content={cell.column.columnDef.cell}
                                        context={cell.getContext()}
                                />
                            </Table.Cell>
                        {/each}
                    </Table.Row>
                    {#if row.getIsExpanded() && row.original.services !== undefined}
                        <Table.Row>
                            <Table.Cell colspan={row.getAllCells().length}>
                                <div class="px-5">
                                    <DataTable data={row.original.services} columns={serviceColumns} type="service" parentRow={row.original} tableData={data} />
                                </div>
                            </Table.Cell>
                        </Table.Row>
                    {:else if row.getIsExpanded() && row.original.sub_services}
                        <Table.Row>
                            <Table.Cell colspan={row.getAllCells().length}>
                                <div class="px-5">
                                    <DataTable data={row.original.sub_services} columns={subServiceColumns} type="sub_service" parentRow={row.original} {tableData} />
                                </div>
                            </Table.Cell>
                        </Table.Row>
                    {/if}
                {:else}
                    <Table.Row>
                        <Table.Cell colspan={columns.length} class="h-24 text-center">
                            No results.
                        </Table.Cell>
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </div>


    <div class="text-muted-foreground flex-1 text-sm pt-3">
        {table.getFilteredSelectedRowModel().rows.length} of{" "}
        {table.getFilteredRowModel().rows.length} row(s) selected.
    </div>
</div>