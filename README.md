# LocalTransport
Мой первый rust-проект!

[![Статус сборки](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)](https://example.com)
[![Лицензия](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)](LICENSE)
[![Версия](https://img.shields.io/badge/version-1.0.0-orange?style=for-the-badge)](https://example.com)

Простой p2p отправитель файлов по локальной сети. (CLI)

---

## Содержание

- [Возможности](#возможности)
- [Быстрый старт](#быстрый-старт)

---

## Возможности

| Функция | Готовность |
|---------|----------|
| Отправка единичного файла | Работает |
| Отправка папки и подпапок | Работает, отправляет только файлы, без папок |
| TUI | Скоро... |

---

## Быстрый старт

### Требования

- rust 2024

### Установка и запуск

```bash
git clone https://github.com/maxsimka4234/LocalTransport.git
cd LocalTransport
cargo run
```
## Лицензия
MIT License

Copyright (c) [2026] [maxsimka]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.