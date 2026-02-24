import { invoke } from '@tauri-apps/api/core';

export type BarStyle = 'light' | 'dark';

export interface SetStyleOptions {
	statusBarStyle: BarStyle;
	navigationBarStyle: BarStyle;
}

export async function setStyle(options: SetStyleOptions): Promise<void> {
	await invoke('plugin:system-bars-styles|set_style', {
		statusBarStyle: options.statusBarStyle,
		navigationBarStyle: options.navigationBarStyle,
	});
}
