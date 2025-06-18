# MapleAI Agent Framework

MapleAI agents bring autonomous characters to Finalverse. Each agent is composed of
three primary pieces:

1. **Agent State** – stores the agent id, current region and reasoning context.
2. **Planner** – decides the next [`BehaviorAction`](../crates/protocol/src/action.rs) using
   simple heuristics based on harmony and tension.
3. **LLM Bridge** – sends the agent state to **AI Orchestra** and records the
   returned reasoning in the agent's memory.

Agents run in their own async loops and can be managed through the `behavior-ai`
service. That service exposes `/agent/spawn` and `/agent/{id}/act` endpoints for
spawning agents and requesting a single reasoning step.

## Supported LLM Providers

`AI Orchestra` discovers LLM providers from environment variables and selects a
`FINALVERSE_DEFAULT_LLM` at runtime. The following providers are built in:

- **Ollama** – default local provider using `http://localhost:11434`.
- **Local** – ONNX model from `LOCAL_LLM_PATH`.
- **OpenAI** – enabled with `OPENAI_API_KEY` and optional `OPENAI_BASE_URL` and
  `OPENAI_MODEL`.
- **Claude** – enabled with `ANTHROPIC_API_KEY`, `ANTHROPIC_BASE_URL` and
  `CLAUDE_MODEL`.
- **Gemini** – enabled with `GEMINI_API_KEY`, `GEMINI_BASE_URL` and
  `GEMINI_MODEL`.
- **Mistral** – enabled with `MISTRAL_API_KEY`, `MISTRAL_BASE_URL` and
  `MISTRAL_MODEL`.

Set the appropriate variables before launching `ai-orchestra` to register the
provider. The chosen default is used whenever a model is not specified.

## Relationship to AI Orchestra

MapleAI relies on the **AI Orchestra** service to abstract over different LLMs.
The `LLM Bridge` inside each agent makes a generation request through AI
Orchestra which then dispatches it to the configured provider. This keeps agent
code simple while allowing new models to be added without recompiling the
`behavior-ai` service.
