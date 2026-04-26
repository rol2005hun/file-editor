const appState = {
    openFiles: [],
    activeFilePath: null
};

function addTab(name, path, content) {
    const existing = appState.openFiles.find((f) => f.path === path);
    if (!existing) {
        appState.openFiles.push({ name: name, path: path, content: content });
    }
    appState.activeFilePath = path;
}

function removeTab(path) {
    appState.openFiles = appState.openFiles.filter((f) => f.path !== path);
    if (appState.activeFilePath === path) {
        appState.activeFilePath = appState.openFiles.length > 0 ? appState.openFiles[0].path : null;
    }
}

function updateActiveContent(content) {
    const active = appState.openFiles.find((f) => f.path === appState.activeFilePath);
    if (active) {
        active.content = content;
    }
}