<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    export let toggleSettingsMenu: () => void;
    export let launcherOptions: { [key: string]: string };
    export let token: string;
    export let username: string;
    export let jvmArguments: string;
    export let minJvmArgument: string;
    export let maxJvmArgument: string;
    export let isSettingsOpen: boolean;

    let maxRAM: number = 0;

    async function fetchRamSize() {
        try {
            let ramSize: number = await invoke("get_ram_size");
            maxRAM = Math.floor(ramSize / 1024) * 1024; // Преобразуем в мегабайты
        } catch (error) {
            console.error(`Failed to get RAM size: ${error}`);
        }
    }

    function resetForm() {
        token = launcherOptions['token'] || "";
        jvmArguments = launcherOptions['jvmArguments'] || "";
        const jvmArgsArray = jvmArguments.split(" ");
        minJvmArgument = jvmArgsArray[0] || "";
        maxJvmArgument = jvmArgsArray[1] || "";
    }

    async function saveLauncherOptions() {
        try {
            const textInput = document.querySelector(".max-box") as HTMLInputElement;
            maxJvmArgument = textInput.value;
            await invoke('save_launcher_options', { 
                username: username,
                token: token, 
                minJvmArgument: minJvmArgument,
                maxJvmArgument: maxJvmArgument
            });
            console.log('Launcher options saved successfully.');
        } catch (error) {
            console.error('Failed to save launcher options:', error);
        }
    }

    function handleAccept() {
        saveLauncherOptions();
        toggleSettingsMenu();
    }

    function handleCancel() {
        resetForm();
        toggleSettingsMenu();
    }

    // Инициализация значений
    onMount(async () => {
        await fetchRamSize();
    });
</script>

<main>
    <div class="settings-block {isSettingsOpen ? 'active' : ''}">
        <div class="namashka-craft-settings-header">
            <button class="close-button" on:click={handleCancel}>
                <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
                    <path d="M23.75 7.5125L21.9875 5.75L15 12.7375L8.0125 5.75L6.25 7.5125L13.2375 14.5L6.25 21.4875L8.0125 23.25L15 16.2625L21.9875 23.25L23.75 21.4875L16.7625 14.5L23.75 7.5125Z"/>
                </svg>
            </button>
        </div>
        <div class="namashka-craft-settings-main">
            <div class="settings-input-box">
                <div class="namashka-craft-settings-name">Настройки</div>
                <div class="token">
                    <input class="namashka-craft-token-input" type="text" maxlength="70" placeholder=" " bind:value={token}>
                    <label class="namashka-craft-token-input-placeholder">Введите токен</label>
                </div>

                <div class="ram">
                    <div class="ram-name">Оперативная память (MB)</div>
                    <div class="ram-input">
                        <input class="ram-range" type="range" min="0" max={maxRAM} step="1000" bind:value={maxJvmArgument}>
                        <div class="max">
                            <input class="max-box" type="text" placeholder=" " maxlength="5" bind:value={maxJvmArgument}>
                            <label class="max-box-placeholder">RAM</label>
                        </div>
                    </div>
                </div>
                <div class="buttons">
                    <button on:click={handleCancel} class="cancel">Отмена</button>
                    <button on:click={handleAccept} class="accept">Принять</button>
                </div>
            </div>
        </div>
    </div>
</main>

<style>
    .settings-block {
        margin-top: 100vh;
        box-shadow: 0 0 10px var(--shadow-color);
        border-radius: 10px;
        transition: margin-top 1s;
    }

    .settings-block.active {
        margin-top: 0;
    }

    .namashka-craft-settings-main {
        background-color: #2A2F32;
        border-radius: 0 0 10px 10px;
        width: 58.33vw;
        height: calc(64.81vh - 40px);
        text-align: center;
    }

    .namashka-craft-settings-header {
        position: relative;
        width: 58.33vw;
        height: 40px;
        background-color: #212429;
        border-radius: 10px 10px 0 0;
    }

    .close-button {
        position: absolute;
        top: 5px;
        right: 5px;
        background-color: rgba(0, 0, 0, 0);
        cursor: pointer;
        border-style: none;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        fill: white;
    }

    .token,
    .token input,
    .ram,
    .max-box,
    .cancel,
    .accept {
        font-family: "Inter", sans-serif;
        font-weight: 600;
        font-optical-sizing: auto;
        font-style: normal;
        font-variation-settings: "slnt" 0;
    }

    .settings-input-box {
        position: relative;
        top: 10%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .namashka-craft-settings-name {
        position: relative;
        color: #fff;
        font-size: 40px;
    }

    .token {
        position: relative;
        display: flex;
        flex-wrap: wrap;
        align-items: start;
        flex-direction: column;
        justify-content: center;
    }

    .namashka-craft-token-input {
        position: relative;
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-items: center;
        justify-content: center;
    }

    .namashka-craft-token-input:focus,
    .namashka-craft-token-input:not(:placeholder-shown) {
        border-color: #4390D8;
    }

    .namashka-craft-token-input {
        position: relative;
        width: 400px;
        height: 40px;
        top: 38px;
        background-color: transparent;
        outline: none;
        border-radius: 10px;
        border-color: white;
        border-width: 1px;
        border-style: solid;
        margin-bottom: 5.19vh;
        color: var(--input-text-color);
        text-align: left;
        padding: 10px;
        font-size: 16px;
        transition: border-color .3s;
        box-sizing: border-box;
    }

    .namashka-craft-token-input-placeholder {
        position: relative;
        width: 125px;
        top: -20px;
        left: 5px;
        color: var(--secondary-text-color);
        text-align: center;
        transition: color 0.3s, background-color 0.3s, transform 0.3s;
        user-select: none;
        pointer-events: none;
    }

    .namashka-craft-token-input:focus + .namashka-craft-token-input-placeholder,
    .namashka-craft-token-input:not(:placeholder-shown) + .namashka-craft-token-input-placeholder {
        color: #4390D8;
        background-color: #2A2F32;
        transform: translate(-8%, -98%) scale(80%);
    }

    .ram {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .ram-name {
        color: #9A9A9A;
        font-size: 14px;
        font-weight: 600;
    }

    .ram-input {
        display: flex;
        flex-direction: row;
        width: 400px;
    }

    .ram-range {
        width: 70%;
    }

    .ram-range::-webkit-slider-thumb {
        cursor: pointer;
    }

    .max {
        position: relative;
        width: 30%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-items: center;
        justify-content: center;
        top: 8px;
    }

    .max-box:focus,
    .max-box:not(:placeholder-shown) {
        border-color: #4390D8;
    }

    .max-box {
        position: relative;
        width: 100px;
        height: 40px;
        background-color: transparent;
        outline: none;
        border-radius: 10px;
        border-color: white;
        border-width: 1px;
        border-style: solid;
        color: var(--input-text-color);
        text-align: center;
        font-size: 16px;
        transition: .3s;
        box-sizing: border-box;
    }

    .max-box-placeholder {
        position: relative;
        width: 40px;
        top: -30px;
        color: var(--secondary-text-color);
        text-align: center;
        transition: 0.3s;
        user-select: none;
        pointer-events: none;
    }

    .max-box:focus + .max-box-placeholder,
    .max-box:not(:placeholder-shown) + .max-box-placeholder {
        color: #4390D8;
        background-color: #2A2F32;
        transform: translate(-65%, -98%) scale(80%);
    }

    .buttons {
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 1.67vw;
        top: 42.41vh;
    }
    
    .cancel {
        width: 13.02vw;
        height: 7.03vh;
        background-color: rgba(0, 0, 0, 0);
        color: #4390D8;
        font-size: 18px;
        user-select: none;
        border-radius: 10px;
        border-style: solid;
        border-width: 2px;
        border-color: #4390D8;
        cursor: pointer;
        transition: color 0.3s, box-shadow 0.3s, font-size 0.3s, background-color 0.3s, border-color 0.3s;
    }

    .cancel:hover {
        color: #6BA8E0;
        font-size: 19px;
        border-color: #6BA8E0;
        box-shadow: 0 0px 10px #6BA8E0;
    }
    
    .accept {
        width: 13.02vw;
        height: 7.03vh;
        background-color: #4390D8;
        color: #fff;
        font-size: 18px;
        user-select: none;
        border: none;
        border-radius: 10px;
        cursor: pointer;
        transition: color 0.3s, box-shadow 0.3s, font-size 0.3s, background-color 0.3s, border-color 0.3s;
    }

    .accept:hover {
        background-color: #6BA8E0;
        font-size: 19px;
        border-color: #6BA8E0;
        box-shadow: 0 0px 10px #6BA8E0;
    }
</style>