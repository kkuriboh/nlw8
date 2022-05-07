import { Camera, Trash } from 'phosphor-react-native'
import React from 'react'

import { View, TouchableOpacity, Image } from 'react-native'
import { theme } from '../../theme'

import { styles } from './styles' //💅

type props = {
	screenshot: string | null
	onTakeShot: () => void
	onRemoveShot: () => void
}

export const ScreenshotButton: React.FC<props> = ({
	screenshot,
	onRemoveShot,
	onTakeShot,
}) => {
	return (
		<TouchableOpacity
			style={styles.container}
			onPress={screenshot ? onRemoveShot : onTakeShot}
		>
			{screenshot ? (
				<View>
					<Image style={styles.image} source={{ uri: screenshot }} />
					<Trash
						size={22}
						color={theme.colors.text_secondary}
						weight="fill"
						style={styles.removeIcon}
					/>
				</View>
			) : (
				<Camera
					size={24}
					color={theme.colors.text_secondary}
					weight="bold"
				/>
			)}
		</TouchableOpacity>
	)
}
