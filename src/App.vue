<script setup lang="ts">
    import { ref } from "vue";
    //import { invoke } from "@tauri-apps/api/core";
    import { open } from '@tauri-apps/plugin-dialog';

    const filePathVM = ref("C:\\Users\\muzud\\OneDrive\\ドキュメント\\temp\\temp.csv");
    const selectedItemVM = ref()
    const textVM = ref()

    async function onReadButtonClicked() {
        console.log("［Read］ボタンを押したぜ。")
        // Open a dialog
        const filePath = await open({
            multiple: false,
            directory: false,  // ファイルを開く。
            defaultPath: filePathVM.value
        });
        filePathVM.value = filePath ?? "";  // dirPath がヌルなら空文字列に変換。
        textVM.value = "テスト中１ read"
    }

    async function onWriteButtonClicked() {
        console.log("［Write］ボタンを押したぜ。")
        textVM.value = "テスト中２ write"
    }

    async function onExecuteButtonClicked() {
        console.log("［Execute］ボタンを押したぜ。")
        textVM.value = "テスト中３ Execute"
    }
</script>

<template>
    <main class="container">
        <div class="row">
            <input style="width:60%; height: 10vh;" :value="filePathVM">
            <button @click="onReadButtonClicked" style="width:20%; height: 10vh;">Read</button>
            <button @click="onWriteButtonClicked" style="width:20%; height: 10vh;">Write</button>
        </div>
        <div class="row">
            <select v-model="selectedItemVM">
                <option value="都道府県スプリット">都道府県スプリット</option>
            </select>
            <p>選ばれた値: {{ selectedItemVM || 'まだ選んでないよ！' }}</p>
            <button @click="onExecuteButtonClicked" style="width:20%; height: 10vh;">Execute</button>
        </div>
        <textarea style="width:100%; height:80vh;" v-model="textVM"></textarea>
    </main>
</template>

<style>
:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}

.container {
    height: 95vh;
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
}

.row {
    display: flex;
    justify-content: center;
}

input,
button {
    border-radius: 8px;
    border: 1px solid transparent;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
    padding: 0.6em 1.2em;
    cursor: pointer;
}

button:hover {
    border-color: #396cd8;
}
button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
}

input,
button {
    outline: none;
}

@media (prefers-color-scheme: dark) {
    :root {
        color: #f6f6f6;
        background-color: #2f2f2f;
    }

    input,
    button {
        color: #ffffff;
        background-color: #0f0f0f98;
    }
    button:active {
        background-color: #0f0f0f69;
    }
}
</style>