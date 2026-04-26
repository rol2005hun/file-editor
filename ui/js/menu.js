const invoke = window.__TAURI__.invoke;
const fileList = document.getElementById('file-list');
const tabsContainer = document.getElementById('tabs-container');

const cm = CodeMirror(document.getElementById('real-editor'), {
    lineNumbers: true,
    theme: 'vscode-dark',
    mode: 'rust',
    indentUnit: 4,
    tabSize: 4,
    lineWrapping: true
});

function getModeByPath(path) {
    const ext = path.split('.').pop().toLowerCase();
    const map = {
        'rs': 'rust',
        'js': 'javascript',
        'ts': 'javascript',
        'css': 'css',
        'html': 'xml',
        'toml': 'toml'
    };
    return map[ext] || 'text/plain';
}

function renderTabs() {
    tabsContainer.innerHTML = '';
    for (const file of appState.openFiles) {
        const tab = document.createElement('div');
        tab.className = file.path === appState.activeFilePath ? 'tab active' : 'tab';
        
        const title = document.createElement('span');
        title.innerText = file.name;
        title.onclick = () => {
            appState.activeFilePath = file.path;
            cm.setValue(file.content);
            cm.setOption('mode', getModeByPath(file.path));
            renderTabs();
        };

        const closeBtn = document.createElement('span');
        closeBtn.className = 'tab-close';
        closeBtn.innerText = '×';
        closeBtn.onclick = (e) => {
            e.stopPropagation();
            removeTab(file.path);
            const newActive = appState.openFiles.find((f) => f.path === appState.activeFilePath);
            cm.setValue(newActive ? newActive.content : '');
            if (newActive) cm.setOption('mode', getModeByPath(newActive.path));
            renderTabs();
        };

        tab.appendChild(title);
        tab.appendChild(closeBtn);
        tabsContainer.appendChild(tab);
    }
}

async function loadExplorer() {
    try {
        const items = await invoke('get_explorer_items');
        fileList.innerHTML = '';
        for (const item of items) {
            const li = document.createElement('li');
            li.className = 'file-item';
            li.innerText = (item.is_dir ? '📁 ' : '📄 ') + item.name;
            
            li.onclick = async () => {
                if (item.is_dir) {
                    await invoke('open_path', { path: item.path });
                    loadExplorer();
                } else {
                    const content = await invoke('read_file', { path: item.path });
                    addTab(item.name, item.path, content);
                    cm.setValue(content);
                    cm.setOption('mode', getModeByPath(item.path));
                    renderTabs();
                }
            };
            fileList.appendChild(li);
        }
    } catch (e) {
        console.error(e);
    }
}

cm.on('change', () => {
    updateActiveContent(cm.getValue());
});

async function saveFile() {
    if (appState.activeFilePath) {
        await invoke('save_file', { 
            path: appState.activeFilePath, 
            content: cm.getValue() 
        });
    }
}

window.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 's') {
        e.preventDefault();
        saveFile();
    }
});

window.addEventListener('DOMContentLoaded', loadExplorer);