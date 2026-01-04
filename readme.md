# Algorithms Learning System / Система изучения алгоритмов

Профессиональная система для изучения и сравнения алгоритмов на Rust.  
Структурированный путь от основ до продвинутых тем с акцентом на практическое понимание.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Progress](https://img.shields.io/badge/Progress-3.3%25-yellow)](https://github.com/Grapple228/algorithms)
[![Last Solved](https://img.shields.io/badge/Last%20Solved-Group%20Anagrams-blue)](https://leetcode.com/problems/contains-duplicate/)
[![Last Updated](https://img.shields.io/badge/Last%20Updated-January%202025-blue)](https://github.com/Grapple228/algorithms)

## 📋 О проекте

Это не просто коллекция решений LeetCode, а **полноценная система обучения** с:

- 🧠 **Сравнением подходов** (минимум 2 решения на задачу)
- 📊 **Бенчмарками производительности** (реальные измерения)
- 🏗️ **Масштабируемой структурой** (готово для 2000+ задач)
- 📈 **Аналитикой паттернов** (практические инсайты)

## 🎯 Ключевые особенности

### 🧩 Структурированный план обучения

15 модулей от простого к сложному согласно [плану обучения](docs/LEARNING_PATH.md):

1. **Hash Tables** (хэш-таблицы)
2. **Two Pointers** (два указателя)
3. **Binary Search** (бинарный поиск)
4. **Sliding Window** (скользящее окно)
5. **Stacks & Queues** (стеки и очереди)
6. **Linked Lists** (связные списки)
7. **Trees** (деревья)
8. **Graphs** (графы)
9. **Heaps** (кучи)
10. **Dynamic Programming** (динамическое программирование)
11. **Backtracking** (поиск с возвратом)
12. **Greedy Algorithms** (жадные алгоритмы)
13. **Intervals** (интервалы)
14. **Trie & Bit Manipulation** (префиксные деревья и битовые операции)
15. **Advanced Patterns** (продвинутые паттерны)

### 📊 Сравнение производительности

Каждая задача решается минимум двумя способами с измерением времени:

```rust
// Пример для Two Sum:
// Brute Force (O(n²)) vs HashMap (O(n))
test_all_solutions_for_cases!(
    test_cases(),
    |(nums, target): Input| solution1_brute_force(nums, target),
    |(nums, target): Input| solution2_hashmap(nums, target),
);
```

### 🔍 Система позволяет обнаруживать паттерны

Для HashMap/HashSet:
n ≤ 20: Brute force быстрее (накладные расходы!)
n ≈ 50-100: Переломный момент
n ≥ 100: Hash-структуры в 5-40x быстрее

## 👥 Для кого этот проект

### 🎓 Начинающие программисты

- Пошаговый план изучения алгоритмов
- Сравнение разных подходов
- Практические примеры на Rust

### 🚀 Опытные разработчики

- Глубокое понимание производительности
- Готовые шаблоны для новых задач
- Система бенчмаркинга

### 🔍 Готовящиеся к собеседованиям

- LeetCode задачи с разбором
- Анализ сложности алгоритмов
- Ключевые инсайты по каждой теме

## 📊 Текущий прогресс

### 📈 Общая статистика

| Метрика                 | Значение | Прогресс |
| ----------------------- | -------- | -------- |
| **Всего задач в плане** | 120      | 100%     |
| **Решено задач**        | 4        | 3.3%     |
| **Начато модулей**      | 1 из 15  | 6.7%     |

---

### 🏗️ Прогресс по модулям

#### 🟢 **Модуль 1: Hash Tables** (4/7 задач, 57%)

| #   | Задача                                                                                      | Статус       | Сложность | Подходы                                                                    | Ключевые инсайты                                                                             |
| --- | ------------------------------------------------------------------------------------------- | ------------ | --------- | -------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| 1   | [Two Sum](https://leetcode.com/problems/two-sum/)                                           | ✅ Решена    | 🟢 Easy   | 1. Brute Force<br>2. HashMap                                               | • HashMap выигрывает при n ≥ 100<br>• Trade-off: память ↔ время                              |
| 217 | [Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)                     | ✅ Решена    | 🟢 Easy   | 1. Brute Force<br>2. HashSet                                               | • HashSet::insert() возвращает bool<br>• Тот же переломный момент n=50-100                   |
| 242 | [Valid Anagram](https://leetcode.com/problems/valid-anagram/)                               | ✅ Решена    | 🟢 Easy   | 1. Frequency array<br>2. Hashmap                                           | • Frequency array на 40% быстрее Hashmap<br>• [u8;26] для ASCII<br>• Hashmap для Unicode     |
| 49  | [Group Anagrams](https://leetcode.com/problems/group-anagrams/)                             | ✅ Решена    | 🟡 Medium | 1. HashMap<[u32;26],...><br>2. HashMap<[u8;26],...><br>3. Оптимизированный | • [u8;26] в 2.1x быстрее [u32;26]<br>• Преаллокация дает +10%<br>• One-pass подход оптимален |
| 349 | [Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/)     | ⏳ В очереди | 🟢 Easy   | -                                                                          | -                                                                                            |
| 128 | [Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/) | ⏳ В очереди | 🔴 Hard   | -                                                                          | -                                                                                            |
| 560 | [Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/)               | ⏳ В очереди | 🟡 Medium | -                                                                          | -                                                                                            |

#### ⚪ **Модуль 2: Two Pointers** (0/6 задач, 0%)

| #   | Задача                    | Статус       | Сложность |
| --- | ------------------------- | ------------ | --------- |
| 125 | Valid Palindrome          | ⏳ В очереди | 🟢 Easy   |
| 167 | Two Sum II                | ⏳ В очереди | 🟡 Medium |
| 15  | 3Sum                      | ⏳ В очереди | 🟡 Medium |
| 11  | Container With Most Water | ⏳ В очереди | 🟡 Medium |
| 42  | Trapping Rain Water       | ⏳ В очереди | 🔴 Hard   |
| 16  | 3Sum Closest              | ⏳ В очереди | 🟡 Medium |

_Остальные 13 модулей пока не начаты_

---

## 🏗️ Структура проекта

```text
algorithms/
├── src/
│   ├── _01_hash_tables/          # Модуль 1: Хэш-таблицы
│   │   ├── about.md              # Теория модуля
│   │   ├── mod.rs               # Экспорт задач модуля
│   │   ├── p001_two_sum/        # Задача #1
│   │   │   ├── mod.rs           # Код + тесты + бенчмарки
│   │   │   └── problem.md       # Описание задачи
│   │   ├──  p049_group_anagrams/  # Задача #049
│   │   ├── p217_contains_duplicate/  # Задача #217
│   │   └── p242_valid_anagram/  # Задача #242
│   ├── utils/                   # Вспомогательные утилиты
│   │   ├── bench.rs            # Бенчмаркинг
│   │   ├── macros.rs           # Макросы для тестирования
│   │   └── mod.rs
│   └── lib.rs
├── docs/
│   ├── LEARNING_PATH.md        # Подробный план обучения
│   └── PROGRESS_TRACKER.md     # Отслеживание прогресса
├── templates/                  # Шаблоны для новых задач
│   ├── module.md             # Шаблон документации модуля
│   ├── problem.rs             # Шаблон кода задачи
│   └── problem.md             # Шаблон документации
└── Cargo.toml
```

## 🚀 Быстрый старт

```sh
# Клонировать репозиторий
git clone https://github.com/Grapple228/algorithms.git
cd algorithms

# Запустить тесты
cargo test

# Запустить бенчмарки (требует feature)
cargo test --features bench

# Запустить тесты конкретной задачи
cargo test -p algorithms _01_hash_tables::p001_two_sum

# Запустить тесты с выводом
cargo test -- --nocapture
```

## 🔧 Фичи

- `bench` - бенчмарки производительности
  Включает подробное сравнение решений с анализом производительности

```sh
# Запуск бенчмарков
cargo test --features bench -- --nocapture

# Пример вывода:
┌───────────────────────────────────────────┐
│          Performance Comparison           │
├────────────┬────────────┬─────────────────┤
│ Solution   │ Avg Time   │ Operations/sec  │
├────────────┼────────────┼─────────────────┤
│ brute      │ 26 ns      │ 38803306 ops/sec │
│ hashmap    │ 73 ns      │ 13708959 ops/sec │
└────────────┴────────────┴─────────────────┘
```

## 🔄 Workflow разработки

1. Добавить требуемый модуль (по желанию с документацией из [`templates/module.md`](./templates/module.md))
2. Создать задачу из шаблонов:

- Код: [`templates/problem.rs`](./templates/problem.rs)
- Документация: [`templates/problem.md`](./templates/problem.md)

3. Настроить типы - обновить `Input` и `Output` под задачу
4. Реализовать логику, желательно от 2-х
5. Добавить `test_cases` - покрыть `edge_cases`
6. Сравнить решения - через бенчмарки
7. Отправить в LeetCode - проверить корректность

## 📅 Планы на будущее

- Добавить измерение памяти - отслеживание использования RAM
- Автоматизация создания задач - скрипт для генерации структуры
- Интеграция с LeetCode API - автоматическая загрузка test cases
- Визуализация результатов - графики сравнения алгоритмов
- Система повторения - spaced repetition для закрепления

## 📄 Лицензия

Этот проект лицензирован под лицензией MIT.
Подробности в файле [LICENSE](LICENSE)

---

### Создано с ❤️ для системного изучения алгоритмов. Буду рад если кому-то пригодится!

### ⭐ Если проект вам полезен — поставьте звезду! ⭐
