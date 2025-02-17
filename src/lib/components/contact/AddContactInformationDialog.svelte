<script lang="ts">
    import {
        Button,
        buttonVariants
    } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import {Input} from "$lib/components/ui/input/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import {Plus} from "lucide-svelte";

    let {contact = $bindable()} = $props();

    let informationName = $state(""), informationValue = $state("");

    function addContactInformation() {
        if (!informationName.trim() || !informationValue.trim()) {
            return alert('Please fill out both name and value fields.');
        }

        const updatedContact = { ...contact };
        updatedContact.contact_information = {
            ...updatedContact.contact_information,
            [informationName.replaceAll(' ', '_')]: informationValue
        };
        contact = updatedContact;

    }
</script>
<Dialog.Root>
    <Dialog.Trigger class="{buttonVariants({ variant: 'outline' })} w-fit">
        <Plus/>
        Add Contact Information
    </Dialog.Trigger>
    <Dialog.Content class="sm:max-w-[425px]">
        <Dialog.Header>
            <Dialog.Title>Add Contact Information</Dialog.Title>
        </Dialog.Header>
        <div class="grid gap-4 py-4">
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="name" class="text-right">Name</Label>
                <Input id="name" bind:value={informationName} class="col-span-3"/>
            </div>
            <div class="grid grid-cols-4 items-center gap-4">
                <Label for="value" class="text-right">Value</Label>
                <Input id="value" bind:value={informationValue} class="col-span-3"/>
            </div>
        </div>
        <Dialog.Close>
            <Button onclick={addContactInformation} variant="outline">Save changes</Button>
        </Dialog.Close>
    </Dialog.Content>
</Dialog.Root>