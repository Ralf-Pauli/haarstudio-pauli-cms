<script lang="ts">
    import type { ComponentProps } from "svelte";
    import ArrowUpDown from "lucide-svelte/icons/arrow-up-down";
    import { Button } from "@/components/ui/button";

    let { variant = "ghost", ...restProps }: ComponentProps<typeof Button> =
        $props();
</script>

<Button {variant} {...restProps}>
    Name
    <ArrowUpDown class="ml-2 size-4" />
</Button>