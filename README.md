# Whisper2

**Work in progess**<br>
Small and simple desktop chat client for large language models.
Only Ollama is currently supported.

## Compiling
Development:
```
npm run tauri dev
```

Release (single executable):
```
npm run tauri build -- --no-bundle
```

## TODO
- [x] Styling of assistant responses
- [x] Persistence of chats
- [x] Cancelation of chat completions
- [ ] Configuration of Ollama backend
- [ ] Import and Export of Chats
- [ ] Proper dark mode
- [ ] Support for vision models
