/**
 * OpenAI API client for code generation
 */

import OpenAI from "openai";
import type { ChatCompletionMessageParam } from "openai/resources/chat/completions";
import { extractCodeFromResponse, validateExtractedCode } from "./parser.js";
import { EDIT_PROMPT, SYSTEM_PROMPT } from "./prompts.js";

export interface AIClientConfig {
  apiKey: string;
  baseURL?: string;
  model?: string;
}

export interface GenerationResult {
  code: string;
  conversationHistory: ChatCompletionMessageParam[];
}

export class AIClient {
  private client: OpenAI;
  private model: string;

  constructor(config: AIClientConfig) {
    this.client = new OpenAI({
      apiKey: config.apiKey,
      baseURL: config.baseURL,
    });
    this.model = config.model || "gpt-4";
  }

  /**
   * Generate new automation code from user description
   */
  async generateCode(
    userPrompt: string,
    conversationHistory: ChatCompletionMessageParam[] = [],
  ): Promise<GenerationResult> {
    const messages: ChatCompletionMessageParam[] = [
      { role: "system", content: SYSTEM_PROMPT },
      ...conversationHistory,
      { role: "user", content: userPrompt },
    ];

    const response = await this.client.chat.completions.create({
      model: this.model,
      messages,
      temperature: 0.7,
      max_tokens: 2000,
    });

    const content = response.choices[0]?.message?.content || "";
    const code = extractCodeFromResponse(content);

    const validation = validateExtractedCode(code);
    if (!validation.valid) {
      throw new Error(`Invalid code generated: ${validation.error}`);
    }

    const newHistory: ChatCompletionMessageParam[] = [
      ...conversationHistory,
      { role: "user", content: userPrompt },
      { role: "assistant", content },
    ];

    return {
      code,
      conversationHistory: newHistory,
    };
  }

  /**
   * Edit existing code based on user feedback
   */
  async editCode(
    existingCode: string,
    userFeedback: string,
    conversationHistory: ChatCompletionMessageParam[] = [],
  ): Promise<GenerationResult> {
    const messages: ChatCompletionMessageParam[] = [
      { role: "system", content: EDIT_PROMPT },
      ...conversationHistory,
      {
        role: "user",
        content: `Current script:\n\`\`\`typescript\n${existingCode}\n\`\`\`\n\nUser request: ${userFeedback}`,
      },
    ];

    const response = await this.client.chat.completions.create({
      model: this.model,
      messages,
      temperature: 0.7,
      max_tokens: 2000,
    });

    const content = response.choices[0]?.message?.content || "";
    const code = extractCodeFromResponse(content);

    const validation = validateExtractedCode(code);
    if (!validation.valid) {
      throw new Error(`Invalid code generated: ${validation.error}`);
    }

    const newHistory: ChatCompletionMessageParam[] = [
      ...conversationHistory,
      {
        role: "user",
        content: `Current script:\n\`\`\`typescript\n${existingCode}\n\`\`\`\n\nUser request: ${userFeedback}`,
      },
      { role: "assistant", content },
    ];

    return {
      code,
      conversationHistory: newHistory,
    };
  }
}

/**
 * Create AI client from environment variables
 */
export function createAIClientFromEnv(): AIClient {
  const apiKey = process.env.OPENAI_API_KEY;
  if (!apiKey) {
    throw new Error(
      "OPENAI_API_KEY environment variable is required. Please set it before running this command.",
    );
  }

  return new AIClient({
    apiKey,
    baseURL: process.env.OPENAI_BASE_URL,
    model: process.env.OPENAI_MODEL,
  });
}
