<script>
    import {
        Header,
        HeaderNav,
        HeaderNavItem,
        SkipToContent,
        Content,
        Grid,
        Row,
        Column,
    } from "carbon-components-svelte";
    import { PageName, currentPage, goToPage } from "../store/nav_store";

    let isSideNavOpen = false;
</script>

<Header company="Derpity" platformName="Gnome App Manager" bind:isSideNavOpen>
    <svelte:fragment slot="skip-to-content">
        <SkipToContent />
    </svelte:fragment>
    <HeaderNav>
        {#if $currentPage.page !== PageName.INDEX}
            <HeaderNavItem
                on:click={() => {
                    goToPage({ page: PageName.INDEX });
                }}
                text="Home"
            />
        {/if}
        {#if $currentPage.page !== PageName.LOGS}
            <HeaderNavItem
                on:click={() => {
                    goToPage({ page: PageName.LOGS });
                }}
                text="Logs"
            />
        {/if}
        <HeaderNavItem text="Help" />
        <HeaderNavItem text="About" />
        <!-- <HeaderNavItem href="/" text="Link 3" /> -->
        <!-- <HeaderNavMenu text="Menu"> -->
        <!--     <HeaderNavItem href="/" text="Link 1" /> -->
        <!--     <HeaderNavItem href="/" text="Link 2" /> -->
        <!--     <HeaderNavItem href="/" text="Link 3" /> -->
        <!-- </HeaderNavMenu> -->
    </HeaderNav>
</Header>

<Content>
    <Grid>
        <Row>
            <Column class="content">
                <slot name="header" />
                <slot name="body" />
            </Column>
        </Row>
    </Grid>
</Content>

<style>
    :global(.content) {
        padding: 2rem;
        min-height: 100vh;
    }
</style>
