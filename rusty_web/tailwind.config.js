module.exports = {
	content: [
		'./src/**/*.rs',
		'./index.html',
		'./src/**/*.html',
		'./src/**/*.css',
	],
	theme: {
		extend: {
			colors: {
				brand: {
					300: '#996DFF',
					500: '#8257E6',
				},
			},
			borderRadius: {
				md: '4px',
			},
		},
	},
	plugins: [require('@tailwindcss/forms'), require('tailwind-scrollbar')],
}
