const fileMenuItems = document.querySelectorAll('.menu-item');

function closeAllMenus() {
    document.querySelectorAll('.dropdown.show').forEach((dropdown) => {
        dropdown.classList.remove('show');
    });
}

fileMenuItems.forEach((item) => {
    item.addEventListener('click', (e) => {
        e.stopPropagation();
        
        const menuId = item.getAttribute('data-menu');
        const targetDropdown = document.getElementById(menuId);
        
        const isAlreadyOpen = targetDropdown && targetDropdown.classList.contains('show');
        
        closeAllMenus();

        if (menuId && !isAlreadyOpen) {
            targetDropdown.classList.add('show');
        }
    });
});

document.addEventListener('click', () => {
    closeAllMenus();
});

document.getElementById('action-new-file').addEventListener('click', async (e) => {
    e.stopPropagation();
    const fileName = prompt('Enter new file name:');
    if (fileName) {
        try {
            await invoke('create_file', { name: fileName });
            if (typeof loadExplorer === 'function') {
                loadExplorer();
            }
            closeAllMenus();
        } catch (error) {
            alert(error);
        }
    } else {
        closeAllMenus();
    }
});

document.getElementById('action-new-dir').addEventListener('click', async (e) => {
    e.stopPropagation();
    const dirName = prompt('Enter new directory name:');
    if (dirName) {
        try {
            await invoke('create_dir', { name: dirName });
            if (typeof loadExplorer === 'function') {
                loadExplorer();
            }
            closeAllMenus();
        } catch (error) {
            alert(error);
        }
    } else {
        closeAllMenus();
    }
});

document.getElementById('action-save').addEventListener('click', async (e) => {
    e.stopPropagation();
    if (appState.activeFilePath) {
        try {
            await invoke('save_file', { 
                path: appState.activeFilePath, 
                content: document.getElementById('editor').value 
            });
            closeAllMenus();
        } catch (error) {
            alert(error);
        }
    } else {
        closeAllMenus();
    }
});

document.getElementById('action-exit').addEventListener('click', async () => {
    await invoke('exit_app');
});

const helpModal = document.getElementById('help-modal-overlay');

document.getElementById('action-help').addEventListener('click', () => {
    closeAllMenus();
    helpModal.classList.add('show');
});

document.getElementById('close-help').addEventListener('click', () => {
    helpModal.classList.remove('show');
});

document.getElementById('help-ok-btn').addEventListener('click', () => {
    helpModal.classList.remove('show');
});

window.addEventListener('click', (e) => {
    if (e.target === helpModal) {
        helpModal.classList.remove('show');
    }
});

document.getElementById('action-undo').addEventListener('click', (e) => {
    e.stopPropagation();
    if (typeof cm !== 'undefined') {
        cm.undo();
        closeAllMenus();
    }
});