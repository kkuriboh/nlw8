"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PrismaFeedbacsRepository = void 0;
const prisma_1 = require("../../prisma");
class PrismaFeedbacsRepository {
    constructor() {
        this.create = async ({ comment, type, screenshot }) => {
            await prisma_1.prisma.feedback.create({
                data: {
                    type,
                    comment,
                    screenshot,
                },
            });
        };
    }
}
exports.PrismaFeedbacsRepository = PrismaFeedbacsRepository;
