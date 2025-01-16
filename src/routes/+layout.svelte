<script lang="ts">
	import '../app.css';
    import * as ShadcnSidebar from '@/components/ui/sidebar/index';
    import Sidebar from "@/components/Sidebar.svelte";
    import {ModeWatcher} from "mode-watcher";
    import {Toaster} from "@/components/ui/sonner";

	let { children } = $props();
</script>

<ShadcnSidebar.Provider>
    <ModeWatcher />
    <Sidebar />
    <Toaster />
    <main class="w-full h-full">
        <ShadcnSidebar.Trigger />
        <div class="px-12 py-5">
            {@render children?.()}
        </div>
    </main>
</ShadcnSidebar.Provider>






