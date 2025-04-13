# AviScript

AviScript is a domain-specific language (DSL) for building voice assistant skills, powered by the Rhai scripting engine. It provides a simple, expressive syntax for handling intents, managing dialogue flow, and interacting with various voice assistant capabilities.

This is just a package to be use by my voice assistant Avi.
## Overview

AviScript enables developers to create interactive voice experiences with features like:

- Intent handling and slot extraction
- Multilingual support with translation management
- Asset management (audio, images, text)
- Context and state management
- HTTP requests
- Event system

## Project Structure

```
my_skill/
├── skill.avi                  # Main logic script
├── metadata.avi               # Skill metadata (name, author, version, etc.)
├── config/
│   ├── default.json           # Default config structure and values
│   └── runtime.json           # User-edited or stored config
├── intents/
│   ├── main.intent            # Intent definition for main activation
│   └── other.intent           # Additional intents
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
│   └── skill_test.avi         # Tests in the DSL or JSON test format
└── README.md                  # Human-readable doc for this skill
```

## Key Modules

AviScript provides the following built-in modules:

- `speak`: Text-to-speech capabilities
- `ask`: User input handling
- `intent`: Intent and slot management
- `assets`: Asset management
- `audio`: Audio playback control
- `context`: State management
- `config`: Configuration handling
- `setting`: User settings
- `http`: Network requests
- `events`: Event emission and handling
- `utils`: Utility functions
- `translation`: Internationalization support

## Example Usage

```
// Handle a weather intent
on_intent "get_weather" |intent| {
    let city = intent.get("city") or "your location";
    speak.say("weather_check", #{ "city": city });
    
    let weather_data = http.get("https://api.weather.com/current", #{ 
        "location": city 
    });
    
    if weather_data.status == 200 {
        speak.say("weather_result", #{ 
            "temp": weather_data.temperature,
            "condition": weather_data.condition
        });
    } else {
        speak.say("weather_error", #{});
    }
}

// Ask a question and handle response
fn ask_name() {
    ask.question("name_prompt", |response| {
        speak.say("greeting", #{ "name": response });
        context.save("user_name", response);
    }, #{}, "");
}

// Translation example
fn greet_user() {
    let name = context.load("user_name") or "friend";
    let greeting = translation.get("hello_message", #{ "user_name": name });
    speak.text(greeting);
}
```

## License

[Package license information]