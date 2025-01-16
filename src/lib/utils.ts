import {type ClassValue, clsx} from "clsx";
import {twMerge} from "tailwind-merge";
import {renderComponent} from "@/components/ui/data-table";
import EditableCell from "@/components/services/data-tables/controls/EditableCell.svelte";
import type {CellContext} from "@tanstack/table-core";
import {invoke} from "@tauri-apps/api/core";
import type {Category} from "$utils/interfaces";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export async function save(categories: Category[]): Promise<{ status: "ok" } | { status: "error"; message: string }> {
    try {
        await invoke("save_categories", {categories});
        console.log("Categories saved");
        return {status: "ok"};
    } catch (error) {
        console.error("Error saving categories:", error);
        console.log({categories})
        return {status: "error", message: String(error)};
    }
}


export function createEditableCell(row: any, fieldKey: keyof typeof row.original) {
    return renderComponent(EditableCell, {
        value: row.original[fieldKey],
        update: (newValue) => {
            row.original[fieldKey] = newValue;
        },
    });
}