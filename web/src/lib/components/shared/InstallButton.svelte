<script lang="ts">
	import Download from '@lucide/svelte/icons/download';
	import Share from '@lucide/svelte/icons/share';
	import Plus from '@lucide/svelte/icons/plus';
	import X from '@lucide/svelte/icons/x';
	import { browser } from '$app/environment';
	import { Button } from '../ui/button';
	import * as Popover from '../ui/popover';

	let showInstallButton = $state(false);
	let showIOSInstructions = $state(false);
	let iosPopoverOpen = $state(false);
	let deferredPrompt = $state<BeforeInstallPromptEvent | null>(null);
	let isInstalling = $state(false);

	interface BeforeInstallPromptEvent extends Event {
		prompt(): Promise<void>;
		userChoice: Promise<{ outcome: 'accepted' | 'dismissed'; platform: string }>;
	}

	const isIOS = (): boolean => {
		if (!browser) return false;
		return /iPad|iPhone|iPod/.test(navigator.userAgent) && !('MSStream' in window);
	};

	const isAppInstalled = (): boolean => {
		if (!browser) return false;

		if (window.matchMedia('(display-mode: standalone)').matches) {
			return true;
		}

		return localStorage.getItem('pwa-installed') === '1';
	};

	const isIOSInstructionsDismissed = (): boolean => {
		if (!browser) return false;
		return localStorage.getItem('ios-install-dismissed') === '1';
	};

	// Listen for beforeinstallprompt event (Chromium browsers)
	// For iOS, show instructions instead
	$effect(() => {
		if (!browser) return;

		// Don't show button if already installed
		if (isAppInstalled()) {
			showInstallButton = false;
			showIOSInstructions = false;
			return;
		}

		// iOS doesn't support beforeinstallprompt, show manual instructions
		if (isIOS()) {
			showIOSInstructions = !isIOSInstructionsDismissed();
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

	// Handle install button click (Chromium)
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

	const dismissIOSInstructions = () => {
		localStorage.setItem('ios-install-dismissed', '1');
		showIOSInstructions = false;
		iosPopoverOpen = false;
	};
</script>

{#if showInstallButton}
	<!-- Standard install button for Chromium browsers -->
	<Button
		onclick={handleInstallClick}
		disabled={isInstalling}
		variant="default"
		title="Install ChordCraft as an app"
	>
		<Download size={16} />
		{#if isInstalling}
			Installing...
		{:else}
			Install App
		{/if}
	</Button>
{:else if showIOSInstructions}
	<!-- iOS-specific install instructions -->
	<Popover.Root bind:open={iosPopoverOpen}>
		<Popover.Trigger>
			{#snippet child({ props })}
				<Button {...props} variant="default" title="Install ChordCraft as an app">
					<Download size={16} />
					Install App
				</Button>
			{/snippet}
		</Popover.Trigger>
		<Popover.Content class="w-80" side="bottom" align="end">
			<div class="flex flex-col gap-3">
				<div class="flex items-start justify-between gap-2">
					<h4 class="text-sm font-semibold">Install ChordCraft</h4>
					<button
						onclick={dismissIOSInstructions}
						class="-mt-1 -mr-1 p-1 text-muted-foreground hover:text-foreground"
						aria-label="Dismiss"
					>
						<X size={16} />
					</button>
				</div>
				<p class="text-sm text-muted-foreground">
					Add this app to your home screen for quick access and offline use:
				</p>
				<ol class="space-y-2 text-sm">
					<li class="flex items-center gap-2">
						<span
							class="flex h-5 w-5 shrink-0 items-center justify-center rounded bg-muted text-xs font-medium"
							>1</span
						>
						<span class="flex items-center gap-1">
							Tap the <Share size={14} class="inline text-primary" /> Share button
						</span>
					</li>
					<li class="flex items-center gap-2">
						<span
							class="flex h-5 w-5 shrink-0 items-center justify-center rounded bg-muted text-xs font-medium"
							>2</span
						>
						<span class="flex items-center gap-1">
							Scroll and tap <Plus size={14} class="inline text-primary" /> Add to Home
							Screen
						</span>
					</li>
					<li class="flex items-center gap-2">
						<span
							class="flex h-5 w-5 shrink-0 items-center justify-center rounded bg-muted text-xs font-medium"
							>3</span
						>
						<span>Tap Add to confirm</span>
					</li>
				</ol>
				<Button variant="outline" size="sm" class="mt-1" onclick={dismissIOSInstructions}>
					Got it
				</Button>
			</div>
		</Popover.Content>
	</Popover.Root>
{/if}
