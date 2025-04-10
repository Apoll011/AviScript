## 📁 `my_skill/` – Root of the Skill
The entire skill lives here.

```
my_skill/
├── skill.avi                   # Main logic script in your custom DSL
├── metadata.avi               # Skill metadata (name, author, version, etc.)
├── config/
│   ├── default.json           # Default config structure and values
│   └── runtime.json           # User-edited or stored config
├── intents/
│   ├── intent1.intent       # Intent definition for main activation
│   └── intent2.intent      # Additional intents
├── responses/
│   ├── en.lang                # English responses
│   ├── es.lang                # Spanish responses
│   └── fr.lang                # French responses
├── assets/
│   ├── audio/
│   │   ├── intro.wav
│   │   ├── success.mp3
│   │   └── error.mp3
│   ├── images/
│   │   └── logo.png
│   └── other/
│       └── data.csv
├── extensions/
│   └── custom.rs              # Optional Rust extensions for heavy lifting
├── tests/
│   └── skill_test.avi         # Tests
└── README.md                  # Human-readable doc for this skill
```

---

### 🧠 `skill.avi`
Your main script logic using `.avi` DSL.
---

### 📇 `metadata.avi`

Defines who made the skill and what it does:

```avi
name = "Greeting Skill"
id = "greet.skill"
version = "1.0.0"
author = "You"
description = "Greets the user and responds with kind words."
language = ["en", "es"]
license = "MIT"
```

---

### ⚙️ `config/`

- `default.json`: defines the **structure** and **defaults**.
- `runtime.json`: actual user-set values after install.

```json
{
  "config": {
    "greeting_style": {
      "type": "text",
      "value": "friendly",
      "default": "friendly"
    },
    "play_sound": {
      "type": "bool",
      "value": true,
      "default": true
    }
  }
}
```

---

### 💬 `intents/`

Each file defines an intent and its slot structure.

---

### 🔈 `responses/en.lang`

Language responses mapped to keys.

```
hello_message:Hello there, {user_name}!
ask_name:What is your name?
bye: Goodbye!
```

---

### 🎧 `assets/`

Organized into types: `audio`, `images`, `other`.

Use them in `.avi` like:

```avi
play_audio("intro.wav")
```

---

### 🧩 `extensions/`

Rust or native code you optionally bind into `.avi`.

---

### 🧪 `tests/`

Optional automated testing for skills.

---

### 🔍 `README.md`

Explain purpose, usage, or slot examples.

---

And they can be compiled