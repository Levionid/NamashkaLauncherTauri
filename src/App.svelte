<script lang="ts">
</script>

<main>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-box">
      <img class="titlebar-image" src="src-tauri/assets/namashkaLogo.png" alt="titlebarImage">
      <div class="titlebar-name">Намашка</div>
    </div>
    <div class="titlebar-button-minimize" id="titlebar-minimize">
      <span>-</span>
    </div>
    <div class="titlebar-button-close" id="titlebar-close">
      <img src="src-tauri/assets/exitButton.svg" alt="x" />
    </div>
  </div>
  <aside class="namashka-craft-tab">
    <div class="namashka-craft">
      <img class="namashka-craft-tab-image" src="src-tauri/assets/namashkaCraftTabImage.png" alt="NamashkaCraftTabImage">
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
    <button class="namashka-craft-play-button">Играть</button>
    <div class="buttons">
      <img class="folder-button" src="src-tauri/assets/folderButton.svg" alt="folderButton">
      <img class="settings-button" src="src-tauri/assets/settingsButton.svg" alt="settingsButton">
    </div>
  </aside>
</main>

<style>
</style>