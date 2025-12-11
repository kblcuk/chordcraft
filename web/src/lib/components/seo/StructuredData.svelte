<script lang="ts">
	// The only component where we should be ok using this
	/* eslint-disable svelte/no-at-html-tags */
	interface Props {
		type: 'WebApplication' | 'WebPage';
		name?: string;
		url?: string;
		description?: string;
	}

	let { type, name, url, description }: Props = $props();

	// Use $derived to create reactive schema based on props
	const schema = $derived(
		type === 'WebApplication'
			? {
					'@context': 'https://schema.org',
					'@type': 'WebApplication',
					name: 'ChordCraft',
					url: 'https://chordcraft.rocks',
					description:
						'A multi-platform tool for bidirectional chord-fingering conversion, supporting guitar, ukulele, and other stringed instruments.',
					applicationCategory: 'MusicApplication',
					creator: {
						'@type': 'Person',
						name: 'kblcuk',
					},
					featureList: [
						'Find chord fingerings for guitar and ukulele',
						'Identify chords from tab notation',
						'Optimize chord progressions for smooth transitions',
						'Multiple voicing types (core, full, jazzy)',
						'Capo support',
						'Position preferences',
						'Playing context (solo/band)',
					],
				}
			: {
					'@context': 'https://schema.org',
					'@type': 'WebPage',
					name: name || 'ChordCraft',
					url: url || 'https://chordcraft.rocks',
					description:
						description ||
						'A multi-platform tool for bidirectional chord-fingering conversion, supporting guitar, ukulele, and other stringed instruments.',
					isPartOf: {
						'@type': 'WebSite',
						name: 'ChordCraft',
						url: 'https://chordcraft.rocks',
					},
				}
	);
</script>

<svelte:head>
	{@html `<\u{73}script type="application/ld+json">${JSON.stringify(schema)}</script>`}
</svelte:head>
