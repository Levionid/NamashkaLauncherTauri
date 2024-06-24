<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    // Import images correctly
    import namashkaCraftTabImage from '../src-tauri/assets/namashkaCraftTabImage.png';

    export let toggleMainMenu: () => void;
    export let toggleSettingsMenu: () => void;

    let launcherOptionsExists: boolean = false;
    let showTokenInput: boolean = false;

    let launcherOptions: { [key: string]: string } = {};
    let token: string = "";
    let username: string = "";
    let jvmArguments: string = "";
    let minJvmArgument: string = "";
    let maxJvmArgument: string = "";

    async function openExplorer() {
        try {
            await invoke('open_explorer');
            console.log('Explorer opened');
        } catch (e) {
            console.error('Failed to open explorer', e);
        }
    }

    async function checkTokenInput() {
        const usernameInput = document.querySelector('.namashka-craft-token-input') as HTMLInputElement;
        const errorFeedback = document.querySelector('.namashka-craft-token-error-feedback') as HTMLElement;

        if (usernameInput && errorFeedback) {
            errorFeedback.textContent = '';

            if (usernameInput.value.trim() === '') {
                errorFeedback.textContent = 'Введите токен!';
                return false;
            }

        }

        return true;
    }

    async function startMinecraftLoader() {
        const usernameInput = document.querySelector('.namashka-craft-username-input-box') as HTMLInputElement;
        const errorFeedback = document.querySelector('.error-feedback') as HTMLElement;

        if (usernameInput && errorFeedback) {
            errorFeedback.textContent = '';

            if (usernameInput.value.trim() === '') {
                errorFeedback.textContent = 'Введите ник!';
                return;
            }

            try {
                saveUsername();
                await invoke('start_minecraft_loader');
                console.log('Minecraft Loader started');
            } catch (e) {
                if (e)  {
                    errorFeedback.textContent = e.toString(); // Display Rust error message
                    console.error('Failed to start Minecraft Loader', e);
                }
            }
        }
    }

    async function checkLauncherOptions() {
        try {
            launcherOptionsExists = await invoke('check_launcher_options');
            launcherOptions = await invoke<{ [key: string]: string }>('read_launcher_options');
            console.log("Launcher options:", launcherOptions);

            if (launcherOptions) {
                token = launcherOptions['token'] || "";
                username = launcherOptions['username'] || "";
                jvmArguments = launcherOptions['jvmArguments'] || "";

                const jvmArgsArray = jvmArguments.split(" ");
                minJvmArgument = jvmArgsArray[0] || "";
                maxJvmArgument = jvmArgsArray[1] || "";
            }
        } catch (e) {
            console.error('Failed to check launcher options', e);
        }
    }

    async function toggleTokenInput() {
        showTokenInput = !showTokenInput;
        const overlay = document.querySelector('.overlay');
        if (overlay) {
            overlay.classList.toggle('active', showTokenInput);
        }
    }

    async function saveLauncherOptions() {
        const min_jvm_argument = "0";
        const max_jvm_argument = "3000";
        const tokenInput = document.querySelector('.namashka-craft-token-input') as HTMLInputElement;
        const tokenInputString = tokenInput.value.trim();
        console.log("Token:", tokenInputString);
        checkTokenInput();

        if (tokenInputString) {
            try {
                await invoke('save_launcher_options', {
                    username: "",
                    token: tokenInputString,
                    minJvmArgument: min_jvm_argument,
                    maxJvmArgument: max_jvm_argument 
                });
                
                console.log('Launcher options saved successfully.');

                checkLauncherOptions();
                toggleTokenInput();
            } catch (error) {
                console.error('Failed to save launcher options:', error);
            }
        }

    }

    async function saveUsername() {
        try {
            launcherOptions = await invoke<{ [key: string]: string }>('read_launcher_options');
            console.log("Launcher options:", launcherOptions);

            if (launcherOptions) {
                token = launcherOptions['token'] || "";
                const usernameHTMLInputElement = document.querySelector('.namashka-craft-username-input-box') as HTMLInputElement;
                const username = usernameHTMLInputElement.value;
                jvmArguments = launcherOptions['jvmArguments'] || "";

                // Split jvmArguments into minJvmArgument and maxJvmArgument
                const jvmArgsArray = jvmArguments.split(" ");
                minJvmArgument = jvmArgsArray[0] || "";
                maxJvmArgument = jvmArgsArray[1] || "";

                console.log("Token:", token);
                console.log("Username:", username);
                console.log("JVM Arguments:", jvmArguments);
                console.log("Min JVM Argument:", minJvmArgument);
                console.log("Max JVM Argument:", maxJvmArgument);

                try {
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
        } catch (error) {
            console.error('Failed to check or read launcher options:', error);
        }
        
    }

    onMount(() => {
        checkLauncherOptions();
    });
</script>

<main>
    <div class="main-content">
        {#if !launcherOptionsExists}
            <button class="namashka-craft-authorization-button" on:click={toggleTokenInput}>Ввести токен</button>
        {/if}
    </div>
    <div class="overlay {showTokenInput ? 'active' : ''}">
        <div class="namashka-craft-token">
            <div class="namashka-craft-token-header">
                <button class="close-button" on:click={toggleTokenInput}>
                    <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
                        <path d="M23.75 7.5125L21.9875 5.75L15 12.7375L8.0125 5.75L6.25 7.5125L13.2375 14.5L6.25 21.4875L8.0125 23.25L15 16.2625L21.9875 23.25L23.75 21.4875L16.7625 14.5L23.75 7.5125Z"/>
                    </svg>
                </button>
            </div>
            <div class="namashka-craft-token-main">
                <div class="token-input-box">
                    <div class="namashka-craft-token-name">Токен</div>
                    <div class="namashka-craft-token-input-box">
                        <div class="namashka-craft-token-text">Токен</div>
                        <input class="namashka-craft-token-input" type="text" maxlength="70" placeholder="Введите токен">
                    </div>
                    <button class="namashka-craft-save-button" on:click={saveLauncherOptions}>Сохранить</button>
                    <div class="namashka-craft-token-error-feedback"></div>
                </div>
            </div>
        </div>
    </div>
    <aside class="namashka-craft-tab">
        <div class="namashka-craft">
            <img class="namashka-craft-tab-image" src={namashkaCraftTabImage} alt="NamashkaCraftTabImage">
            <span class="namashka-craft-name">NamashkaCraft</span>
            <span class="namashka-craft-description">
                сборка на версии <span class="green-text">Fabric 1.20.1</span><br>
                содержит в себе <span class="green-text">>100 модов</span><br>
                сборка <span class="green-text">оптимизирована</span>
            </span>
        </div>
        <div class="namashka-craft-username">
            <span class="namashka-craft-username-text">Ник</span>
            <input class="namashka-craft-username-input-box" type="text" maxlength="15" placeholder="Введите ник" value={username} disabled={!launcherOptionsExists}>
        </div>
        <button class="namashka-craft-play-button" on:click={startMinecraftLoader} on:click={saveUsername} disabled={!launcherOptionsExists}>Играть</button>
        <div class="error-feedback"></div>
        <div class="buttons">
            <button on:click={openExplorer} disabled={!launcherOptionsExists} class="button-container">
                <svg class="folder-button" id="folder-button" viewBox="0 0 21 17" xmlns="http://www.w3.org/2000/svg">
                    <path d="M8.41665 0.166687H2.16665C1.02081 0.166687 0.0937296 1.10419 0.0937296 2.25002L0.083313 14.75C0.083313 15.8959 1.02081 16.8334 2.16665 16.8334H18.8333C19.9791 16.8334 20.9166 15.8959 20.9166 14.75V4.33335C20.9166 3.18752 19.9791 2.25002 18.8333 2.25002H10.5L8.41665 0.166687Z"/>
                </svg>
            </button>
            
            <button on:click={toggleMainMenu} on:click={toggleSettingsMenu} disabled={!launcherOptionsExists} class="button-container">
                <svg class="settings-button" id="settings-button" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 21 21">
                    <path id="icon_path" d="M17.9375 11.4792C17.9792 11.1667 18 10.8438 18 10.5C18 10.1667 17.9792 9.83333 17.9271 9.52083L20.0417 7.875C20.2292 7.72917 20.2813 7.44792 20.1667 7.23958L18.1667 3.78125C18.0417 3.55208 17.7813 3.47917 17.5521 3.55208L15.0625 4.55208C14.5417 4.15625 13.9896 3.82292 13.375 3.57292L13 0.927083C12.9584 0.677083 12.75 0.5 12.5 0.5H8.50003C8.25003 0.5 8.05212 0.677083 8.01045 0.927083L7.63545 3.57292C7.02087 3.82292 6.45837 4.16667 5.94795 4.55208L3.45837 3.55208C3.2292 3.46875 2.96878 3.55208 2.84378 3.78125L0.8542 7.23958C0.7292 7.45833 0.770866 7.72917 0.979199 7.875L3.09378 9.52083C3.0417 9.83333 3.00003 10.1771 3.00003 10.5C3.00003 10.8229 3.02087 11.1667 3.07295 11.4792L0.958366 13.125C0.770866 13.2708 0.718783 13.5521 0.833366 13.7604L2.83337 17.2188C2.95837 17.4479 3.21878 17.5208 3.44795 17.4479L5.93753 16.4479C6.45837 16.8437 7.01045 17.1771 7.62503 17.4271L8.00003 20.0729C8.05212 20.3229 8.25003 20.5 8.50003 20.5H12.5C12.75 20.5 12.9584 20.3229 12.9896 20.0729L13.3646 17.4271C13.9792 17.1771 14.5417 16.8437 15.0521 16.4479L17.5417 17.4479C17.7709 17.5312 18.0313 17.4479 18.1563 17.2188L20.1563 13.7604C20.2813 13.5312 20.2292 13.2708 20.0313 13.125L17.9375 11.4792ZM10.5 14.25C8.43753 14.25 6.75003 12.5625 6.75003 10.5C6.75003 8.4375 8.43753 6.75 10.5 6.75C12.5625 6.75 14.25 8.4375 14.25 10.5C14.25 12.5625 12.5625 14.25 10.5 14.25Z"/>
                </svg>
            </button>
        </div>
        
    </aside>
</main>

<style>
    main {
        display: flex;
        margin: 0;
    }

    aside {
        font-family: "Inter", sans-serif;
        font-weight: 600;
        font-optical-sizing: auto;
        font-style: normal;
        font-variation-settings: "slnt" 0;
    }

    .namashka-craft-authorization-button,
    .namashka-craft-name,
    .namashka-craft-description,
    .namashka-craft-username-text,
    .namashka-craft-username-input-box,
    .error-feedback,
    .namashka-craft-token input {
        font-family: "Inter", sans-serif;
        font-weight: 600;
        font-optical-sizing: auto;
        font-style: normal;
        font-variation-settings: "slnt" 0;
    }

    .main-content {
        position: absolute;
        top: var(--header-height);
        width: calc(100vw - 30.21vw);
        height: calc(100vh - var(--header-height));
    }

    .namashka-craft-authorization-button {
        position: relative;
        width: 26.88vw;
        height: 9.81vh;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: #4390D8;
        border-style: none;
        border-radius: 10px;
        color: #fff;
        font-size: 28px;
        text-shadow: 0 4px 4px var(--shadow-color);
        user-select: none;
        cursor: pointer;
        transition: 0.3s;
    }

    .namashka-craft-authorization-button:hover {
        background-color: #6BA8E0;
        box-shadow: 0 0 10px #6BA8E0;
        font-size: 29px;
    }

    .namashka-craft-tab {
        position: absolute;
        right: 0;
        top: var(--header-height);
        width: 30.21vw;
        height: calc(100vh - var(--header-height));
        background-color: var(--tab-background-color);
        box-shadow: -4px 0 4px var(--shadow-color);
        display: flex;
        flex-direction: column;
        align-items: center;
        z-index: 0;
    }

    .namashka-craft {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }

    .namashka-craft-tab-image {
        border-radius: 15px;
        width: 25.9vw;
        height: 26vh;
        box-shadow: 0 4px 4px var(--shadow-color);
        margin-top: 4.1vh;
        margin-bottom: 0.6vh;
    }

    .namashka-craft-name {
        color: var(--text-color);
        font-size: 28px;
        text-shadow: 0 4px 4px var(--shadow-color);
        margin-bottom: 1vh;
        width: 22.08vw;
        height: 5.37vh;
    }

    .namashka-craft-description {
        color: var(--secondary-text-color);
        font-size: 13px;
        text-shadow: 0 4px 4px var(--shadow-color);
    }

    .namashka-craft-username {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        margin-top: 13.21vh;
    }

    .namashka-craft-username-text {
        color: var(--username-text-color);
        font-size: 12px;
    }

    .namashka-craft-username-input-box {
        width: 11.8vw;
        height: 3.52vh;
        background-color: var(--input-background-color);
        border-radius: 3px;
        color: var(--input-text-color);
        text-align: center;
        font-size: 14px;
        border: none;
    }

    .namashka-craft-play-button {
        margin-top: 1.11vh;
        width: 12.5vw;
        height: 6.48vh;
        background-color: var(--button-background-color);
        border-radius: 10px;
        border: none;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 0.3s;
        text-shadow: 0 4px 4px var(--shadow-color);
        font-size: 28px;
        color: var(--text-color);
        user-select: none;
        font-family: "Inter", sans-serif;
        font-weight: 500;
        font-optical-sizing: auto;
        font-style: normal;
        font-variation-settings: "slnt" 0;
    }
    
    .namashka-craft-play-button:disabled {
        background-color: #999999;
    }
    
    .namashka-craft-play-button:enabled:hover {
        font-size: 29px;
        background: var(--button-hover-color);
        box-shadow: 0 0 10px var(--button-hover-color);
        cursor: pointer;
    }

    .error-feedback,
    .namashka-craft-token-error-feedback {
        color: red;
        font-size: 12px;
        font-weight: 500;
        text-align: center;
        margin-top: 5px;
    }

    .buttons {
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        bottom: 0;
        right: 0;
    }

    .button-container {
        background-color: rgba(0, 0, 0, 0);
        border-style: none;
        padding: 0;
    }

    .settings-button,
    .folder-button {
        width: 2vw;
        height: 3.7vh;
        padding: 2px;
        fill: #6A6B77;
        transition: 0.3s;
    }
    
    .button-container:enabled:hover svg {
        cursor: pointer;
        fill: #838490;
    }

    .green-text {
        color: var(--green-color);
    }

    .overlay {
        position: fixed;
        top: var(--header-height);
        left: 0;
        width: 100%;
        height: calc(100% - var(--header-height));
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        backdrop-filter: blur(0);
        background-color: rgba(0, 0, 0, 0);
        z-index: 999;
        opacity: 0; /* Initially transparent */
        visibility: hidden; /* Initially hidden */
        transition: opacity 0.3s, background-color 0.3s; /* Smooth transitions */
    }

    .overlay.active {
        opacity: 1; /* Fully visible */
        visibility: visible; /* Make visible */
        backdrop-filter: blur(5px); /* Apply blur effect */
        background-color: rgba(0, 0, 0, 0.6); /* Darken the background */
    }

    .namashka-craft-token {
        box-shadow: 0 0 10px var(--shadow-color);
        border-radius: 10px;
    }

    .namashka-craft-token-main {
        background-color: #2A2F32;
        border-radius: 0 0 10px 10px;
        width: 58.33vw;
        height: calc(64.81vh - 40px);
        text-align: center;
    }

    .namashka-craft-token-header {
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

    .token-input-box {
        position: relative;
        top: 20%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .namashka-craft-token-name {
        position: relative;
        color: #fff;
        margin-bottom: 23px;
        font-size: 40px;
        text-shadow: 0 4px 10px var(--shadow-color);
    }

    .namashka-craft-token-input-box {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: start;
    }

    .namashka-craft-token-input-box div {
        color: #9A9A9A;
        font-size: 12px;
        font-weight: 600;
        margin-bottom: 5px;
    }

    .namashka-craft-token-input-box input {
        width: 23.44vw;
        height: 4.44vh;
        color: #fff;
        background-color: #222628;
        border: none;
        border-radius: 6px;
        margin-bottom: 5.19vh;
    }

    .namashka-craft-save-button {
        width: 23.44vw;
        height: 4.44vh;
        background-color: #4390D8;
        border-radius: 6px;
        border: none;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 0.3s;
        cursor: pointer;
        text-shadow: 0 4px 4px var(--shadow-color);
        font-size: 14px;
        color: var(--text-color);
        cursor: pointer;
        user-select: none;
        font-family: "Inter", sans-serif;
        font-weight: 500;
        font-optical-sizing: auto;
        font-style: normal;
        font-variation-settings: "slnt" 0;
    }

    .namashka-craft-save-button:hover {
        font-size: 15px;
        background: #6BA8E0;
        box-shadow: 0 0 10px #6BA8E0;
    }
</style>