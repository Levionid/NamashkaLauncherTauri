<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    import TokenInputMenu from './TokenInputMenu.svelte';

    import namashkaCraftImage1201 from '../src-tauri/assets/namashkaCraftImage1201.png';
    import namashkaCraftImageLite121 from '../src-tauri/assets/namashkaCraftImageLite121.png';
    import namashkaCraftImageMix121 from '../src-tauri/assets/namashkaCraftImageMix121.png';

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

    let selectedImage: string = namashkaCraftImage1201;
    let selectedName: string = "NamashkaCraft";
    let selectedDescription: string = "Fabric 1.20.1";
    let selectedElement: null = null;

    async function openExplorer() {
        try {
            await invoke('open_explorer');
            console.log('Explorer opened');
        } catch (e) {
            console.error('Failed to open explorer', e);
        }
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

    async function selectItem(image: string, name: string, description: string, event) {
        // Assign values to selected variables
        selectedImage = image;
        selectedName = name;
        selectedDescription = description;

        // Remove styles from previously selected element
        if (selectedElement) {
            selectedElement.querySelector('.namashka-craft-name').style.color = '';
            selectedElement.querySelector('img').style.boxShadow = '';
        }

        // Get the current target element
        const target = event.currentTarget;

        // Set styles for the newly selected element
        target.querySelector('.namashka-craft-name').style.color = 'var(--green-text)';
        target.querySelector('img').style.boxShadow = '0 4px 10px var(--green-text)';

        // Update the selectedElement variable
        selectedElement = target;
    }

    // Use this function to set the initial selected element
    async function setInitialSelectedElement() {
        const initialElement = document.querySelector(".main-img-content.initial-selected");
        if (initialElement) {
            selectItem(
                namashkaCraftImage1201,
                "NamashkaCraft",
                "сборка на версии <span style=\"color: var(--green-text)\">Fabric 1.20.1</span><br>содержит в себе <span style=\"color: var(--green-text)\">>100 модов</span><br>сборка <span style=\"color: var(--green-text)\">оптимизирована</span>",
                { currentTarget: initialElement }
            );
        }
    }

    onMount(() => {
        checkLauncherOptions().then(() => {
            setInitialSelectedElement();
        });
    });
</script>

<main>
    <div class="main-content">
        <div class="authorization-button-container {!launcherOptionsExists ? 'active' : ''}">
            <button class="namashka-craft-authorization-button" on:click={toggleTokenInput}>Ввести токен</button>
        </div>
        <div class="main-img-content-container {launcherOptionsExists ? 'active' : ''}">
            <div class="main-img-content initial-selected" on:click={(event) => selectItem(namashkaCraftImage1201, "NamashkaCraft", "сборка на версии <span style=\"color: var(--green-text)\">Fabric 1.20.1</span><br>содержит в себе <span style=\"color: var(--green-text)\">>100 модов</span><br>сборка <span style=\"color: var(--green-text)\">оптимизирована</span>", event)}>
                <img src={namashkaCraftImage1201} alt="namashkaCraftImage1201">
                <div class="namashka-craft-name">NamashkaCraft</div>
                <div class="namashka-craft-description">Fabric 1.20.1</div>
            </div>
            <div class="main-img-content" on:click={(event) => selectItem(namashkaCraftImageMix121, "Namashka Mix", "сборка на версии <span style=\"color: var(--green-text)\">Fabric 1.21</span><br>сборка содержит <span style=\"color: var(--green-text)\">моды</span><br>для <span style=\"color: var(--green-text)\">улучшения геймлея</span><br>сборка <span style=\"color: var(--green-text)\">оптимизирована</span>", event)}>
                <img src={namashkaCraftImageMix121} alt="namashkaCraftImageMix121">
                <div class="namashka-craft-name">Namashka Mix</div>
                <div class="namashka-craft-description">Fabric 1.21</div>
            </div>
            <div class="main-img-content" on:click={(event) => selectItem(namashkaCraftImageLite121, "Namashka Lite", "сборка на версии <span style=\"color: var(--green-text)\">Fabric 1.21</span><br>сборка <span style=\"color: var(--green-text)\">оптимизирована</span>", event)}>
                <img src={namashkaCraftImageLite121} alt="namashkaCraftImageLite121">
                <div class="namashka-craft-name">Namashka Lite</div>
                <div class="namashka-craft-description">Fabric 1.21</div>
            </div>
        </div>
    </div>
    <div class="overlay {showTokenInput ? 'active' : ''}">
        <TokenInputMenu {toggleTokenInput} {checkLauncherOptions} {setInitialSelectedElement} />
    </div>
    <aside class="namashka-craft-tab">
        <div class="namashka-craft">
            <img class="namashka-craft-tab-image" src={selectedImage} alt="Selected Image">
            <span class="namashka-craft-name">{selectedName}</span>
            <span class="namashka-craft-description">
                {@html selectedDescription}
            </span>
        </div>
        <div class="namashka-craft-user-box">
            <div class="namashka-craft-username">
                <span class="namashka-craft-username-text">Ник</span>
                <input class="namashka-craft-username-input-box" type="text" maxlength="15" placeholder="Введите ник" value={username} disabled={!launcherOptionsExists}>
            </div>
            <button class="namashka-craft-play-button" on:click={startMinecraftLoader} on:click={saveUsername} disabled={!launcherOptionsExists}>Играть</button>
            <div class="error-feedback"></div>
        </div>
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
    :root {
        --green-text: #3ECD5E;
    }

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
    .error-feedback {
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

    .main-img-content-container {
        position: absolute;
        width: 100%;
        height: 100%;
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        padding: 20px;
        box-sizing: border-box;
    }

    .main-content img {
        width: 250px;
        height: 140px;
        margin: 0 auto;
        border-radius: 15px;
        box-shadow: 0 4px 4px var(--shadow-color);
        transition: 0.3s;
    }

    .main-content img:hover {
        width: calc(250px * 1.1);
        height: calc(140px * 1.1);
    }
    
    .main-img-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
        user-select: none;
    }

    .authorization-button-container {
        position: absolute;
        width: 100%;
        height: 100%;
        opacity: 0;
        visibility: hidden;
    }

    .authorization-button-container.active {
        opacity: 1;
        visibility: visible;
    }

    .namashka-craft-authorization-button {
        position: relative;
        width: 258px;
        height: 53px;
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
        transition: color 0.3s, box-shadow 0.3s, font-size 0.3s;
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
        transition: color 0.3s;
    }

    .namashka-craft-description {
        color: var(--secondary-text-color);
        font-size: 13px;
        text-shadow: 0 4px 4px var(--shadow-color);
    }

    .namashka-craft-user-box {
        position: absolute;
        top: 60vh;
    }

    .namashka-craft-username {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
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
        transition: color 0.3s, box-shadow 0.3s, font-size 0.3s;
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

    .error-feedback {
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
        transition: fill 0.3s;
    }
    
    .button-container:enabled:hover svg {
        cursor: pointer;
        fill: #838490;
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
        opacity: 0;
        visibility: hidden;
        transition: opacity 0.3s, background-color 0.3s;
    }

    .overlay.active {
        opacity: 1;
        visibility: visible;
        backdrop-filter: blur(5px);
        background-color: rgba(0, 0, 0, 0.25);
    }

    .main-img-content-container {
        opacity: 0;
        visibility: hidden;
    }

    .main-img-content-container.active {
        opacity: 1;
        visibility: visible;
    }
</style>