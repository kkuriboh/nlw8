import { StatusBar } from 'expo-status-bar'
import { View } from 'react-native'

import {
	useFonts,
	Inter_400Regular,
	Inter_500Medium,
} from '@expo-google-fonts/inter'

import Widget from './src/components/Widget'
import { theme } from './src/theme'
import AppLoading from 'expo-app-loading'

export default function App() {
	const [fontsloaded] = useFonts({
		Inter_400Regular,
		Inter_500Medium,
	})

	if (!fontsloaded) return <AppLoading />

	return (
		<View
			style={{
				flex: 1,
				backgroundColor: theme.colors.background,
			}}
		>
			<Widget />
			<StatusBar
				animated
				backgroundColor="transparent"
				translucent
				style="light"
			/>
		</View>
	)
}
