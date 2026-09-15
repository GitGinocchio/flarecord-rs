# 🎯 Documentazione: Modal Submissions in Flarecord

## Panoramica

Le **Modal** sono dialoghi a più campi che Discord può mostrare agli utenti tramite webhook. In Flarecord, puoi gestire le modal submissions ricevute da Discord e elaborare i dati inviati dagli utenti.

---

## Quando usare le Modali

✅ **Casi d'uso ideali:**
- Raccolta feedback strutturato (rating, commenti, ecc.)
- Form di configurazione multi-passo
- Data entry complessa con validazione client-side
- Survey o questionari interattivi

❌ **Non usare per:**
- Comandi slash (usa i `Command` trait)  
- Componenti semplici come bottoni (usa `Component` trait)
- Dati che richiedono presenza/voice (necessita Gateway events)

---

## Architettura

```
┌─────────────────────────────────────────────────┐
│           Discord Bot User Interface            │
│                                                 │
│   ┌───────────────┐    ┌───────────────────┐   │
│   │  Button Click │───▶│  Modal Display   │   │
│   │  (via Webhook)│    │  (native UI)     │   │
│   └───────────────┘    └───────────────────┘   │
│         │                         │             │
│         ▼                         ▼             │
│   ┌───────────────┐    ┌───────────────────┐   │
│   │ Discord API   │◀──▶│ Modal Submission  │   │
│   │ POST /channels │   │ JSON Payload      │   │
│   └───────────────┘    └───────────────────┘   │
│         │                         │             │
│         ▼                         ▼             │
│  ┌───────────────┐    ┌───────────────────┐   │
│  │ Signature      │    │ ModalHandler      │   │
│  │ Verification   │    │ on_submit()       │   │
│  └───────────────┘    └───────────────────┘   │
└─────────────────────────────────────────────────┘
```

---

## API Reference

### `Modal` Trait

Trait principale per gestire le modal submissions:

```rust
pub trait Modal: Send + Sync {
    fn id(&self) -> String;              // Custom ID del modal
    fn title(&self) -> String;           // Titolo (opzionale)
    fn components(&self) -> Vec<TextInput>;  // TextInput campi
    fn on_submit(
        &self, 
        interaction: ModalInteraction, 
        ctx: ModalContext
    ) -> impl Future<Output = BotResult<()>> + Send;
}
```

#### Metodi:

| Metodo | Descrizione | Tipo Restituito |
|--------|-------------|-----------------|
| `id()` | Ottiene il custom_id (required per lookup) | `String` |
| `title()` | Ottiene il titolo del modal | `String` |
| `components()` | Lista di tutti i TextInput | `Vec<TextInput>` |
| `on_submit()` | Handler chiamato al submit | `Future<BotResult<()>>` |

### `ModalInteraction`

Struttura dati per le submissions:

```rust
pub struct ModalInteraction {
    pub data: ModalData,              // I dati del modal
    pub token: String,                // Token di sessione
    pub channel_id: Id<ChannelMarker>,// Canale del submit
    pub guild_id: Option<Id<GuildMarker>>,
    pub member: Option<Member>,       // Member info
    pub user: Option<User>,           // User info
    // ... altri campi Twilight
}
```

### `ModalData`

Contiene i dati submission del modal:

```rust
pub struct ModalData {
    pub custom_id: String,            // Identificativo univoco
    pub values: HashMap<String, String>, // Mappa campo -> valore
    pub version: u64,                 // Versioning Discord
}
```

#### Metodi helper:

| Metodo | Descrizione | Esempio |
|--------|-------------|---------|
| `custom_id()` | Restituisce il custom_id | `"user_feedback"` |
| `values()` | Raw JSON values | `&serde_json::Value` |
| `get_values_as_map()` | Mappa parsed | `HashMap<String, String>` |
| `version()` | Version Discord | `u64` (1-9) |

### `TextInput`

Campo di testo in una modal:

```rust
pub struct TextInput {
    pub custom_id: String,            // Required per submit
    pub style: Option<TextInputStyle>, // PlainText/Paragraph
    pub label: Option<String>,        // Label UI
    pub placeholder: Option<String>,  // Placeholder text
    pub value: Option<String>,        // Pre-filled value
    pub max_length: u32,              // 1-4000 chars
    pub min_length: Option<u32>,      // Optional 0-4000
    pub short_length: bool,           // <=60 chars (UI effect)
    pub required: bool,               // Campo obbligatorio
    pub autocomplete: bool,           // Abilita autocomplete
}
```

#### Metodi helper:

| Metodo | Descrizione | Restituisce |
|--------|-------------|-------------|
| `get_custom_id()` | Ottiene il custom_id | `&str` |
| `is_validated()` | Verifica validità campo | `bool` |

### `TextInputBuilder`

Builder pattern per creare `TextInput`:

```rust
pub struct TextInputBuilder {
    custom_id: String,
    label: Option<String>,
    placeholder: Option<String>,
    max_length: u32,
    required: bool,
    short_length: bool,
    style: TextInputStyle,
}
```

#### Metodi chain:

| Metodo | Descrizione | Parametro |
|--------|-------------|-----------|
| `new()` | Crea builder con default | `impl Into<String>` |
| `label()` | Imposta label UI | `impl Into<String>` |
| `placeholder()` | Imposta placeholder | `impl Into<String>` |
| `max_length()` | Max characters | `u32` (1-4000) |
| `min_length()` | Min characters | `u32` (0-4000) |
| `short_length()` | UI short effect | `bool` |
| `required()` | Campo obbligatorio | `bool` |
| `style()` | Text style | `TextInputStyle` |
| `build()` | Costruisci TextInput | `TextInput` |

#### Esempio completo:

```rust
let text_input = TextInputBuilder::new("username")
    .label("Nome Utente")
    .placeholder("Il tuo nome...")
    .max_length(50)
    .short_length(true)  // Input corto per UI ottimizzata
    .required(true)
    .build();
```

### `ModalBuilder`

Helper ergonomico per costruire modali:

```rust
pub struct ModalBuilder {
    custom_id: String,
    title: Option<String>,
    components: Vec<TextInput>,
}
```

#### Metodi chain:

| Metodo | Descrizione | Restituisce |
|--------|-------------|-------------|
| `new()` | Crea builder vuoto | `ModalBuilder` |
| `title()` | Imposta titolo | `&mut Self` |
| `add_text_input()` | Aggiungi TextInput | `&mut Self` |
| `text_input()` | Chain helper | `TextInputBuilder` |
| `build()` | Costruisci Modal | `Result<Modal, String>` |

#### Esempio:

```rust
let modal = ModalBuilder::new("user_feedback")
    .title("Feedback Utente")
    .add_text_input(TextInputBuilder::new("name")
        .label("Nome")
        .max_length(50)
        .required(true)
        .build())
    .add_text_input(TextInputBuilder::new("email")
        .label("Email")
        .placeholder("email@esempio.com")
        .max_length(254)
        .required(true)
        .build())
    .build()?;  // ? - almeno un campo richiesto
```

---

## Implementazione Completa

### Esempio Base: Feedback Form

```rust
use flarecord::{prelude::*, models::modals::*};

pub struct FeedbackModal;

impl Modal for FeedbackModal {
    fn id(&self) -> String {
        "feedback_modal".into()
    }

    fn title(&self) -> String {
        "Feedback Utente".into()
    }

    fn components(&self) -> Vec<TextInput> {
        vec![
            TextInputBuilder::new("user_name")
                .label("Nome Completo")
                .placeholder("Es: Mario Rossi")
                .max_length(100)
                .required(true)
                .build(),

            TextInputBuilder::new("rating")
                .label("Rating 1-5")
                .placeholder("Numero da 1 a 5")
                .max_length(1)
                .short_length(true)
                .required(true)
                .build(),

            TextInputBuilder::new("comment")
                .label("Commento (opzionale)")
                .placeholder("Condividi i tuoi pensieri...")
                .max_length(1000)
                .required(false)
                .build(),
        ]
    }

    async fn on_submit(
        &self, 
        interaction: ModalInteraction, 
        ctx: ModalContext
    ) -> BotResult<()> {
        let data = &interaction.data;
        
        // Logging dei dati ricevuti
        worker::console_info!("=== Modal Feedback Submitted ===");
        worker::console_info!("Custom ID: {}", data.custom_id);
        worker::console_info!("Version: {}", data.version());
        
        // Mappa dei valori
        let values_map = data.get_values_as_map();
        for (key, value) in &values_map {
            worker::console_info!(key: "{key}", "Value: {value}");
        }

        Ok(())
    }
}

// Registrazione nel bot
pub fn build_bot() -> Arc<Bot> {
    BotBuilder::new()
        .register_modal(FeedbackModal)  // O .with_modal(|| FeedbackModal)
        .build()
}
```

---

## Validazione dei Dati

### Controllo Campi Obbligatori

Discord valida client-side i campi `required`, ma puoi aggiungere validazione server-side:

```rust
async fn on_submit(
    &self, 
    interaction: ModalInteraction, 
    ctx: ModalContext
) -> BotResult<()> {
    let data = &interaction.data;
    
    // Verifica tutti i campi required
    if let Some(values) = data.values.as_object() {
        for (key, _) in values.iter() {
            // Se key contiene "required_" e value è vuoto, errore
            if key.starts_with("required_") && values[key].is_empty() {
                return Err(Error::Generic(format!("Campo {} vuoto", key)));
            }
        }
    }

    Ok(())
}
```

### Validazione Rating (1-5)

```rust
async fn on_submit(
    &self, 
    interaction: ModalInteraction, 
    ctx: ModalContext
) -> BotResult<()> {
    let data = &interaction.data;
    
    if let Some(rating_str) = data.values.as_object().get("rating") {
        match rating_str.parse::<u32>() {
            Ok(rating) => {
                if rating < 1 || rating > 5 {
                    return Err(Error::Generic(
                        "Rating deve essere tra 1 e 5".into()
                    ));
                }
            },
            Err(_) => {
                return Err(Error::Generic(
                    "Rating non è un numero valido".into()
                ));
            }
        }
    }

    Ok(())
}
```

---

## Styling dei TextInput

### PlainText vs Paragraph

Discord supporta due stili per i `TextInput`:

| Stile | Uso Consigliato | Max Length |
|-------|-----------------|------------|
| **PlainText** | Input brevi, dati strutturati | 1-60 chars (UI optim.) |
| **Paragraph** | Testi lunghi, descrizioni | 1-4000 chars |

```rust
// PlainText per campi corti
let short_input = TextInputBuilder::new("rating")
    .label("Rating")
    .max_length(1)
    .short_length(true)
    .style(TextInputStyle::PlainText)
    .build();

// Paragraph per commenti lunghi
let long_text = TextInputBuilder::new("story")
    .label("La tua storia")
    .placeholder("Scrivi liberamente...")
    .max_length(4000)
    .short_length(false)  // Default è false, ok così!
    .style(TextInputStyle::Paragraph)
    .build();
```

### Autocomplete (in sviluppo futuro)

L'autocompletamento è disponibile ma disabilitato di default. Per attivarlo:

```rust
TextInputBuilder::new("name")
    .label("Nome")
    .autocomplete(true)  // Future feature
    .build();
```

---

## Error Handling

### ModalNotFound Error

Se Discord invia un modal submission per un `custom_id` non registrato:

```rust
// Questo errore viene thrown quando:
// - Custom ID del modal non è nel registro bot.modals
// - Errore: "Modal 'feedback_modal' is not registered"

Error::ModalNotFound("feedback_modal".into())
```

### Validazione Fallita

```rust
async fn on_submit(...) -> BotResult<()> {
    // Validazione fallita
    if !data.values.as_object().iter()
        .any(|(k, v)| k.starts_with("required_"))
    {
        return Err(Error::Generic("Nessun campo compilato".into()));
    }

    Ok(())
}
```

---

## Best Practices

### 1. Custom ID Unico

Usa custom_id significativi e univoci:

```rust
❌ Buono (generico):
"modal_1"

✅ Ottimo (descrittivo):
"user_feedback_form_v2"
"server_configuration_modal"
"payment_confirmation"
```

### 2. Label Chiari e Descrittivi

```rust
// ❌ Ambiguo
TextInputBuilder::new("field_1")
    .label("") // No label!

// ✅ Chiaro
TextInputBuilder::new("full_name")
    .label("Nome Completo")
    .placeholder("Mario Rossi")
    .required(true)
    .build();
```

### 3. Max Length Appropriate

```rust
// ❌ Troppo rigido per testo libero
TextInputBuilder::new("bio")
    .max_length(10) // Troppo corto!
    .build();

// ✅ Flessibile
TextInputBuilder::new("bio")
    .max_length(500) // Ragionevole per bio
    .build();
```

### 4. Short Length Solo Quando Necessario

`short_length(true)` ottimizza UI per input brevi (≤60 chars):

```rust
// ✅ Per rating 1-5
TextInputBuilder::new("rating")
    .max_length(1)
    .short_length(true)
    .build();

// ❌ Non necessario per testi lunghi
TextInputBuilder::new("comment")
    .max_length(1000)
    .short_length(false) // Default è false, ok così!
    .build();
```

---

## Integrations

### Modal + Button Component

Le modali possono essere aperte da bottoni interattivi:

```rust
// Botone che apre una modal
pub struct OpenFeedbackButton;

impl Component for OpenFeedbackButton {
    fn build(&self, root: &mut RootComponent) {
        let button = Button::new()
            .style(ButtonStyle::Primary)
            .label("Invia Feedback")
            .build();

        root.add(button);
    }

    async fn handle(
        &self, 
        interaction: ComponentInteraction, 
        ctx: ComponentContext
    ) -> BotResult<CommandResponse> {
        // Invia modal tramite webhook
        let channel_id = interaction.channel_id;
        
        // Discord webhook invierà la modal
        // L'utente compila i campi e clicca "Submit"
        // La submission arriva a on_submit() del ModalHandler
        
        Ok(CommandResponse::new())
    }
}
```

### Multi-Step Form Pattern

Puoi implementare modali multi-passo:

```rust
pub struct RegistrationModal;

impl Modal for RegistrationModal {
    fn components(&self) -> Vec<TextInput> {
        // Passo 1: Dati utente
        vec![
            TextInputBuilder::new("username").label("Username").required(true).build(),
            TextInputBuilder::new("email").label("Email").required(true).build(),
            TextInputBuilder::new("age").label("Età").max_length(3).short_length(true).build(),
        ]
    }

    async fn on_submit(&self, interaction, ctx) -> BotResult<()> {
        // Salva i dati step 1
        // In un secondo modal: verifica email
        // Terzo modal: password
        
        Ok(())
    }
}
```

---

## API Discord Reference

### Endpoint REST per Modali

```
POST https://discord.com/api/v10/channels/{channel_id}/messages
Headers:
  Authorization: Bearer {token}
  Content-Type: application/json

Body (modal submission):
{
  "type": 5,
  "data": {
    "custom_id": "feedback_modal",
    "values": {
      "username": "Mario Rossi",
      "email": "mario@example.com",
      "rating": "5"
    },
    "version": 1
  }
}
```

### Risposta Expected

Discord invia la modal come `/messages` payload. Il server risponde con:

```json
{
  "type": 1,  // ACK: Acknowledged
  "data": { ... } // Echo dei dati (opzionale)
}
```

---

## Testing Modal Handlers

### Unit Test Esempio

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_modal_builder() {
        let modal = ModalBuilder::new("test_modal")
            .add_text_input(TextInputBuilder::new("name")
                .label("Name")
                .max_length(50)
                .required(true))
            .build();
        
        assert_eq!(modal.custom_id, "test_modal");
        assert_eq!(modal.components.len(), 1);
    }

    #[test]
    fn test_required_field_validation() {
        let modal = ModalBuilder::new("invalid")
            // Nessun campo!
            .build();
        
        assert!(modal.is_err());  // Errore: "Modal deve avere almeno un TextInput"
    }
}
```

### Integration Test Esempio

```rust
#[tokio::test]
async fn test_modal_submission() {
    let interaction = ModalInteraction {
        data: ModalData {
            custom_id: "test_modal".into(),
            values: json!({
                "username": "TestUser",
                "rating": "5"
            }),
            version: 1
        },
        // ... altri campi mockati
    };

    let ctx = ModalContext::new(/* mock bot state */);
    
    let result = FeedbackModal.on_submit(interaction, ctx).await;
    
    assert!(result.is_ok());
}
```

---

## Troubleshooting

### Errore: "Modal not found"

**Causa:** Custom ID del modal non registrato nel bot.

**Soluzione:**
```rust
BotBuilder::new()
    .register_modal(FeedbackModal)  // Assicurati di registrare!
    .build();
```

### Errore: "Validation failed"

**Causa:** Campo obbligatorio non compilato o validazione custom fallita.

**Soluzione:**
1. Verifica che tutti i campi `required=true` siano compilati
2. Aggiungi logging nel `on_submit()` per debug
3. Usa `.get_values_as_map()` per vedere tutti i valori

### Errore: "Max length exceeded"

**Causa:** Input supera il limite di caratteri impostato.

**Soluzione:**
1. Aumenta `max_length()` (max 4000)
2. Usa `.short_length(true)` per input brevi (≤60 chars UI optimized)

---

## Roadmap Futura

### Planned Features:

- ✅ **Modals** - Implemented (MVP)
- ⏳ **Follow-up Messages** - Webhook responses after modal submit
- ⏳ **File Attachments in Modals** - Upload files durante submission
- ⏳ **Autocomplete Enhancement** - Richiedi autocomplete per campi specifici

### Not Implemented:

- ❌ **Presence Updates** - Richiede Gateway events (WASM incompatibile)
- ❌ **Activities** - Richiede Gateway events (WASM incompatibile)  
- ❌ **Voice Channels** - Richiede Gateway events

---

## Risorse Aggiuntive

### Documentazione Correlata:

- [docs/COMMANDS.md](<../../../docs/COMMANDS.md>) - Command trait
- [docs/COMPONENTS.md](<../../../docs/COMPONENTS.md>) - Componenti messaggi
- [GETTING_STARTED.md](<../../../GETTING_STARTED.md>) - Setup rapido

### Esempi:

- [examples/modals/src/main.rs](<../../../examples/modals/src/main.rs>) - Implementazione completa
- [examples/with_commands/](<../../../examples/with_commands/>) - Command + Components integration

### Discord API Docs:

- Modal Interactions: [Discord Developer Portal](https://discord.com/developers/docs/interactions/modal-interactions)
- Message Components: [Components docs](https://discord.com/developers/docs/interactions/message-components)

---

**Version:** 1.0 (MVP - Modals only)  
**Last Updated:** 2026-08-23  
**Status:** Stable, WASM-compatible