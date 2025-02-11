<script lang="ts">
    import {ENTRY_TYPES} from "$utils/interfaces.js";
    import {Label} from "@/components/ui/label/index.js";
    import {Input} from "@/components/ui/input/index.js";
    import {Button} from "@/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import {Separator} from "@/components/ui/separator/index.js";
    import {Plus, Trash2} from "lucide-svelte";
    import {Checkbox} from "@/components/ui/checkbox/index.js";
    import {formatKey} from "$utils";
    import {z} from "zod";


    let {contact = $bindable()} = $props();

    let showAddOpeningHourDialog = $state(false);
    let begin: string | null = $state(null);
    let end: string | null = $state(null);
    let days: string = $state("");
    let closed: boolean = $state(false);

    const schema = z.object({
        begin: z.string().min(1, "Begin is required"),
        end: z.string().min(1, "End is required"),
        closed: z.boolean(),
        days: z.string().min(1, "Days is required"),
    });

    let errors = $state({
        begin: null as string | null,
        end: null as string | null,
        closed: null as string | null,
        days: null as string | null,
    });


    function validateInput() {
        try {
            schema.parse({
                begin, end, closed, days,
            });
            errors = {begin: null, end: null, closed: null, days: null};
            return true;
        } catch (err) {
            if (err instanceof z.ZodError) {
                errors = err.errors.reduce(
                    (acc, error) => {
                        if (error.path[0] === "begin") {
                            acc.begin = error.message;
                        }
                        if (error.path[0] === "end") {
                            acc.end = error.message;
                        }
                        if (error.path[0] === "closed") {
                            acc.closed = error.message;
                        }
                        if (error.path[0] === "days") {
                            acc.days = error.message;
                        }
                        return acc;
                    },
                    {begin: null, end: null, closed: null, days: null}
                );
            }
            return false;
        }
    }


    function resetTimeFields(openingHour: any, index: number) {
        if (openingHour.closed) {
            contact.opening_hours[index].begin = null;
            contact.opening_hours[index].end = null;
        }
    }

    function resetDialogTimeFields() {
        begin = null;
        end = null;
    }

    function deleteOpeningHour(index: number) {
        contact.opening_hours.splice(index, 1);
    }

    function addOpeningHour() {
        if (!validateInput()) {
            return;
        }

        contact.opening_hours.push({
            begin, end, closed, days,
        });

        showAddOpeningHourDialog = false;
    }
</script>


<div class="flex flex-col gap-5 pb-2">
    <div class="text-lg font-bold">
        <h1 class="inline border-b-2 border-b-primary">Opening hours</h1>
    </div>

    <div class="flex flex-col gap-3 pl-2">
        {#each contact.opening_hours as openingHour, index}
            <div class="flex flex-col gap-5">
                {#each Object.entries(openingHour) as [key, value]}
                    <div class="flex flex-row items-center gap-5">
                        <Label for={`${key}-${index}`} class="min-w-[100px] text-left">{formatKey(key)}</Label>

                        {#if key === 'begin' || key === 'end'}
                            <Input
                                    id={`${key}-${index}`}
                                    type="time"
                                    class="w-[100px]"
                                    bind:value={openingHour[key]}
                                    disabled={openingHour.closed}
                            />
                        {:else if key === 'closed'}
                            <Checkbox
                                    id={`${key}-${index}`}
                                    class="border-border"
                                    bind:checked={openingHour.closed}
                                    onCheckedChange={() => resetTimeFields(openingHour, index)}
                            />
                        {:else}
                            <Input
                                    id={`${key}-${index}`}
                                    type="text"
                                    class="flex-1"
                                    bind:value={openingHour[key]}
                            />
                        {/if}
                    </div>
                {/each}
            </div>

            <div class="flex flex-row items-center gap-5">
                <Button variant="outline" onclick={() => deleteOpeningHour(index)}>
                    <Trash2/>
                    Delete
                </Button>
            </div>


            {#if index !== contact.opening_hours.length - 1}
                <Separator class="border border-zinc-700 my-2"/>
            {/if}
        {/each}


        <div class="pt-2">
            <Button variant="outline" onclick={() => showAddOpeningHourDialog = true}>
                <Plus/>
                Add
            </Button>
        </div>
    </div>
</div>


<Dialog.Root bind:open={showAddOpeningHourDialog}>
    <Dialog.Content class="sm:max-w-1/5">
        <Dialog.Header>
            <Dialog.Title>Add Opening Hour</Dialog.Title>
        </Dialog.Header>
        <div class="grid grid-2 pt-4 gap-3 px-10">
            <div class="grid grid-flow-row items-center grid-cols-6 gap-1.5">
                <Label for="begin" class="col-span-1">Begin</Label>
                <Input
                        disabled={closed}
                        type="time"
                        autocomplete="off"
                        id="begin"
                        bind:value={begin}
                        placeholder="Begin"
                        class="col-span-5 w-fit {errors.begin ? 'focus-visible:ring-red-500' : ''}"
                />
                <p class="text-red-500 text-sm col-span-6 h-4 pl-2">
                    {errors.begin || "\u00A0"}
                </p>
            </div>

            <div class="grid grid-flow-row items-center grid-cols-6 gap-1.5">
                <Label for="end" class="col-span-1">End</Label>
                <Input
                        disabled={closed}
                        type="time"
                        autocomplete="off"
                        id="end"
                        bind:value={end}
                        placeholder="End"
                        class="col-span-5 w-fit {errors.end ? 'focus-visible:ring-red-500' : ''}"
                />
                <p class="text-red-500 text-sm col-span-5 h-4 pl-2">
                    {errors.end || "\u00A0"}
                </p>
            </div>


            <div class="grid grid-flow-row items-center grid-cols-6 gap-1.5">
                <Label for="closed" class="col-span-1">Closed</Label>
                <Checkbox
                        id="closed"
                        bind:checked={closed}
                        placeholder="closed"
                        class="col-span-5 {errors.closed ? 'focus-visible:ring-red-500' : ''}"
                        onCheckedChange={resetDialogTimeFields}
                />
                <p class="text-red-500 text-sm col-span-6 h-4 pl-2">
                    {errors.closed || "\u00A0"}
                </p>
            </div>

            <div class="grid grid-flow-row items-center grid-cols-6 gap-1.5">
                <Label for="days" class="col-span-1">Days</Label>
                <Input
                        autocomplete="off"
                        id="days"
                        bind:value={days}
                        placeholder="Days"
                        class="col-span-5 {errors.days ? 'focus-visible:ring-red-500' : ''}"
                />
                <p class="text-red-500 text-sm col-span-6 h-4 pl-2">
                    {errors.days || "\u00A0"}
                </p>
            </div>
        </div>

        <div class="flex justify-end space-x-2">
            <Dialog.Close>
                <Button variant="outline">Cancel</Button>
            </Dialog.Close>

            <Button variant="outline" onclick={addOpeningHour}>Add</Button>
        </div>
    </Dialog.Content>
</Dialog.Root>



