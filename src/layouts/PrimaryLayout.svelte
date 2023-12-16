<script>
    import {
        Header,
        HeaderNav,
        HeaderNavItem,
        HeaderNavMenu,
        SkipToContent,
        Content,
        Grid,
        Row,
        Column,
    } from "carbon-components-svelte";

    let isSideNavOpen = false;
</script>

<Header company="Derpity" platformName="Gnome App Manager" bind:isSideNavOpen>
    <svelte:fragment slot="skip-to-content">
        <SkipToContent />
    </svelte:fragment>
    <HeaderNav>
        <HeaderNavItem href="/" text="Link 1" />
        <HeaderNavItem href="/" text="Link 2" />
        <HeaderNavItem href="/" text="Link 3" />
        <HeaderNavMenu text="Menu">
            <HeaderNavItem href="/" text="Link 1" />
            <HeaderNavItem href="/" text="Link 2" />
            <HeaderNavItem href="/" text="Link 3" />
        </HeaderNavMenu>
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
