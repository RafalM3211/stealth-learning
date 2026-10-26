const { invoke } = window.__TAURI__.core;
const { getCurrentWindow } = window.__TAURI__.window;

const { LogicalSize } = window.__TAURI__.dpi;
let imageBase64 = null;
const textEl = document.getElementById('text');
const preview = document.getElementById('preview');
const result = document.getElementById('result');
const sendBtn = document.getElementById('send');

document.getElementById('close').onclick = () => getCurrentWindow().close();

window.addEventListener('paste', (e) => {
  for (const item of e.clipboardData.items) {
    if (item.type.startsWith('image/')) {
      const reader = new FileReader();
      reader.onload = ev => {
        imageBase64 = ev.target.result;
        preview.src = imageBase64;
        preview.style.display = 'block';
      };
      reader.readAsDataURL(item.getAsFile());
    }
  }
});

sendBtn.onclick = async () => {
  sendBtn.textContent = 'Myślę...';
  sendBtn.disabled = true;
  result.textContent = '';
  try {
    const answer = await invoke('solve_task', {
      question: textEl.value,
      imageBase64: imageBase64
    });
    result.textContent = answer;
  } catch (err) {
    result.textContent = 'Błąd: ' + err;
  } finally {
    sendBtn.textContent = 'Wyjaśnij zadanie';
    sendBtn.disabled = false;
  }
};

let isCollapsed = false;
let lastSize = null;
document.getElementById('minimize').onclick = async () => {
  const win = getCurrentWindow();
  const content = document.getElementById('content');
  const btn = document.getElementById('minimize');
  if (!isCollapsed) {
    lastSize = await win.innerSize();
    content.style.opacity = '0';
    await new Promise(r => setTimeout(r, 120));
    content.style.display = 'none';
    await win.setSize(new LogicalSize(120, 130));
    btn.textContent = '☐';
    isCollapsed = true;
  } else {
    await win.setSize(new LogicalSize(120, 130));
    await new Promise(r => setTimeout(r, 50));
    content.style.display = 'block';
    requestAnimationFrame(() => content.style.opacity = '1');
    btn.textContent = '—';
    isCollapsed = false;
  }
};