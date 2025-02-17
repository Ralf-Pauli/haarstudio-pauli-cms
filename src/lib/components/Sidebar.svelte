<script lang="ts">
    import Calendar from "lucide-svelte/icons/calendar";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import {CircleUser, House, Images, ListTree, Settings} from "lucide-svelte";

    // Menu items.
    const items = [
        {
            title: "Home",
            url: "/",
            icon: House,
        },
        {
            title: "Leistungen",
            url: "/services",
            icon: ListTree,
        },
        {
            title: "Galerie",
            url: "/gallery",
            icon: Images,
        },
        {
            title: "Kontakt",
            url: "/contact",
            icon: CircleUser,
        }
    ];
</script>

<Sidebar.Root collapsible="icon" variant="sidebar">
    <Sidebar.Content>
        <Sidebar.Group>
            <Sidebar.GroupLabel>Haarstudio Pauli</Sidebar.GroupLabel>
            <Sidebar.GroupContent>
                <Sidebar.Menu>
                    {#each items as item (item.title)}
                        <Sidebar.MenuItem>
                            <Sidebar.MenuButton>
                                {#snippet child({ props })}
                                    <a href={item.url} {...props}>
                                        <item.icon />
                                        <span>{item.title}</span>
                                    </a>
                                {/snippet}
                            </Sidebar.MenuButton>
                        </Sidebar.MenuItem>
                    {/each}
                </Sidebar.Menu>
            </Sidebar.GroupContent>
        </Sidebar.Group>
    </Sidebar.Content>
    <Sidebar.Footer>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <Sidebar.MenuButton>
                    {#snippet child({ props })}
                        <!--<a href="/images" {...props}>
                            <Images />
                            <span>Images</span>
                        </a>-->
                        <a href="/settings" {...props}>
                            <Settings />
                            <span>Settings</span>
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Footer>
</Sidebar.Root>