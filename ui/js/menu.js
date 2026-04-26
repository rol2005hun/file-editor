const fileMenuItems = document.querySelectorAll('.menu-item');

fileMenuItems.forEach((item) => {
    item.addEventListener('click', (e) => {
        e.stopPropagation();
        
        document.querySelectorAll('.dropdown.show').forEach((dropdown) => {
            dropdown.classList.remove('show');
        });

        const menuId = item.getAttribute('data-menu');
        if (menuId) {
            document.getElementById(menuId).classList.toggle('show');
        }
    });
});

document.addEventListener('click', () => {
    document.querySelectorAll('.dropdown.show').forEach((dropdown) => {
        dropdown.classList.remove('show');
    });
});

document.getElementById('action-new-file').addEventListener('click', async () => {
    const fileName = prompt('Enter new file name:');
    if (fileName) {
        try {
            await invoke('create_file', { name: fileName });
            if (typeof loadExplorer === 'function') {
                loadExplorer();
            }
        } catch (error) {
            alert(error);
        }
    }
});

document.getElementById('action-new-dir').addEventListener('click', async () => {
    const dirName = prompt('Enter new directory name:');
    if (dirName) {
        try {
            await invoke('create_dir', { name: dirName });
            if (typeof loadExplorer === 'function') {
                loadExplorer();
            }
        } catch (error) {
            alert(error);
        }
    }
});

document.getElementById('action-save').addEventListener('click', async () => {
    if (appState.activeFilePath) {
        try {
            await invoke('save_file', { 
                path: appState.activeFilePath, 
                content: document.getElementById('editor').value 
            });
        } catch (error) {
            alert(error);
        }
    }
});

document.getElementById('action-exit').addEventListener('click', async () => {
    await invoke('exit_app');
});