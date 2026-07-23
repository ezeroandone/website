<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import SiteNav from '$lib/components/SiteNav.svelte';
  import SiteFooter from '$lib/components/SiteFooter.svelte';
  import type { LayoutData } from './$types';

  let { children, data } = $props();

  // Hide the shared nav/footer on admin and auth routes — those have their own chrome.
  const isAdminRoute = $derived($page.url.pathname.startsWith('/admin'));
  const isAuthRoute  = $derived($page.url.pathname.startsWith('/auth') || $page.url.pathname.startsWith('/onboarding'));
  const showChrome   = $derived(!isAdminRoute && !isAuthRoute);
</script>

<svelte:head>
  <title>eZeroAndOne</title>
  <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
  <link rel="icon" type="image/png" href="/favicon.png" sizes="32x32" />
  <link rel="apple-touch-icon" href="/favicon.svg" />
  <meta name="theme-color" content="#000000" />
</svelte:head>

<div class="app-shell" data-theme="dark">
  {#if showChrome}
    <SiteNav isLoggedIn={data.isLoggedIn} />
  {/if}

  {@render children()}

  {#if showChrome}
    <SiteFooter />
  {/if}
</div>

<style>
  .app-shell {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }
</style>
