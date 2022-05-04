"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SubmitFeedbackUseCase = void 0;
class SubmitFeedbackUseCase {
    constructor(feedbacsRepository, mailAdapter) {
        this.feedbacsRepository = feedbacsRepository;
        this.mailAdapter = mailAdapter;
        this.execute = async (req) => {
            const { type, comment, screenshot } = req;
            if (!type)
                throw new Error('Type is required.');
            if (!comment)
                throw new Error('Comment is required.');
            if (screenshot && !screenshot.startsWith('data:image/png;base64'))
                throw new Error('Invalid screenshot format.');
            await this.feedbacsRepository.create({
                type,
                comment,
                screenshot,
            });
            await this.mailAdapter.sendMail({
                subject: 'Novo feedback',
                body: [
                    `<div style="font-family: sans-serif; font-size: 16px; color: #111;">`,
                    `<p>Tipo do feedback ${type}</p>`,
                    `<p>Comentário: ${comment}</p>`,
                    `</div>`,
                ].join('\n'),
            });
        };
    }
}
exports.SubmitFeedbackUseCase = SubmitFeedbackUseCase;
