<script lang="ts">
    import {Button} from "@/components/ui/button";
    import {type Category, type Entity, ENTRY_TYPES, type Service} from "$utils/interfaces";
    import {Trash2} from "lucide-svelte";
    import {saveCategories} from "$utils";
    import {toast} from "svelte-sonner";

    let {data = $bindable(), tableData = $bindable(), entries, type} = $props();

    function deleteEntries() {
        if (entries.length === 0) {
            toast.error("No entries selected");
            return
        }

        for (const entry of entries) {
            const originalEntry = entry.original;

            if (type === ENTRY_TYPES.CATEGORY) {
                data = data.filter((entity: Category) => entity.id !== originalEntry.id);
            } else if (type === ENTRY_TYPES.SERVICE) {
                // Remove the service from `data` (services of the current category)
                data = data.filter((entity: Service) => entity.id !== originalEntry.id);

                // Update the parent category in `tableData`
                tableData = tableData.map((category: Category) => {
                    category.services = category.services.filter((service) => service.id !== originalEntry.id);
                    return category;
                });
            } else if (type === ENTRY_TYPES.SUB_SERVICE) {
                // Remove the sub-service from `data` (subservices of the current service)
                data = data.filter((entity: Entity) => entity.id !== originalEntry.id);

                // Update the parent service in `tableData`
                tableData = tableData.map((category: Category) => {
                    category.services = category.services.map((service: Service) => {
                        service.sub_services = service.sub_services.filter((subService) => subService.id !== originalEntry.id);
                        return service;
                    });
                    return category;
                });
            }
        }

        saveCategories(tableData || data);
        toast.success("Entries deleted");
    }
</script>

<Button variant="outline" onclick={deleteEntries}>
    <Trash2/>
    Delete Entry
</Button>