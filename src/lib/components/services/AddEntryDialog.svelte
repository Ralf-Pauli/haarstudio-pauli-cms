<script lang="ts">
    import {
        Button,
        buttonVariants
    } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import {Input} from "$lib/components/ui/input/index";
    import {Label} from "@/components/ui/label";
    import {Plus} from "lucide-svelte";
    import {toast} from "svelte-sonner";


    let {data = $bindable(), type, parentRow} = $props();

    let name = $state();
    let price = $state();

    function createNewEntry() {
        if (type === "category") {
            const highestId = data.reduce((max, obj) => Math.max(max, obj.id), 0);

            data = [
                ...data,
                {
                    id: highestId + 1,
                    name: name,
                    services: []
                }
            ];
        } else if (type === "service") {
            const highestId = parentRow.services.reduce((max, obj) => Math.max(max, obj.id), 0);

            parentRow.services = [
                ...parentRow.services,
                {
                        id: highestId + 1,
                        name: name,
                        price: price,
                }
            ]
        } else if (type === "sub_service") {
            const highestId = parentRow.sub_services.reduce((max, obj) => Math.max(max, obj.id), 0);

            parentRow.sub_services = [
                ...parentRow.sub_services,
                {
                        id: highestId + 1,
                        name: name,
                        price: price,
                }
            ]
        }

        toast.success("Entry created");
    }

    function addEntry(arr: []) {

    }

</script>

<Dialog.Root>
    <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        <Plus/>
        Add Entry
    </Dialog.Trigger>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Add Entry</Dialog.Title>
        </Dialog.Header>
        <div class="grid gap-4 py-4">
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="name" class="text-right">Name</Label>
                <Input id="name" bind:value={name} placeholder="Name" class="col-span-3"/>
            </div>
            {#if type !== "category"}
                <div class="grid grid-cols-4 items-center gap-4">
                    <Label for="price" class="text-right">Price</Label>
                    <Input id="price" bind:value={price} placeholder="Price" class="col-span-3"/>
                </div>
            {/if}
        </div>
        <Dialog.Close>
            <Button variant="outline" onclick={createNewEntry}>Save changes</Button>
        </Dialog.Close>
    </Dialog.Content>
</Dialog.Root>