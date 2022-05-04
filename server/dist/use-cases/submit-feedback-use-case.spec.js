"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const submit_feedback_use_case_1 = require("./submit-feedback-use-case");
const createFeedbackSpy = jest.fn();
const sendEmailSpy = jest.fn();
const submitFeedback = new submit_feedback_use_case_1.SubmitFeedbackUseCase({
    create: createFeedbackSpy,
}, {
    sendMail: sendEmailSpy,
});
describe('Submit feedback', () => {
    it('should be able to submit a feedback', async () => {
        await expect(submitFeedback.execute({
            type: 'BUG',
            comment: 'example',
            screenshot: 'data:image/png;base64aaa.png',
        })).resolves.not.toThrow();
        expect(createFeedbackSpy).toHaveBeenCalled();
        expect(sendEmailSpy).toHaveBeenCalled();
    });
    it('should be able to submit a feedback without type', async () => {
        await expect(submitFeedback.execute({
            type: '',
            comment: 'example',
            screenshot: 'data:image/png;base64aaa.png',
        })).rejects.toThrow();
    });
    it('should be able to submit a feedback without comment', async () => {
        await expect(submitFeedback.execute({
            type: 'IDEA',
            comment: '',
            screenshot: 'data:image/png;base64aaa.png',
        })).rejects.toThrow();
    });
    it('should be able to submit a feedback with invalid screenshot', async () => {
        await expect(submitFeedback.execute({
            type: 'IDEA',
            comment: 'example',
            screenshot: 'aaa.png',
        })).rejects.toThrow();
    });
});
