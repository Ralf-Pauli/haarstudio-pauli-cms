<script lang="ts">
    import {Input} from "@/components/ui/input";
    import {Label} from "@/components/ui/label";
    import Toolbar from "@/components/services/Toolbar.svelte";
    import {saveContact} from "$utils";
    import {Button} from "@/components/ui/button";
    import {Trash2} from "lucide-svelte";
    import AddContactInformationDialog from "@/components/contact/AddContactInformationDialog.svelte";
    import OpeningHour from "@/components/contact/OpeningHours.svelte";
    import {formatKey} from "$utils";
    import SocialMediaEntries from "@/components/contact/SocialMediaEntries.svelte";


    const {data} = $props();
    let contact = $state(data.contact);


    function deleteContactInformation(key: string) {
        delete contact.contact_information[key];
    }

</script>


<div class="flex flex-col gap-8">
    <div class="text-xl font-bold">
        <h1 class="inline border-b-2 border-b-primary">Contact</h1>
    </div>

    <Toolbar config={{save: true, resetAll:false}} onSave={() => saveContact(contact)}/>

    <div class="flex flex-col gap-8 pl-2 w-3/5">
        <div class="flex flex-col gap-5 pb-2">
            <div class="text-lg font-bold">
                <h1 class="inline border-b-2 border-b-primary">Address</h1>
            </div>

            <div class="flex flex-col gap-5 pl-2">
                {#each Object.entries(contact.address) as [key, value]}
                    <div class="flex flex-row items-center gap-5">
                        <Label for={key} class="min-w-[100px] text-left">{formatKey(key)}</Label>
                        <Input id={key} type="text" class="flex-1" bind:value={contact.address[key]}/>
                    </div>
                {/each}
            </div>
        </div>

        <div class="flex flex-col gap-5 pb-2">
            <div class="text-lg font-bold">
                <h1 class="inline border-b-2 border-b-primary">Contact Information</h1>
            </div>

            <div class="flex flex-col gap-5 pl-2">
                {#each Object.keys(contact.contact_information) as key}
                    <div class="flex flex-row items-center gap-5">
                        <Label for={key} class="min-w-[100px] text-left">{formatKey(key)}</Label>
                        <Input id={key} type="text" class="flex-1" bind:value={contact.contact_information[key]}/>
                        <Button variant="outline" onclick={() => deleteContactInformation(key)}>
                            <Trash2/>
                        </Button>
                    </div>
                {/each}
            </div>

            <AddContactInformationDialog bind:contact={contact}/>
        </div>

        <OpeningHour bind:contact={contact}/>

        <SocialMediaEntries bind:contact={contact}/>
    </div>
</div>


