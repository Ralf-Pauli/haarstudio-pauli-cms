import {invoke} from "@tauri-apps/api/core";
import type {PageLoad} from "./$types";
import type {Category} from "$utils/interfaces";

export const load: PageLoad = async () => {
    const categories = await invoke<Category[]>("load_categories");
    return { categories };
};