<script lang="ts">
  import Header from '../src/Header.svelte';
  import MainMenu from './MainMenu.svelte';
  import SettingsMenu from './SettingsMenu.svelte';

  import { invoke } from '@tauri-apps/api/tauri';

  export let isMenuOpen = true;
  export let isSettingsOpen = false;

  let launcherOptions: { [key: string]: string } = {};
  let token: string = "";
  let username: string = "";
  let jvmArguments: string = "";
  let minJvmArgument: string = "";
  let maxJvmArgument: string = "";
  let launcherOptionsExists: boolean = false;

  async function checkLauncherOptions() {
    try {
      launcherOptionsExists = await invoke('check_launcher_options');
      if (launcherOptionsExists) {
        launcherOptions = await invoke<{ [key: string]: string }>('read_launcher_options');
        console.log("Launcher options:", launcherOptions);

        if (launcherOptions) {
          token = launcherOptions['token'] || "";
          username = launcherOptions['username'] || "";
          jvmArguments = launcherOptions['jvmArguments'] || "";

          const jvmArgsArray = jvmArguments.split(" ");
          minJvmArgument = jvmArgsArray[0] || "";
          maxJvmArgument = jvmArgsArray[1] || "";
        } else {
          token = "";
          username = "";
          jvmArguments = "0 3000";

          const jvmArgsArray = jvmArguments.split(" ");
          minJvmArgument = jvmArgsArray[0] || "";
          maxJvmArgument = jvmArgsArray[1] || "";
        }
      }
    } catch (e) {
      console.error('Failed to check launcher options', e);
    }

    return {
      launcherOptions,
      token,
      username,
      jvmArguments,
      minJvmArgument,
      maxJvmArgument
    };
  }

  function toggleMainMenu() {
    isMenuOpen = !isMenuOpen;
  }

  function toggleSettingsMenu() {
    isSettingsOpen = !isSettingsOpen;
  }
</script>

<main>
  <div class="border-top"></div>
  <div class="border-bottom"></div>
  <div class="border-left"></div>
  <div class="border-right"></div>
  <Header />
  {#if isMenuOpen}
    <MainMenu 
      {checkLauncherOptions}
      {toggleMainMenu} 
      {toggleSettingsMenu} 
      {launcherOptions} 
      {token} 
      {username} 
      {jvmArguments} 
      {minJvmArgument} 
      {maxJvmArgument} 
      {launcherOptionsExists} />
  {:else if isSettingsOpen}
    <SettingsMenu 
      {toggleMainMenu} 
      {toggleSettingsMenu} 
      {launcherOptions} 
      {token} 
      {username} 
      {jvmArguments} 
      {minJvmArgument} 
      {maxJvmArgument} />
  {/if}
</main>

<style>
</style>