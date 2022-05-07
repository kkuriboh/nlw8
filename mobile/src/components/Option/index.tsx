import {
	View,
	TouchableOpacity,
	TouchableOpacityProps,
	Image,
	Text,
	ImageSourcePropType,
} from 'react-native'

import { styles } from './styles'

type props = TouchableOpacityProps & {
	title: string
	image: ImageSourcePropType
}

export const Option: React.FC<props> = ({ title, image, ...rest }) => {
	return (
		<TouchableOpacity style={styles.container} {...rest}>
			<Image source={image!} style={styles.image} />
			<Text style={styles.title}>Title</Text>
		</TouchableOpacity>
	)
}
