import React from 'react'

import {
	Text,
	TouchableOpacity,
	TouchableOpacityProps,
	ActivityIndicator,
} from 'react-native'
import { theme } from '../../theme'

import { styles } from './styles' //💅

type props = TouchableOpacityProps & {
	isLoading: boolean
}

export const Button: React.FC<props> = ({ isLoading, ...rest }) => {
	return (
		<TouchableOpacity {...rest} style={styles.container}>
			{isLoading ? (
				<ActivityIndicator color={theme.colors.text_on_brand_color} />
			) : (
				<Text style={styles.title}>Enviar Feedback</Text>
			)}
		</TouchableOpacity>
	)
}
