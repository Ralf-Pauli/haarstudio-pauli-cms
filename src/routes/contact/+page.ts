import type {PageLoad} from "../../../.svelte-kit/types/src/routes/services/$types";
import {invoke} from "@tauri-apps/api/core";
import type {Contact} from "$utils/interfaces";


export const load: PageLoad = async () => {
    const contact = await invoke<Contact>("load_contact");
    console.log(contact);
    return { contact };
};
