import { View, Text } from 'react-native'

import { styles } from './styles'

export const Copyright: React.FC = () => {
	return (
		<View style={styles.container}>
			<Text style={styles.text}>Feito com ♥ pela Rocketseat</Text>
		</View>
	)
}
