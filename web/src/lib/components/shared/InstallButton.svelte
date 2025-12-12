<script lang="ts">
	import Download from '@lucide/svelte/icons/download';
	import { browser } from '$app/environment';
	import { Button } from '../ui/button';

	let showInstallButton = $state(false);
	let deferredPrompt = $state<BeforeInstallPromptEvent | null>(null);
	let isInstalling = $state(false);

	interface BeforeInstallPromptEvent extends Event {
		prompt(): Promise<void>;
		userChoice: Promise<{ outcome: 'accepted' | 'dismissed'; platform: string }>;
	}

	const isAppInstalled = (): boolean => {
		if (!browser) return false;

		if (window.matchMedia('(display-mode: standalone)').matches) {
			return true;
		}

		return localStorage.getItem('pwa-installed') === '1';
	};

	// Listen for beforeinstallprompt event
	$effect(() => {
		if (!browser) return;

		// Don't show button if already installed
		if (isAppInstalled()) {
			showInstallButton = false;
			return;
		}

		const handleBeforeInstallPrompt = (e: Event) => {
			// Prevent the mini-infobar from appearing on mobile
			e.preventDefault();

			// Store the event for later use
			deferredPrompt = e as BeforeInstallPromptEvent;
			showInstallButton = true;
		};

		const handleAppInstalled = () => {
			// Hide button after successful installation
			showInstallButton = false;
			deferredPrompt = null;
			localStorage.setItem('pwa-installed', '1');
		};

		window.addEventListener('beforeinstallprompt', handleBeforeInstallPrompt);
		window.addEventListener('appinstalled', handleAppInstalled);

		return () => {
			window.removeEventListener('beforeinstallprompt', handleBeforeInstallPrompt);
			window.removeEventListener('appinstalled', handleAppInstalled);
		};
	});

	// Handle install button click
	const handleInstallClick = async () => {
		if (!deferredPrompt) return;

		isInstalling = true;

		try {
			// Show the install prompt
			await deferredPrompt.prompt();

			// Wait for the user to respond to the prompt
			const { outcome } = await deferredPrompt.userChoice;

			if (outcome === 'accepted') {
				localStorage.setItem('pwa-installed', 'true');
			}

			// Clear the deferred prompt
			deferredPrompt = null;
		} catch (error) {
			console.error('Error during installation:', error);
		} finally {
			isInstalling = false;
			showInstallButton = false;
		}
	};
</script>

{#if showInstallButton}
	<Button
		onclick={handleInstallClick}
		disabled={isInstalling}
		variant="default"
		title="Install ChordCraft as an app"
	>
		<Download size={16} />
		<span class="hidden sm:inline">
			{#if isInstalling}
				Installing...
			{:else}
				Install App
			{/if}
		</span>
		<span class="sm:hidden">Install</span>
	</Button>
{/if}
