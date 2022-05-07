module.exports = {
    purge: {
        mode: "all",
        content: [
            "./src/**/*.rs",
            "./index.html",
            "./src/**/*.html",
            "./src/**/*.css",
        ],
    },
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
    variants: {},
    plugins: [],
};
