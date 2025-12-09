/**
 * Example data for quick testing
 */

export const exampleChords = ['C', 'Cmaj7', 'Fm7', 'Abm7', 'F#7b9', 'Dsus4'];

/** Guitar tab examples (6 strings) */
export const guitarTabs = [
	{ tab: 'x32010', label: 'C' },
	{ tab: '022100', label: 'E' },
	{ tab: '133211', label: 'F (barre)' },
	{ tab: 'xx0232', label: 'D' },
	{ tab: '320003', label: 'G' },
];

/** Ukulele tab examples (4 strings, GCEA tuning) */
export const ukuleleTabs = [
	{ tab: '0003', label: 'C' },
	{ tab: '2000', label: 'Am' },
	{ tab: '0232', label: 'G' },
	{ tab: '2010', label: 'F' },
	{ tab: '4442', label: 'E' },
];

export const commonProgressions = [
	{ name: 'I-IV-V in C', chords: 'C F G' },
	{ name: 'I-V-vi-IV', chords: 'C G Am F' },
	{ name: 'ii-V-I Jazz', chords: 'Dm7 G7 Cmaj7' },
	{ name: 'I-vi-IV-V', chords: 'C Am F G' },
	{ name: '12-Bar Blues', chords: 'C7 F7 C7 G7' },
	{ name: 'Coltrane Changes', chords: 'Cmaj7 Ebmaj7 Abmaj7 Bmaj7' },
];
