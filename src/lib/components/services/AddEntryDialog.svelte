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
    import {z} from "zod";
    import {ENTRY_TYPES} from "$utils/interfaces";

    let {data = $bindable(), type, parentRow} = $props();

    let name: string = $state("");
    let price: number | null = $state(null);
    let open: boolean = $state(false);
    let showConfirmationDialog: boolean = $state(false);

    // Validation state
    let errors = $state({
        name: null as string | null,
        price: null as string | null,
    });

    // Shared Zod validation schema
    const schema = z.object({
        name: z.string().min(1, "Name is required"),
        price: type !== ENTRY_TYPES.CATEGORY ? z.number({
            invalid_type_error: "Price must be a number",
        }).positive("Price must be greater than 0") : z.any(),
    });

    function validateInput() {
        try {
            schema.parse({
                name,
                price: type !== ENTRY_TYPES.CATEGORY ? price : null,
            });
            errors = {name: null, price: null};
            return true;
        } catch (err) {
            if (err instanceof z.ZodError) {
                errors = err.errors.reduce(
                    (acc, error) => {
                        if (error.path[0] === "name") {
                            acc.name = error.message;
                        }
                        if (error.path[0] === "price") {
                            acc.price = error.message;
                        }
                        return acc;
                    },
                    {name: null, price: null}
                );
            }
            return false;
        }
    }

    function createNewEntry() {
        if (!validateInput()) return;

        debugger
        if (type === ENTRY_TYPES.SUB_SERVICE && parentRow.price !== null) {
            showConfirmationDialog = true;
            return;
        }

        updateData();
    }

    function cancelAddition() {
        showConfirmationDialog = false;
    }


    function updateData() {
        const getNextId = (items: any[]) => items.reduce((max, obj) => Math.max(max, obj.id), 0) + 1;

        if (type === ENTRY_TYPES.CATEGORY) {
            data = [
                ...data,
                {
                    id: getNextId(data),
                    name,
                    services: [],
                },
            ];
        } else if (type === ENTRY_TYPES.SERVICE) {
            parentRow.services = [
                ...parentRow.services,
                {
                    id: getNextId(parentRow.services),
                    name,
                    price,
                    sub_services: [],
                },
            ];
        } else if (type === ENTRY_TYPES.SUB_SERVICE) {
            parentRow.price = null;
            parentRow.sub_services = [
                ...parentRow.sub_services,
                {
                    id: getNextId(parentRow.sub_services),
                    name,
                    price,
                },
            ];
        }

        toast.success("Entry created");
        open = false;
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        <Plus/>
        Add Entry
    </Dialog.Trigger>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Add Entry</Dialog.Title>
        </Dialog.Header>
        <div class="grid gap-4 py-4 justify-items-center">
            <div class="grid grid-flow-row items-center grid-cols-4 gap-1.5">
                <Label for="name" class="col-span-1">Name</Label>
                <Input
                        autocomplete="off"
                        id="name"
                        bind:value={name}
                        placeholder="Name"
                        class="col-span-3 {errors.name ? 'focus-visible:ring-red-500' : ''}"
                />
                <p class="text-red-500 text-sm col-span-4 h-4 pl-2">
                    {errors.name || "\u00A0"} <!-- Keeps space consistent -->
                </p>
            </div>
            {#if type !== ENTRY_TYPES.CATEGORY}
                <div class="grid grid-flow-row items-center grid-cols-4 gap-1.5">
                    <Label for="price" class="col-span-1">Price</Label>
                    <Input
                            autocomplete="off"
                            id="price"
                            type="number"
                            bind:value={price}
                            placeholder="Price"
                            class="col-span-3 {errors.price ? 'focus-visible:ring-red-500' : ''}"
                    />
                    <p class="text-red-500 text-sm col-span-4 h-4 pl-2">
                        {errors.price || "\u00A0"} <!-- Keeps space consistent -->
                    </p>
                </div>
            {/if}
        </div>
        <Button variant="outline" onclick={createNewEntry}>Save changes</Button>
    </Dialog.Content>
</Dialog.Root>

<!-- Confirmation Modal -->
<Dialog.Root bind:open={showConfirmationDialog}>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Confirm Price Update</Dialog.Title>
        </Dialog.Header>
        <p class="text-sm">
            Setting a sub-service will reset the service's price to <strong>null</strong>. Do you want to continue?
        </p>
        <div class="flex justify-end mt-4 space-x-2">
            <Dialog.Close>
                <Button variant="outline" onclick={cancelAddition}>Cancel</Button>
            </Dialog.Close>
            <Dialog.Close>
                <Button variant="ghost" onclick={updateData}>Yes, Confirm</Button>
            </Dialog.Close>
        </div>
    </Dialog.Content>
</Dialog.Root>
