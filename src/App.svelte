<script type="ts">
  import Header from '../src/Header.svelte';
  import MainMenu from './MainMenu.svelte';
  import SettingsMenu from './SettingsMenu.svelte';

  export let isMenuOpen = true;
  export let isSettingsOpen = false;

  function toggleMainMenu() {
    isMenuOpen = !isMenuOpen;
  }

  function toggleSettingsMenu() {
    isSettingsOpen = !isSettingsOpen;
  }
</script>

<style>
  .main-menu {
    display: none;
  }
  .main-menu.open {
    display: block;
  }

  .settings-menu {
    display: none;
  }
  .settings-menu.open {
    display: block;
  }
</style>

<main>
  <div class="border-top"></div>
  <div class="border-bottom"></div>
  <div class="border-left"></div>
  <div class="border-right"></div>
  <Header />
  <div class="main-menu" class:open={isMenuOpen}>
    <MainMenu {toggleMainMenu} {toggleSettingsMenu}/>
  </div>
  <div class="settings-menu" class:open={isSettingsOpen}>
    <SettingsMenu {toggleMainMenu} {toggleSettingsMenu}/>
  </div>
</main>