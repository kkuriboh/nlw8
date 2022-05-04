import bugImageUrl from '../../../assets/bug.svg'
import ideaImageUrl from '../../../assets/idea.svg'
import thoughtImageUrl from '../../../assets/thought.svg'
import { CloseButton } from '../../CloseButton'

export const feedbackTypes = {
	BUG: {
		title: 'Problema',
		image: {
			source: bugImageUrl,
			alt: 'Imagem de um inseto',
		},
	},
	IDEA: {
		title: 'Ideia',
		image: {
			source: ideaImageUrl,
			alt: 'Imagem de uma lâmpada',
		},
	},
	OTHER: {
		title: 'Outro',
		image: {
			source: thoughtImageUrl,
			alt: 'Imagem de um balão de pensamento',
		},
	},
}
export type FeedbackType = keyof typeof feedbackTypes

type Props = {
	onFeedbackTypeChanged: (feedbackType: FeedbackType) => void
}

export const FeedbackTypeStep: React.FC<Props> = ({
	onFeedbackTypeChanged,
}) => {
	return (
		<>
			<header>
				<span className="text-xl leading-6">Deixe seu feedback</span>
				<CloseButton />
			</header>
			<div className="flex py-8 gap-2 w-full">
				{Object.entries(feedbackTypes).map(([key, value]) => (
					<button
						key={key}
						className="bg-zinc-800 rounded-lg py-5 w-24 flex-1 flex flex-col items-center gap-2 border-2 border-transparent hover:border-brand-500 focus:border-brand-500 focus:outline-none"
						type="button"
						onClick={() =>
							onFeedbackTypeChanged(key as FeedbackType)
						}
					>
						<img src={value.image.source} alt={value.image.alt} />
						<span>{value.title}</span>
					</button>
				))}
			</div>
		</>
	)
}
