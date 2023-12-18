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
        <HeaderNavItem
            isSelected={$currentPage.page === PageName.INDEX}
            on:click={() => {
                goToPage({ page: PageName.INDEX });
            }}
            text="Home"
        />
        <HeaderNavItem
            isSelected={$currentPage.page === PageName.LOGS}
            on:click={() => {
                goToPage({ page: PageName.LOGS });
            }}
            text="Logs"
        />
        <HeaderNavItem
            isSelected={$currentPage.page === PageName.HELP}
            on:click={() => {
                goToPage({ page: PageName.HELP });
            }}
            text="Help"
        />
        <HeaderNavItem
            isSelected={$currentPage.page === PageName.ABOUT}
            on:click={() => {
                goToPage({ page: PageName.ABOUT });
            }}
            text="About"
        />
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
