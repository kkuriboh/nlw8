import { prisma } from '../../prisma'
import { FeedbacksRepository, FeedbackCreateData } from '../feedbacs-repository'

export class PrismaFeedbacsRepository implements FeedbacksRepository {
	create = async ({ comment, type, screenshot }: FeedbackCreateData) => {
		await prisma.feedback.create({
			data: {
				type,
				comment,
				screenshot,
			},
		})
	}
}
