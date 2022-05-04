import { useState } from 'react'

import { FeedbackContentStep } from './steps/FeedbackContentStep'
import { FeedbackSuccessStep } from './steps/FeedbackSuccessStep'
import { FeedbackType, FeedbackTypeStep } from './steps/FeedbackTypeStep'

export const WidgetForm: React.FC = () => {
	const [feedbackType, setFeedbackType] = useState<FeedbackType | null>(null)
	const [feedbackSent, setFeedbackSent] = useState(false)

	const handleRestartFeedback = () => {
		setFeedbackSent(false)
		setFeedbackType(null)
	}

	return (
		<div className="bg-zinc-900 p-4 relative rounded-2xl mb-4 flex flex-col items-center shadow-lg w-[calc(100vw-2rem)] md:w-auto">
			{feedbackSent ? (
				<FeedbackSuccessStep
					onFeedbackRestartRequested={handleRestartFeedback}
				/>
			) : (
				<>
					{!feedbackType ? (
						<FeedbackTypeStep
							onFeedbackTypeChanged={setFeedbackType}
						/>
					) : (
						<FeedbackContentStep
							feedbackType={feedbackType}
							onFeedbackRestartRequested={handleRestartFeedback}
							onFeedbackSent={() => setFeedbackSent(true)}
						/>
					)}
				</>
			)}
			<footer className="text-xs text-neutral-400">
				Feito com ♥ pela{' '}
				<a
					className="underline underline-offset-2"
					href="https://rocketseat.com.br"
				>
					Rocketseat
				</a>
			</footer>
		</div>
	)
}
