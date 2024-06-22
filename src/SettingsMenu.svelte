<script lang="ts">
    export let toggleMainMenu: () => void;
    export let toggleSettingsMenu: () => void;

    function handleAccept() {
        toggleMainMenu();
        toggleSettingsMenu();
    }

    function handleCancel() {
        toggleMainMenu();
        toggleSettingsMenu();
    }
</script>

<main>
    <div class="settings-block">
        <div class="token">
            <div>Токен</div>
            <input type="text" maxlength="70" placeholder="Введите токен">
        </div>

        <div class="ram">
            <div>Оперативная память (MB)</div>
            <div class="ram-input">
                Min
                <input class="min" type="text" maxlength="70">
                <span>-</span>  
                Max
                <input class="max" type="text" maxlength="70">
            </div>
        </div>
    </div>
    <div class="buttons">
        <button on:click={handleCancel} class="cancel">Отмена</button>
        <button on:click={handleAccept} class="accept">Принять</button>
    </div>
</main>

<style>
    main {
        display: flex;
        margin: 0;
    }
    
    .settings-block {
        margin-left: 2.92vw;
        display: flex;
        flex-direction: column;
        align-items: start;
    }

    .token {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: start;
        margin-top: calc(10.56vh + var(--header-height));
    }

    .token div {
        color: #9A9A9A;
        font-size: 12px;
        font-weight: 600;
        margin-bottom: 5px;
    }

    .token input {
        width: 52.08vw;
        height: 4.29vh;
        color: #fff;
        background-color: #222628;
        border: none;
        border-radius: 6px;
    }

    .ram {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: start;
        margin-top: 3.89vh;
    }

    .ram div {
        color: #9A9A9A;
        font-size: 12px;
        font-weight: 600;
        margin-bottom: 5px;
    }

    .ram-input {
        display: flex;
        flex-direction: row;
        align-items: center;
        font-size: 12px;
    }

    .ram-input span {
        font-size: 24px;
        margin: 0 10px 0 10px;
    }

    .min {
        width: 6.25vw;
        height: 4.29vh;
        color: #fff;
        text-align: center;
        background-color: #222628;
        border: none;
        border-radius: 6px;
        margin: 0 4px 0 4px;
    }

    .max {
        width: 6.25vw;
        height: 4.29vh;
        color: #fff;
        text-align: center;
        background-color: #222628;
        border: none;
        border-radius: 6px;
        margin: 0 4px 0 4px;
    }

    .buttons {
        position: fixed;
        display: flex;
        flex-direction: row;
        align-items: center;
        right: 1.35vw;
        bottom: 2.41vh;
    }
    
    .cancel {
        width: 13.02vw;
        height: 7.03vh;
        background-color: rgba(0, 0, 0, 0);
        color: #4390D8;
        font-size: 18px;
        user-select: none;
        text-shadow: 0 4px 4px var(--shadow-color);
        border-radius: 10px;
        border-style: solid;
        border-width: 2px;
        border-color: #4390D8;
        margin-right: 1.67vw;
        transition: 0.3s;
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
        text-shadow: 0 4px 4px var(--shadow-color);
        border: none;
        border-radius: 10px;
        transition: 0.3s;
    }

    .accept:hover {
        background-color: #6BA8E0;
        font-size: 19px;
        border-color: #6BA8E0;
        box-shadow: 0 0px 10px #6BA8E0;
    }
</style>