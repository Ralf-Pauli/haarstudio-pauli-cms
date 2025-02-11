import {type ClassValue, clsx} from "clsx";
import {twMerge} from "tailwind-merge";
import {renderComponent} from "@/components/ui/data-table";
import EditableCell from "@/components/services/data-tables/controls/EditableCell.svelte";
import type {CellContext} from "@tanstack/table-core";
import {invoke} from "@tauri-apps/api/core";
import type {Category, Contact} from "$utils/interfaces";
import {toast} from "svelte-sonner";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

export function createEditableCell(row: any, fieldKey: keyof typeof row.original) {
    return renderComponent(EditableCell, {
        value: row.original[fieldKey],
        update: (newValue) => {
            row.original[fieldKey] = newValue;
        },
    });
}

async function invokeTauriMethod(
    method: string,
    payloadKey: string,
    payloadValue: Object
): Promise<{ status: "ok" } | { status: "error"; message: string }> {
    try {
        await invoke(method, { [payloadKey]: payloadValue });
        return { status: "ok" };
    } catch (error) {
        return { status: "error", message: String(error) };
    }
}

export async function saveCategories(categories: Category[]) {
    const result = await invokeTauriMethod("save_categories", "categories", categories);
    toast.success("Saved categories");
}

export async function saveContact(contact: Contact) {
    console.log('Contact being saved:', JSON.stringify(contact, null, 2));

    const result = await invoke('save_contact', { contact });
}

export function formatKey(key: string) {
    return key.charAt(0).toUpperCase() + key.slice(1).replaceAll('_', ' ');
}
