import { setStyle } from 'tauri-plugin-system-bars-styles';
import type { BarStyle } from 'tauri-plugin-system-bars-styles';

const statusBarSelect = document.getElementById('status-bar') as HTMLSelectElement;
const navBarSelect = document.getElementById('nav-bar') as HTMLSelectElement;
const applyButton = document.getElementById('apply')!;
const statusEl = document.getElementById('status')!;

applyButton.addEventListener('click', async () => {
  const statusBarStyle = statusBarSelect.value as BarStyle;
  const navigationBarStyle = navBarSelect.value as BarStyle;

  try {
    await setStyle({ statusBarStyle, navigationBarStyle });
    statusEl.textContent = `Applied: status=${statusBarStyle}, nav=${navigationBarStyle}`;
  } catch (e) {
    statusEl.textContent = `Error: ${e}`;
  }
});
