# codexco 💰

Monitor y calculador de costos en tiempo real para sesiones de **Codex** (OpenAI).

## ¿Cómo funciona?

Codex persiste el estado y los eventos de cada turno en `~/.codex/sessions/{año}/{mes}/{día}/rollout-*.jsonl`.
`codexco` inspecciona de forma no invasiva dichos rollouts, extrae el conteo granular de tokens (`TokenUsage` con desglose de input no cacheado, cache hit, output y razonamiento) y aplica la tabla de tarifas oficial para calcular el costo real de tus sesiones.

## Comandos

- `codexco current` (o simplemente `codexco`): Muestra el desglose de tokens y costo de la sesión activa o más reciente.
- `codexco history`: Lista las últimas sesiones con el consumo de tokens y el costo acumulado por sesión.
- `codexco summary`: Resumen agregado de tokens consumidos y gasto total acumulado en USD.
- `codexco watch`: Modo monitor que actualiza la terminal en tiempo real a medida que interactuás con Codex.

## Tarifas soportadas

Incluye tarifas preconfiguradas para GPT-4o, GPT-4o-mini, o1, o3-mini y modelos compatibles, diferenciando el descuento por cached inputs.
