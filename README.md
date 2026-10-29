# codexco 🪙

> High-performance, non-invasive session cost analyzer and real-time monitor for the **OpenAI Codex CLI**.

`codexco` reads local Codex session rollouts, streams real-time token events per turn, breaks down usage across multiple models (non-cached, prompt cache hits, output, and reasoning), and applies official pricing tiers to calculate accurate session expenditure.

---

## Features

- **Accurate Token-Accounting**: Ingests exact usage telemetry from OpenAI returned at the end of each turn.
- **Prompt Cache Awareness**: Differentiates between non-cached input tokens and cached inputs for true pricing fidelity.
- **Multi-Model Session Tracking**: Supports dynamic model switching within a single session (e.g., transitioning between `gpt-6.1-terra` and `gpt-6-luna`), calculating costs per model.
- **Zero-Invasive Architecture**: No patched binaries or external dependencies needed. Reads native Codex session streams directly from `~/.codex/sessions`.
- **Live Terminal Monitor**: Watch mode to inspect ongoing sessions and track token consumption turn-by-turn.

---

## Installation

### Option 1: Quick Install (No Rust required)

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/ale-01cu/codexco/main/install.ps1 | iex
```

**Linux / macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/ale-01cu/codexco/main/install.sh | bash
```

---

### Option 2: Via Cargo (For Rust developers)

```bash
cargo install --git https://github.com/ale-01cu/codexco.git
```

---

### Option 3: Build from source

```bash
git clone https://github.com/ale-01cu/codexco.git
cd codexco
cargo install --path .
```

---

## Usage

### 1. Inspect Active / Latest Session

Displays a detailed token breakdown and total cost for the most recent session:

```bash
codexco current
# or simply
codexco
```

**Example Output:**

```text
=================================================================
Sesión: 01a1836f-f82c-7191-b6f3-f674aebdfb61
Fecha:   2026-10-28T16:55:15.768Z
Workspace: C:\Projects\MyProject
Modelos usados: gpt-6.1-terra, gpt-6-luna
-----------------------------------------------------------------
DESGLOSE POR MODELO:
  • gpt-6.1-terra      230588 tok (in: 1102, cached: 228941, out: 545) -> $0.01428 USD
  • gpt-6-luna       57536068 tok (in: 774493, cached: 56646168, out: 115407) -> $0.70195 USD
-----------------------------------------------------------------
Tokens Entrada (Total):          57650704
  - No cacheados:                  775595
  - Cacheados (ahorro):          56875109
Tokens Salida:                     115952
  - Razonamiento:                   47943
Total Tokens Acumulados:         57766656
-----------------------------------------------------------------
COSTO TOTAL ESTIMADO:         $0.71623 USD
=================================================================
```

### 2. Session History

List recent sessions with their respective token volume and spend:

```bash
codexco history --limit 15
```

### 3. Aggregate Spend Summary

Display all-time metrics across all recorded sessions:

```bash
codexco summary
```

### 4. Real-time Session Watcher

Continuously updates the terminal display as you collaborate with Codex:

```bash
codexco watch --interval 2
```

---

## Supported Pricing Models

`codexco` includes official token pricing for:
- **GPT-6 Series**: `gpt-6-astra`, `gpt-6.1-sol`, `gpt-6.1-terra`, `gpt-6-luna`, `gpt-6-sol`
- **GPT-5 Series**: `gpt-5.6-sol`, `gpt-5.6-terra`, `gpt-5.6-luna`, `gpt-5.5`, `gpt-5.5-pro`, `gpt-5.4-mini`, `gpt-5.4-nano`, `gpt-5.2`, `gpt-5.1`, `gpt-5`, etc.
- **GPT-4 Series**: `gpt-4.1`, `gpt-4.1-mini`, `gpt-4o`, `gpt-4o-mini`
- **Reasoning Models**: `o1`, `o3-mini`

---

## License

Apache-2.0 or MIT
