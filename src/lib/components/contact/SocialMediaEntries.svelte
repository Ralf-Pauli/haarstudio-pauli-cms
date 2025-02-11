<script lang="ts">
    import {Label} from "@/components/ui/label";
    import {formatKey} from "$utils";
    import {Input} from "@/components/ui/input";
    import {Separator} from "@/components/ui/separator";
    import {Plus, Trash2} from "lucide-svelte";
    import {Button} from "@/components/ui/button";
    import {z} from "zod";
    import {Checkbox} from "@/components/ui/checkbox";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type {ImageDetails} from "$utils/interfaces";


    let {contact = $bindable()} = $props();

    let showDialog = $state(false);
    let name = $state("");
    let url = $state("");
    let files: FileList | undefined = $state(undefined);
    // let icon: ImageDetails | undefined = $state();

    let icon = $derived.by(() => {
        const file = files?.[0];
        if (file) {
            return  {
                name: file.name,
                alternative_text: file.name,
                height: 0,
                width: 0,
                url: URL.createObjectURL(file),
                size: file.size,
                formats: {}
            }
        }
        return undefined;
    });


    const schema = z.object({
        name: z.string().min(1, "Name is required"),
        url: z.string().min(1, "URL is required"),
        icon: z.object({
            name: z.string().min(1, "Name is required"),
            alternative_text: z.string(),
            height: z.number(),
            width: z.number(),
            url: z.string().min(1, "URL is required"),
            size: z.number(),
            formats: z.object({}),
        })
    });

    let errors = $state({
        name: null as string | null,
        url: null as string | null,
        icon: null as string | null,
    })

    function validateInput() {
        try {
            schema.parse({
                name, url, icon
            });
            errors = {name: null, url: null, icon: null};
            return true;
        } catch (err) {
            if (err instanceof z.ZodError) {
                errors = err.errors.reduce(
                    (acc, error) => {
                        if (error.path[0] === "name") {
                            acc.name = error.message;
                        }
                        if (error.path[0] === "url") {
                            acc.url = error.message;
                        }
                        if (error.path[0] === "icon") {
                            acc.icon = error.message;
                        }
                        return acc;
                    },
                    {name: null, url: null, icon: null}
                );
            }
            return false;
        }
    }
    

    function updateIcon(e: Event, index: number) {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = (event: ProgressEvent<FileReader>) => {
                // Replace the current icon's URL with the new one (base64 string)
                contact.social_media[index].icon.url = event.target?.result as string;
            };
            reader.readAsDataURL(file);
        }
    }

    function handleDeleteEntry(index: number) {
        delete contact.social_media[index];
    }

    function handleAddEntry() {
        if (!validateInput()) {
            return;
        }

        contact.social_media.push({
           name, url, icon
        });

        showDialog = false;
    }

</script>

<div class="flex flex-col gap-5 pb-2">
    <div class="text-lg font-bold">
        <h1 class="inline border-b-2 border-b-primary">Social Media</h1>
    </div>

    <div class="flex flex-col gap-3 pl-2">
        {#each contact.social_media as socialMedia, index}
            <div class="flex flex-col gap-5">
                {#each Object.entries(socialMedia) as [key, value]}
                    <div class="flex flex-row items-center gap-5">
                        <!-- Render Label -->
                        <Label for={`${key}-${index}`} class="min-w-[100px] text-left">{formatKey(key)}</Label>

                        {#if key === 'icon'}
                            <div class="flex flex-row items-center gap-2">
                                <img
                                        src={socialMedia.icon.url}
                                        alt={socialMedia.icon.alternative_text}
                                        class="w-10 h-10 object-contain border border-zinc-400 rounded"
                                />
                                <Input
                                        id={`${key}-${index}`}
                                        type="file"
                                        class="text-sm"
                                        accept="image/*"
                                        onchange={(e) => updateIcon(e, index)}
                                />
                            </div>
                        {:else if key === 'url'}
                            <Input
                                    id={`${key}-${index}`}
                                    type="url"
                                    class="flex-1"
                                    bind:value={socialMedia[key]}
                            />
                        {:else if key === 'name'}
                            <Input
                                    id={`${key}-${index}`}
                                    type="text"
                                    class="flex-1"
                                    bind:value={socialMedia[key]}
                            />
                        {/if}
                    </div>
                {/each}
            </div>


            <div class="flex flex-row items-center gap-5">
                <Button variant="outline" onclick={() => handleDeleteEntry(index)}>
                    <Trash2/>
                    Delete
                </Button>
            </div>

            {#if index !== contact.social_media.length - 1}
                <Separator class="border border-zinc-700 my-2"/>
            {/if}
        {/each}

        <div class="pt-2">
            <Button variant="outline" onclick={() => showDialog = true}>
                <Plus/>
                Add
            </Button>
        </div>
    </div>
</div>

<Dialog.Root bind:open={showDialog}>
    <Dialog.Content class="sm:max-w-1/5">
        <Dialog.Header>
            <Dialog.Title>Add Opening Hour</Dialog.Title>
        </Dialog.Header>
        <div class="grid grid-2 pt-4 gap-3 px-10">
            <div class="grid grid-flow-row items-center grid-cols-6 gap-1.5">
                <Label for="name" class="col-span-1">Name</Label>
                <Input
                        autocomplete="off"
                        id="name"
                        bind:value={name}
                        placeholder="Name"
                        class="col-span-5 w-fit {errors.name ? 'focus-visible:ring-red-500' : ''}"
                />
                <p class="text-red-500 text-sm col-span-6 h-4 pl-2">
                    {errors.name || "\u00A0"}
                </p>
            </div>

            <div class="grid grid-flow-row items-center grid-cols-6 gap-1.5">
                <Label for="url" class="col-span-1">URL</Label>
                <Input
                        autocomplete="off"
                        id="url"
                        bind:value={url}
                        placeholder="URL"
                        class="col-span-5 w-fit {errors.url ? 'focus-visible:ring-red-500' : ''}"
                />
                <p class="text-red-500 text-sm col-span-5 h-4 pl-2">
                    {errors.url || "\u00A0"}
                </p>
            </div>
            
            <div class="grid grid-flow-row items-center grid-cols-6 gap-1.5">
                <Label for="icon" class="col-span-1">Icon</Label>
                <input
                        accept="image/*"
                        type="file"
                        id="icon"
                        bind:files
                        placeholder="Icon"
                        class="col-span-5 {errors.icon ? 'focus-visible:ring-red-500' : ''}"
                />
                <p class="text-red-500 text-sm col-span-6 h-4 pl-2">
                    {errors.icon || "\u00A0"}
                </p>
            </div>
        </div>

        <div class="flex justify-end space-x-2">
            <Dialog.Close>
                <Button variant="outline">Cancel</Button>
            </Dialog.Close>

            <Button variant="outline" onclick={handleAddEntry}>Add</Button>
        </div>
    </Dialog.Content>
</Dialog.Root>