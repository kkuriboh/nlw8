"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.NodemailerMailAdapter = void 0;
const nodemailer_1 = __importDefault(require("nodemailer"));
class NodemailerMailAdapter {
    constructor() {
        this.sendMail = async ({ body, subject }) => {
            const transport = nodemailer_1.default.createTransport({
                host: 'smtp.mailtrap.io',
                port: 2525,
                auth: {
                    user: process.env.MAILTRAP_USER,
                    pass: process.env.MAILTRAP_PASS,
                },
            });
            await transport.sendMail({
                from: 'Equipe Feedget <oi@feedget.com>',
                to: 'Josué <josue@batatinha.com>',
                subject,
                html: body,
            });
        };
    }
}
exports.NodemailerMailAdapter = NodemailerMailAdapter;
