import { MailAdapter } from '../adapters/mail-adapter'
import { FeedbacksRepository } from '../repositories/feedbacs-repository'

type SubmitFeedbackUseCaseRequest = {
	type: string
	comment: string
	screenshot?: string
}

export class SubmitFeedbackUseCase {
	constructor(
		private feedbacsRepository: FeedbacksRepository,
		private mailAdapter: MailAdapter
	) {}

	execute = async (req: SubmitFeedbackUseCaseRequest) => {
		const { type, comment, screenshot } = req

		if (!type) throw new Error('Type is required.')
		if (!comment) throw new Error('Comment is required.')
		if (screenshot && !screenshot.startsWith('data:image/png;base64'))
			throw new Error('Invalid screenshot format.')

		await this.feedbacsRepository.create({
			type,
			comment,
			screenshot,
		})

		await this.mailAdapter.sendMail({
			subject: 'Novo feedback',
			body: [
				`<div style="font-family: sans-serif; font-size: 16px; color: #111;">`,
				`<p>Tipo do feedback ${type}</p>`,
				`<p>Comentário: ${comment}</p>`,
				`</div>`,
			].join('\n'),
		})
	}
}
