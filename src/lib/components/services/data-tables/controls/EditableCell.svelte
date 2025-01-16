<script lang="ts">
    import {Input} from "@/components/ui/input";

    let {value, update}: { value: string | number | undefined, update: (value: string | number | undefined) => void } = $props();

    function handleInput(event: Event) {
        const newValue = (event.target as HTMLInputElement).value;
        update(newValue);
    }
</script>

<Input bind:value oninput={handleInput}/>

