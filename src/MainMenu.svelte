<script type="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import namashkaCraftTabImage from '../src-tauri/assets/namashkaCraftTabImage.png';

    export let toggleMainMenu;
    export let toggleSettingsMenu;

    async function openExplorer() {
        try {
            await invoke('open_explorer');
            console.log('Explorer opened');
        } catch (e) {
            console.error('Failed to open explorer', e);
        }
    }

    async function startMinecraftLoader() {
        const nicknameInput = document.querySelector('.namashka-craft-nickname-input-box');
        const errorFeedback = document.querySelector('.error-feedback');

        // Clear error feedback if it's already displayed
        errorFeedback.textContent = '';

        if (nicknameInput.value.trim() === '') {
            errorFeedback.textContent = 'Введите ник!';
            return;
        }

        try {
            await invoke('start_minecraft_loader');
            console.log('Minecraft Loader started');
        } catch (e) {
            errorFeedback.textContent = e; // Отображаем сообщение об ошибке из Rust
            console.error('Failed to start Minecraft Loader', e);
        }
    }
</script>

<main>
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
    <div class="namashka-craft-nickname">
        <span class="namashka-craft-nickname-text">Ник</span>
        <input class="namashka-craft-nickname-input-box" type="text" maxlength="15" placeholder="Введите ник">
    </div>
    <button class="namashka-craft-play-button" on:click={startMinecraftLoader}>Играть</button>
    <div class="error-feedback"></div>
    <div class="buttons">
        <svg on:click={openExplorer} class="folder-button" id="folder-button" viewBox="0 0 21 17" xmlns="http://www.w3.org/2000/svg">
            <path d="M8.41665 0.166687H2.16665C1.02081 0.166687 0.0937296 1.10419 0.0937296 2.25002L0.083313 14.75C0.083313 15.8959 1.02081 16.8334 2.16665 16.8334H18.8333C19.9791 16.8334 20.9166 15.8959 20.9166 14.75V4.33335C20.9166 3.18752 19.9791 2.25002 18.8333 2.25002H10.5L8.41665 0.166687Z"/>
        </svg>
        <svg on:click={toggleMainMenu} on:click={toggleSettingsMenu} class="settings-button" id="settings-button" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 21 21">
            <path id="icon_path" d="M17.9375 11.4792C17.9792 11.1667 18 10.8438 18 10.5C18 10.1667 17.9792 9.83333 17.9271 9.52083L20.0417 7.875C20.2292 7.72917 20.2813 7.44792 20.1667 7.23958L18.1667 3.78125C18.0417 3.55208 17.7813 3.47917 17.5521 3.55208L15.0625 4.55208C14.5417 4.15625 13.9896 3.82292 13.375 3.57292L13 0.927083C12.9584 0.677083 12.75 0.5 12.5 0.5H8.50003C8.25003 0.5 8.05212 0.677083 8.01045 0.927083L7.63545 3.57292C7.02087 3.82292 6.45837 4.16667 5.94795 4.55208L3.45837 3.55208C3.2292 3.46875 2.96878 3.55208 2.84378 3.78125L0.8542 7.23958C0.7292 7.45833 0.770866 7.72917 0.979199 7.875L3.09378 9.52083C3.0417 9.83333 3.00003 10.1771 3.00003 10.5C3.00003 10.8229 3.02087 11.1667 3.07295 11.4792L0.958366 13.125C0.770866 13.2708 0.718783 13.5521 0.833366 13.7604L2.83337 17.2188C2.95837 17.4479 3.21878 17.5208 3.44795 17.4479L5.93753 16.4479C6.45837 16.8437 7.01045 17.1771 7.62503 17.4271L8.00003 20.0729C8.05212 20.3229 8.25003 20.5 8.50003 20.5H12.5C12.75 20.5 12.9584 20.3229 12.9896 20.0729L13.3646 17.4271C13.9792 17.1771 14.5417 16.8437 15.0521 16.4479L17.5417 17.4479C17.7709 17.5312 18.0313 17.4479 18.1563 17.2188L20.1563 13.7604C20.2813 13.5312 20.2292 13.2708 20.0313 13.125L17.9375 11.4792ZM10.5 14.25C8.43753 14.25 6.75003 12.5625 6.75003 10.5C6.75003 8.4375 8.43753 6.75 10.5 6.75C12.5625 6.75 14.25 8.4375 14.25 10.5C14.25 12.5625 12.5625 14.25 10.5 14.25Z"/>
        </svg>
    </div>
</aside>
</main>

<style>
    main {
        display: flex;
        margin: 0;
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

    .namashka-craft{
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

    .namashka-craft-nickname {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        margin-top: 13.21vh;
    }

    .namashka-craft-nickname-text {
        color: var(--nickname-text-color);
        font-size: 12px;
    }

    .namashka-craft-nickname-input-box {
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
        cursor: pointer;
        text-shadow: 0 4px 4px var(--shadow-color);
        font-size: 28px;
        color: var(--text-color);
        cursor: pointer;
        user-select: none;
    }

    .namashka-craft-play-button:hover {
        font-size: 29px;
        background: var(--button-hover-color);
        box-shadow: 0 0 10px var(--button-hover-color);
    }

    .error-feedback {
        color: red;
        font-size: 12px;
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

    .settings-button {
        width: 2vw;
        height: 3.7vh;
        cursor: pointer;
        padding: 2px;
        fill: #6A6B77;
        transition: 0.3s;
    }

    .settings-button:hover {
        fill: #838490;
    }

    .folder-button {
        width: 2vw;
        height: 3.7vh;
        cursor: pointer;
        padding: 2px;
        fill: #6A6B77;
        transition: 0.3s;
    }

    .folder-button:hover {
        fill: #838490;
    }

    .green-text {
        color: var(--green-color);
    }
</style>