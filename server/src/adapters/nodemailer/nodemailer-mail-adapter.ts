import nodemailer from 'nodemailer'
import { MailAdapter, SendMailData } from '../mail-adapter'

export class NodemailerMailAdapter implements MailAdapter {
	sendMail = async ({ body, subject }: SendMailData) => {
		const transport = nodemailer.createTransport({
			host: 'smtp.mailtrap.io',
			port: 2525,
			auth: {
				user: process.env.MAILTRAP_USER,
				pass: process.env.MAILTRAP_PASS,
			},
		})

		await transport.sendMail({
			from: 'Equipe Feedget <oi@feedget.com>',
			to: 'Josué <josue@batatinha.com>',
			subject,
			html: body,
		})
	}
}
