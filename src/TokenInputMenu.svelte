<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    export let toggleTokenInput: () => void;
    export let toggleLauncherOptionsExists: () => void;
    export let setInitialSelectedElement: () => void;

    export let token: string;
    export let minJvmArgument: string;
    export let maxJvmArgument: string;
    export let showTokenInput: boolean;

    async function saveLauncherOptions() {
        minJvmArgument = "0";
        maxJvmArgument = "3000";
        const tokenInput = document.querySelector('.namashka-craft-token-input') as HTMLInputElement;
        token = tokenInput.value.trim();
        console.log("Token:", token);
        checkTokenInput();

        if (token) {
            try {
                await invoke('save_launcher_options', {
                    username: "",
                    token: token,
                    minJvmArgument: minJvmArgument,
                    maxJvmArgument: maxJvmArgument 
                });
                
                console.log('Launcher options saved successfully.');

                toggleLauncherOptionsExists();
                toggleTokenInput();
                setInitialSelectedElement();
            } catch (error) {
                console.error('Failed to save launcher options:', error);
            }
        }
    }

    async function checkTokenInput() {
        const tokenInput = document.querySelector('.namashka-craft-token-input') as HTMLInputElement;
        const errorFeedback = document.querySelector('.namashka-craft-token-error-feedback') as HTMLElement;

        if (tokenInput && errorFeedback) {
            errorFeedback.textContent = '';

            if (tokenInput.value.trim() === '') {
                errorFeedback.textContent = 'Введите токен!';
                return false;
            }
        }

        return true;
    }
</script>

<main>
    <div class="namashka-craft-token {showTokenInput ? 'active' : ''}">
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
</main>

<style>
    .namashka-craft-token {
        margin-top: 100vh;
        box-shadow: 0 0 10px var(--shadow-color);
        border-radius: 10px;
        transition: margin-top 1s;
    }

    .namashka-craft-token.active {
        margin-top: 0;
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
        transition: color 0.3s, box-shadow 0.3s, font-size 0.3s, background-color 0.3s;
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
        background-color: #6BA8E0;
        box-shadow: 0 0 10px #6BA8E0;
    }

    .namashka-craft-token-error-feedback {
        color: red;
        font-size: 12px;
        font-weight: 500;
        text-align: center;
        margin-top: 5px;
    }

    .namashka-craft-token input {
        font-family: "Inter", sans-serif;
        font-weight: 600;
        font-optical-sizing: auto;
        font-style: normal;
        font-variation-settings: "slnt" 0;
    }
</style>