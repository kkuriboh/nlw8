export type FeedbackCreateData = {
	type: string
	comment: string
	screenshot?: string
}

export type FeedbacksRepository = {
	create: (data: FeedbackCreateData) => Promise<void>
}
