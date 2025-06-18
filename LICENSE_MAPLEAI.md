# MapleAI Integration Licensing

MapleAI relies on several external protocols and frameworks to provide AI capabilities. The table below lists each reference and links to the source license or terms of use.

| Protocol | Purpose in MapleAI | License / Terms |
|---------|-------------------|----------------|
| OpenAI API | Text generation via the `ai-orchestra` service. | [OpenAI API Terms of Use](https://openai.com/policies/terms-of-use) |
| Anthropic Claude API | Optional provider for the LLM orchestra. | [Anthropic Terms of Use](https://www.anthropic.com/legal) |
| Google Gemini API | Optional provider for the LLM orchestra. | [Google Generative AI Service Terms](https://ai.google.dev/terms) |
| Mistral AI API | Optional provider for the LLM orchestra. | [Mistral Terms](https://mistral.ai/terms) |
| Ollama API | Local LLM provider interface. | [Ollama License](https://github.com/ollama/ollama/blob/main/LICENSE) |
| ONNX Runtime | Local model execution through the `ort` crate. | [MIT License](https://github.com/microsoft/onnxruntime/blob/main/LICENSE) |
| gRPC / Protocol Buffers | Internal service communication. | [BSD-3-Clause](https://github.com/protocolbuffers/protobuf/blob/main/LICENSE) |

When extending MapleAI, ensure that any new integrations comply with these upstream licenses and terms. Contributors must include appropriate attribution where required and respect the usage limitations of each provider.
