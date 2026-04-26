const fileMenuItems = document.querySelectorAll('.menu-item');
const helpModal = document.getElementById('help-modal-overlay');
const explorerCtx = document.getElementById('explorer-context-menu');
const editorCtx = document.getElementById('editor-context-menu');

function closeAllMenus() {
    document.querySelectorAll('.dropdown.show').forEach(d => d.classList.remove('show'));
    explorerCtx.classList.remove('show');
    editorCtx.classList.remove('show');
}

fileMenuItems.forEach(item => {
    item.addEventListener('click', e => {
        e.stopPropagation();
        const menuId = item.getAttribute('data-menu');
        const target = document.getElementById(menuId);
        const isOpen = target && target.classList.contains('show');
        closeAllMenus();
        if (menuId && !isOpen) target.classList.add('show');
    });
});

document.addEventListener('contextmenu', e => {
    e.preventDefault();
    closeAllMenus();
    let menu = null;
    if (e.target.closest('#sidebar')) menu = explorerCtx;
    else if (e.target.closest('#real-editor')) menu = editorCtx;
    
    if (menu) {
        menu.style.top = `${e.pageY}px`;
        menu.style.left = `${e.pageX}px`;
        menu.classList.add('show');
    }
});

document.addEventListener('click', closeAllMenus);

async function handleNewFile() {
    const n = prompt('New file name:');
    if (n) {
        await invoke('create_file', { name: n });
        loadExplorer();
    }
}

async function handleNewDir() {
    const n = prompt('New directory name:');
    if (n) {
        await invoke('create_dir', { name: n });
        loadExplorer();
    }
}

document.getElementById('action-new-file').onclick = handleNewFile;
document.getElementById('ctx-new-file').onclick = handleNewFile;
document.getElementById('action-new-dir').onclick = handleNewDir;
document.getElementById('ctx-new-dir').onclick = handleNewDir;
document.getElementById('action-save').onclick = saveFile;
document.getElementById('ctx-save').onclick = saveFile;
document.getElementById('action-undo').onclick = () => cm.undo();
document.getElementById('ctx-undo').onclick = () => cm.undo();
document.getElementById('action-exit').onclick = () => invoke('exit_app');

document.getElementById('action-help').onclick = () => helpModal.classList.add('show');
document.getElementById('close-help').onclick = () => helpModal.classList.remove('show');
document.getElementById('help-ok-btn').onclick = () => helpModal.classList.remove('show');