<script lang="ts">
  import { onMount } from 'svelte';

  import Header from '../src/Header.svelte';
  import MainMenu from './MainMenu.svelte';

  import { invoke } from '@tauri-apps/api/tauri';

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
      console.log(launcherOptionsExists);
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

  onMount(() => {
    checkLauncherOptions();
  });
</script>

<main>
  <div class="border-top"></div>
  <div class="border-bottom"></div>
  <div class="border-left"></div>
  <div class="border-right"></div>
  <Header />
  <MainMenu 
    {checkLauncherOptions}
    {launcherOptions} 
    {token} 
    {username} 
    {jvmArguments} 
    {minJvmArgument} 
    {maxJvmArgument} 
    {launcherOptionsExists} />
</main>

<style>
</style>