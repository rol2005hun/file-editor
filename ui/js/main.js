const invoke = window.__TAURI__.invoke;

const editor = document.getElementById("editor");
const fileList = document.getElementById("file-list");
const tabsContainer = document.getElementById("tabs-container");

function renderTabs() {
    tabsContainer.innerHTML = "";
    
    for (const file of appState.openFiles) {
        const tab = document.createElement("div");
        tab.className = file.path === appState.activeFilePath ? "tab active" : "tab";
        
        const title = document.createElement("span");
        title.innerText = file.name;
        title.addEventListener("click", () => {
            appState.activeFilePath = file.path;
            editor.value = file.content;
            renderTabs();
        });

        const closeBtn = document.createElement("span");
        closeBtn.className = "tab-close";
        closeBtn.innerText = "×";
        closeBtn.addEventListener("click", (e) => {
            e.stopPropagation();
            removeTab(file.path);
            const newActive = appState.openFiles.find((f) => f.path === appState.activeFilePath);
            editor.value = newActive ? newActive.content : "";
            renderTabs();
        });

        tab.appendChild(title);
        tab.appendChild(closeBtn);
        tabsContainer.appendChild(tab);
    }
}

async function loadExplorer() {
    const items = await invoke("get_explorer_items");
    fileList.innerHTML = "";
    
    for (const item of items) {
        const li = document.createElement("li");
        li.className = "file-item";
        
        const icon = item.is_dir ? "📁" : "📄";
        li.innerText = icon + " " + item.name;
        
        li.addEventListener("click", async () => {
            if (item.is_dir) {
                await invoke("open_path", { path: item.path });
                loadExplorer();
            } else {
                const content = await invoke("read_file", { path: item.path });
                addTab(item.name, item.path, content);
                editor.value = content;
                renderTabs();
            }
        });
        
        fileList.appendChild(li);
    }
}

editor.addEventListener("input", () => {
    updateActiveContent(editor.value);
});

editor.addEventListener("keydown", async (e) => {
    if (e.ctrlKey && e.key === "s") {
        e.preventDefault();
        if (appState.activeFilePath) {
            await invoke("save_file", { path: appState.activeFilePath, content: editor.value });
        }
    }
});

window.addEventListener("DOMContentLoaded", () => {
    loadExplorer();
});