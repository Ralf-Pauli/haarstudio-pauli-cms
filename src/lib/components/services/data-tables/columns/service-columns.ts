import type {CellContext, ColumnDef} from "@tanstack/table-core";
import type {Service} from "$utils/interfaces";
import {renderComponent} from "@/components/ui/data-table";
import DataTableActions from "@/components/services/data-tables/controls/DataTableActions.svelte";
import DataTableNameButton from "@/components/services/data-tables/controls/DataTableNameButton.svelte";
import {Checkbox} from "@/components/ui/checkbox";
import CollapseButton from "@/components/services/data-tables/controls/CollapseButton.svelte";
import EditableCell from "@/components/services/data-tables/controls/EditableCell.svelte";
import {createEditableCell} from "$utils";

export const serviceColumns: ColumnDef<Service>[] = [
    {
        id: "select",
        header: ({table}) =>
            renderComponent(Checkbox, {
                checked: table.getIsAllPageRowsSelected(),
                indeterminate:
                    table.getIsSomePageRowsSelected() &&
                    !table.getIsAllPageRowsSelected(),
                onCheckedChange: (value) => table.toggleAllPageRowsSelected(!!value),
                "aria-label": "Select all",
            }),
        cell: ({row}) =>
            renderComponent(Checkbox, {
                checked: row.getIsSelected(),
                onCheckedChange: (value) => row.toggleSelected(!!value),
                "aria-label": "Select row",
            }),
        enableSorting: false,
        enableHiding: false,
    },
    {
        accessorKey: "id",
        header: "Id"
    },
    {
        accessorKey: "name",
        header: ({column}) =>
            renderComponent(DataTableNameButton, {
                onclick: () => column.toggleSorting(column.getIsSorted() === "asc"),
            }),
        cell: ({row}) => {
            return createEditableCell(row, "name");
        }
    },
    {
        accessorKey: "price",
        header: "Price",
        cell: ({row}) => {
            if (row.original.sub_services && row.original.sub_services.length > 0) {
                return row.original.price;
            }
            return createEditableCell(row, "price");

        }
    },
    {
        id: "actions",
        cell: ({row}) => {
            return renderComponent(DataTableActions, {row: row});
        }
    },
]