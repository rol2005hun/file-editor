const invoke = window.__TAURI__.invoke;

const fileList = document.getElementById('file-list');
const tabsContainer = document.getElementById('tabs-container');

const cm = CodeMirror(document.getElementById('real-editor'), {
    lineNumbers: true,
    theme: 'vscode-dark',
    mode: 'rust',
    indentUnit: 4,
    tabSize: 4,
    lineWrapping: true,
    extraKeys: { 'Ctrl-S': function(instance) { saveActiveFile(); } }
});

function getModeByPath(path) {
    const ext = path.split('.').pop().toLowerCase();
    const modes = {
        'rs': 'rust',
        'js': 'javascript',
        'ts': 'javascript',
        'css': 'css',
        'toml': 'toml',
        'html': 'xml',
        'md': 'markdown'
    };
    return modes[ext] || 'text/plain';
}

function renderTabs() {
    tabsContainer.innerHTML = '';
    for (const file of appState.openFiles) {
        const tab = document.createElement('div');
        tab.className = file.path === appState.activeFilePath ? 'tab active' : 'tab';
        
        const title = document.createElement('span');
        title.innerText = file.name;
        title.addEventListener('click', () => {
            appState.activeFilePath = file.path;
            cm.setValue(file.content);
            cm.setOption('mode', getModeByPath(file.path));
            renderTabs();
        });

        const closeBtn = document.createElement('span');
        closeBtn.className = 'tab-close';
        closeBtn.innerText = '×';
        closeBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            removeTab(file.path);
            const newActive = appState.openFiles.find((f) => f.path === appState.activeFilePath);
            cm.setValue(newActive ? newActive.content : '');
            if (newActive) cm.setOption('mode', getModeByPath(newActive.path));
            renderTabs();
        });

        tab.appendChild(title);
        tab.appendChild(closeBtn);
        tabsContainer.appendChild(tab);
    }
}

async function saveActiveFile() {
    if (appState.activeFilePath) {
        await invoke('save_file', { 
            path: appState.activeFilePath, 
            content: cm.getValue() 
        });
    }
}

async function loadExplorer() {
    const items = await invoke('get_explorer_items');
    fileList.innerHTML = '';
    for (const item of items) {
        const li = document.createElement('li');
        li.className = 'file-item';
        const icon = item.is_dir ? '📁' : '📄';
        li.innerText = icon + ' ' + item.name;
        
        li.addEventListener('click', async () => {
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
        });
        fileList.appendChild(li);
    }
}

cm.on('change', () => {
    updateActiveContent(cm.getValue());
});

window.addEventListener('keydown', async (e) => {
    if (e.ctrlKey && e.key === 's') {
        e.preventDefault();
        saveActiveFile();
    }
});

window.addEventListener('DOMContentLoaded', () => {
    loadExplorer();
});