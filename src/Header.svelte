<script type="ts">
    import { appWindow } from "@tauri-apps/api/window";

    import titlebarImage from '../src-tauri/assets/namashkaLogo.png';
    import exitButton from '../src-tauri/assets/exitButton.svg';

    function openGit() {
        window.open("https://github.com/Levionid/NamashkaLauncherTauri");
    }

    function minimizeButton() {
        appWindow.minimize();
    }

    function closeButton() {
        appWindow.close();
    }
</script>

<main>
    <div data-tauri-drag-region class="titlebar">
        <div class="titlebar-box">
          <img class="titlebar-image" src={titlebarImage} alt="titlebarImage">
          <div class="titlebar-name" on:click={openGit}>Намашка</div>
        </div>
        <button class="titlebar-button-minimize" id="titlebar-minimize" on:click={minimizeButton}>-</button>
        <button class="titlebar-button-close" id="titlebar-close" on:click={closeButton}>
          <img src={exitButton} alt="x">
        </button>
      </div>
</main>

<style>
    .titlebar {
        width: 100vw;
        height: var(--header-height);
        background-color: var(--header-color);
        display: flex;
        justify-content: flex-end;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        user-select: none;
        z-index: 1;
    }

    .titlebar-box {
        position: absolute;
        display: flex;
        flex-direction: row;
        align-items: center;
        left: 0;
        height: var(--header-height);
    }

    .titlebar-image {
        height: 45px;
        margin: 5px;
        pointer-events: none;
    }
    
    .titlebar-name {
        color: #fff;
        text-decoration: none;
        font-size: 28px;
        text-shadow: 0 4px 4px var(--shadow-color);
        cursor: pointer;
        transition: 0.3s;
    }
    
    .titlebar-name:hover {
        text-shadow: 0 0 10px #fff;
    }

    .titlebar-button-minimize {
        position: relative;
        width: 40px;
        height: 29px;
        background-color: #00000000;
        border-style: none;
        transition: 0.3s;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        margin-right: 4px;
        color: #fff;
        font-size: 34px;
        bottom: 3px;
        text-shadow: 0 4px 4px var(--shadow-color);
    }

    .titlebar-button-minimize:hover {
        background-color: #2D3239;
        box-shadow: 0 0 10px #2D3239;
    }

    .titlebar-button-close {
        width: 40px;
        height: 29px;
        background-color: #BD2626;
        border-style: none;
        border-radius: 10px 0px 0 10px;
        box-shadow: 0 4px 4px var(--shadow-color);
        transition: 0.3s;
        display: inline-flex;
        justify-content: center;
        align-items: center;
    }

    .titlebar-button-close img {
        user-select: none;
        pointer-events: none;
    }

    .titlebar-button-close:hover {
        background-color: #D94040;
        box-shadow: 0 0 10px #D94040;
    }
</style>