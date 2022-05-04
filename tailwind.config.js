module.exports = {
	content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
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
