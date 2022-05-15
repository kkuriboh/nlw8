export const take_screenshot = async () => {
	const canvas = await html2canvas(document.querySelector('html'))
	return canvas.toDataURL('image/png')
}
