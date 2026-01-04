# План изучения алгоритмов

## 📌 Общая стратегия

- **1 тема = 1 неделя** (можно адаптировать под свой темп)
- **5-7 задач на тему** для понимания паттерна
- **1 сложная задача** для закрепления
- **Повторение через неделю** для закрепления материала
- Всегда писать **минимум 2 решения** для сравнения подходов

---

## 🎯 Рекомендации по прохождению

1. **Следуйте порядку** — темы построены от простого к сложному
2. **Не пропускайте сложные задачи** — они закрепляют понимание
3. **Ведите дневник прогресса** в `docs/PROGRESS_TRACKER.md`
4. **Повторяйте через неделю** — решайте те же задачи без подсказок
5. **Сравнивайте решения** — всегда пишите минимум 2 варианта
6. **Анализируйте производительность** — используйте встроенные бенчмарки
7. **Делайте заметки** — записывайте ключевые инсайты для каждой темы

---

## 🏋️ Как эффективно практиковаться:

### Перед решением:

1. **Прочитайте задачу 3 раза** - убедитесь что понимаете все edge cases
2. **Напишите тесты** - до написания решения
3. **Продумайте подходы** - минимум 2 разных подхода

### Во время решения:

1. **Пишите наивное решение** - даже если знаете оптимальное
2. **Оптимизируйте шаг за шагом** - не сразу прыгайте к финальной версии
3. **Комментируйте сложные части** - объясняйте себе почему это работает

### После решения:

1. **Сравните с решениями других** - но только после своей реализации
2. **Проанализируйте производительность** - запустите бенчмарки
3. **Запишите ключевые инсайты** - в таблицу прогресса

---

## Оглавление

1. [Hash Tables](#📚-модуль-1-hash-tables-хэш-таблицы)
2. [Two Pointers](#📚-модуль-2-two-pointers-два-указателя)
3. [Binary Search](#📚-модуль-3-binary-search-бинарный-поиск)
4. [Sliding Window](#📚-модуль-4-sliding-window-оконный-метод)
5. [Stacks & Queues](#📚-модуль-5-stacks--queues-стеки-и-очереди)
6. [Linked Lists](#📚-модуль-6-linked-lists-связные-списки)
7. [Trees](#📚-модуль-7-trees-деревья)
8. [Graphs](#📚-модуль-8-graphs-графы)
9. [Heaps](#📚-модуль-9-heaps-кучиприоритетные-очереди)
10. [Dynamic Programming](#📚-модуль-10-dynamic-programming-динамическое-программирование)
11. [Backtracking](#📚-модуль-11-backtracking-поиск-с-возвратом)
12. [Greedy Algorithms](#📚-модуль-12-greedy-algorithms-жадные-алгоритмы)
13. [Intervals](#📚-модуль-13-intervals-интервалы)
14. [Trie & Bit Manipulation](#📚-модуль-14-trie--bit-manipulation)
15. [Advanced Patterns](#📚-модуль-15-advanced-patterns-продвинутые-паттерны)

---

## 📚 Модуль 1: [Hash Tables](../src/_01_hash_tables/about.md) (Хэш-таблицы)

### 🎯 Когда использовать

- Нужен O(1) доступ к данным
- Подсчёт частот элементов
- Поиск дубликатов или пар
- Группировка элементов по признаку

### 📊 Основные задачи (обязательные)

1. **[Two Sum](https://leetcode.com/problems/two-sum/)** (#1) - поиск пары чисел с заданной суммой

   - **Подходы:** HashMap one-pass, HashMap two-pass, Brute force
   - **Ключевой инсайт:** Обмен памяти на время

2. **[Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)** (#217) - проверка на дубликаты

   - **Подходы:** HashSet, Sorting + linear scan
   - **Ключевой инсайт:** Базовая работа с множествами

3. **[Valid Anagram](https://leetcode.com/problems/valid-anagram/)** (#242) - проверка анаграмм

   - **Подходы:** Frequency array (26 элементов), Sorting, HashMap
   - **Ключевой инсайт:** Подсчёт частот символов

4. **[Group Anagrams](https://leetcode.com/problems/group-anagrams/)** (#49) - группировка анаграмм

   - **Подходы:** Сортировка строки как ключ, Frequency count как ключ
   - **Ключевой инсайт:** Выбор хорошего ключа для группировки

5. **[Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/)** (#349) - пересечение массивов
   - **Подходы:** Два HashSet'а, Sorting + two pointers
   - **Ключевой инсайт:** Операции над множествами

### 🚀 Сложные задачи для закрепления

1. **[Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/)** (#128) - **Hard**

   - **Подходы:** HashSet с поиском начала последовательностей
   - **Сложность:** O(n) время, O(n) память
   - **Ключевой инсайт:** Поиск в HashSet без сортировки массива

2. **[Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/)** (#560) - **Medium**

   - **Подходы:** Prefix Sum + HashMap
   - **Сложность:** O(n) время, O(n) память
   - **Ключевой инсайт:** Cumulative sums и их разницы

3. **[Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/)** (#76) - **Hard**
   - **Подходы:** Sliding Window + HashMap частот
   - **Сложность:** O(n) время, O(k) память (k = размер алфавита)
   - **Ключевой инсайт:** Два указателя с условиями на частоты

---

## 📚 Модуль 2: [Two Pointers](../src/_02_two_pointers/about.md) (Два указателя)

### 🎯 Когда использовать

- Массив отсортирован (или можно отсортировать)
- Нужно найти пару/тройку элементов
- Работа с палиндромами
- Задачи на слияние интервалов

### 📊 Основные задачи (обязательные)

1. **[Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)** (#125) - проверка палиндрома

   - **Подходы:** Two pointers с пропуском символов
   - **Ключевой инсайт:** Работа с неалфавитными символами

2. **[Two Sum II](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)** (#167) - Two Sum для отсортированного массива

   - **Подходы:** Two pointers с краёв, Binary search для каждого элемента
   - **Ключевой инсайт:** Сравнение с HashMap решением из модуля 1

3. **[3Sum](https://leetcode.com/problems/3sum/)** (#15) - тройка чисел с нулевой суммой

   - **Подходы:** Сортировка + фиксация + two pointers
   - **Ключевой инсайт:** Пропуск дубликатов

4. **[Container With Most Water](https://leetcode.com/problems/container-with-most-water/)** (#11) - максимальная площадь

   - **Подходы:** Two pointers с движением более короткого
   - **Ключевой инсайт:** Greedy выбор указателя

5. **[Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)** (#42) - сбор дождевой воды
   - **Подходы:** Two pointers с максимумами, Dynamic programming
   - **Ключевой инсайт:** Уровень воды определяется минимумом максимумов

### 🚀 Сложные задачи для закрепления

1. **[3Sum Closest](https://leetcode.com/problems/3sum-closest/)** (#16) - **Medium**

   - **Подходы:** Сортировка + фиксация + two pointers
   - **Ключевой инсайт:** Отслеживание минимальной разницы

2. **[4Sum](https://leetcode.com/problems/4sum/)** (#18) - **Medium**

   - **Подходы:** Двойная фиксация + two pointers
   - **Ключевой инсайт:** Обобщение на k-Sum

3. **[Sort Colors](https://leetcode.com/problems/sort-colors/)** (#75) - **Medium** (Dutch Flag Problem)
   - **Подходы:** Three pointers (Dutch National Flag)
   - **Ключевой инсайт:** In-place сортировка за один проход

---

## 📚 Модуль 3: [Binary Search](../src/_03_binary_search/about.md) (Бинарный поиск)

### 🎯 Когда использовать

- Массив отсортирован
- Нужно найти элемент или границу
- Можно представить монотонную функцию
- Пространство ответов можно разделить пополам

### 📊 Основные задачи (обязательные)

1. **[Binary Search](https://leetcode.com/problems/binary-search/)** (#704) - классический поиск

   - **Подходы:** while left <= right, while left < right
   - **Ключевой инсайт:** Шаблон без ошибок в границах

2. **[Search Insert Position](https://leetcode.com/problems/search-insert-position/)** (#35) - поиск позиции вставки

   - **Подходы:** Модификация бинарного поиска
   - **Ключевой инсайт:** Понимание left/right после цикла

3. **[First Bad Version](https://leetcode.com/problems/first-bad-version/)** (#278) - поиск первой true

   - **Подходы:** while left < right шаблон
   - **Ключевой инсайт:** Поиск границы в [false...true]

4. **[Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)** (#153) - поиск в повёрнутом массиве

   - **Подходы:** Анализ mid относительно краёв
   - **Ключевой инсайт:** Определение отсортированной половины

5. **[Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)** (#33) - поиск в повёрнутом

   - **Подходы:** Определение какой половина отсортирована + поиск в ней
   - **Ключевой инсайт:** Двойное условие

### 🚀 Сложные задачи для закрепления

1. **[Find First and Last Position](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/)** (#34) - **Medium**

   - **Подходы:** Два бинарных поиска (левая и правая граница)
   - **Ключевой инсайт:** Поиск границы с разными условиями

2. **[Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)** (#4) - **Hard**

   - **Подходы:** Бинарный поиск по меньшему массиву
   - **Ключевой инсайт:** Разделение на две равные части

3. **[Split Array Largest Sum](https://leetcode.com/problems/split-array-largest-sum/)** (#410) - **Hard**

   - **Подходы:** Бинарный поиск по ответу + greedy проверка
   - **Ключевой инсайт:** Бинарный поиск по пространству ответов

---

## 📚 Модуль 4: [Sliding Window](../src/_04_sliding_window/about.md) (Оконный метод)

### 🎯 Когда использовать

- Задачи на подмассивы/подстроки
- Нужно найти минимум/максимум окна
- Окно фиксированного или переменного размера
- Подсчёт элементов в окне

### 📊 Основные задачи (обязательные)

1. **[Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/)** (#643) - фиксированное окно

   - **Подходы:** Sliding window с фиксированным размером
   - **Ключевой инсайт:** Пересчёт суммы за O(1)

2. **[Longest Substring Without Repeating](https://leetcode.com/problems/longest-substring-without-repeating-characters/)** (#3) - переменное окно

   - **Подходы:** HashSet для отслеживания символов
   - **Ключевой инсайт:** Сдвиг left при дубликате

3. **[Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/)** (#209) - переменное окно

   - **Подходы:** Расширение right, сжатие left
   - **Ключевой инсайт:** Поиск минимального окна

4. **[Permutation in String](https://leetcode.com/problems/permutation-in-string/)** (#567) - фиксированное окно с частотой

   - **Подходы:** Frequency array/hash
   - **Ключевой инсайт:** Сравнение частот символов

5. **[Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/)** (#424) - окно с заменой
   - **Подходы:** Отслеживание max_count символа
   - **Ключевой инсайт:** Формула: window_len - max_count <= k

### 🚀 Сложные задачи для закрепления

1. **[Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/)** (#76) - **Hard**

   - **Подходы:** Два frequency maps, расширение/сжатие окна
   - **Ключевой инсайт:** Условие на наличие всех нужных символов

2. **[Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/)** (#239) - **Hard**

   - **Подходы:** Монотонная очередь (deque)
   - **Ключевой инсайт:** Поддержка максимума в окне за O(1)

3. **[Subarrays with K Different Integers](https://leetcode.com/problems/subarrays-with-k-different-integers/)** (#992) - **Hard**

   - **Подходы:** Exactly K = AtMost(K) - AtMost(K-1)
   - **Ключевой инсайт:** Преобразование условия

---

## 📚 Модуль 5: [Stacks & Queues](../src/_05_stacks_queues/about.md) (Стеки и очереди)

### 🎯 Когда использовать

- Обработка вложенных структур (скобки)
- Нужен LIFO или FIFO порядок
- Задачи на next greater element
- BFS обход графов
- Отложенные вычисления

### 📊 Основные задачи (обязательные)

1. **[Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)** (#20) - проверка скобок

   - **Подходы:** Стек для сопоставления
   - **Ключевой инсайт:** Сопоставление открывающих и закрывающих

2. **[Min Stack](https://leetcode.com/problems/min-stack/)** (#155) - стек с поддержкой минимума

   - **Подходы:** Два стека или один с парами
   - **Ключевой инсайт:** O(1) для всех операций

3. **[Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)** (#739) - next greater temperature

   - **Подходы:** Монотонный убывающий стек
   - **Ключевой инсайт:** Хранение индексов в стеке

4. **[Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/)** (#232) - очередь на стеках

   - **Подходы:** Два стека (input/output)
   - **Ключевой инсайт:** Амортизированная O(1)

5. **[Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/)** (#150) - оценка RPN
   - **Подходы:** Стек для операторов и операндов
   - **Ключевой инсайт:** Обработка постфиксной записи

### 🚀 Сложные задачи для закрепления

1. **[Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/)** (#84) - **Hard**

   - **Подходы:** Монотонный возрастающий стек
   - **Ключевой инсайт:** Вычисление площади для каждого барьера

2. **[Basic Calculator](https://leetcode.com/problems/basic-calculator/)** (#224) - **Hard**

   - **Подходы:** Стек для скобок и знаков
   - **Ключевой инсайт:** Обработка унарных операторов и приоритетов

3. **[Design Circular Queue](https://leetcode.com/problems/design-circular-queue/)** (#622) - **Medium**

   - **Подходы:** Кольцевой буфер на массиве
   - **Ключевой инсайт:** Управление head/tail индексами

---

## 📚 Модуль 6: [Linked Lists](../src/_06_linked_lists/about.md) (Связные списки)

### 🎯 Когда использовать

- Нужны частые вставки/удаления в середине
- Задачи на перестановку узлов
- Работа с циклами
- Реализация LRU Cache
- Объединение отсортированных списков

### 📊 Основные задачи (обязательные)

1. **[Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)** (#206) - разворот списка

   - **Подходы:** Три указателя (prev, curr, next), рекурсия
   - **Ключевой инсайт:** Сохранение ссылок перед изменением

2. **[Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/)** (#141) - обнаружение цикла

   - **Подходы:** Floyd's Cycle Detection (fast & slow)
   - **Ключевой инсайт:** Расстояние до точки встречи

3. **[Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)** (#21) - слияние списков

   - **Подходы:** Dummy node для упрощения
   - **Ключевой инсайт:** Построение нового списка

4. **[Remove Nth Node From End](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)** (#19) - удаление n-го с конца

   - **Подходы:** Два указателя с разницей n
   - **Ключевой инсайт:** Dummy node для edge cases

5. **[Copy List with Random Pointer](https://leetcode.com/problems/copy-list-with-random-pointer/)** (#138) - копирование
   - **Подходы:** Два прохода + HashMap, чередование узлов
   - **Ключевой инсайт:** Связывание старых и новых узлов

### 🚀 Сложные задачи для закрепления

1. **[LRU Cache](https://leetcode.com/problems/lru-cache/)** (#146) - **Hard**

   - **Подходы:** HashMap + Doubly Linked List
   - **Ключевой инсайт:** O(1) для get и put операций

2. **[Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)** (#23) - **Hard**

   - **Подходы:** Priority Queue (min-heap), Divide and conquer
   - **Ключевой инсайт:** Слияние за O(n log k)

3. **[Reverse Nodes in k-Group](https://leetcode.com/problems/reverse-nodes-in-k-group/)** (#25) - **Hard**

   - **Подходы:** Рекурсивное обращение групп
   - **Ключевой инсайт:** Связывание частей после реверса

---

## 📚 Модуль 7: [Trees](../src/_07_trees/about.md) (Деревья)

### 🎯 Когда использовать

- Иерархические данные
- Нужен быстрый поиск/вставка (BST)
- Обход в глубину/ширину
- Задачи на пути и суммы
- Сбалансированные структуры данных

### 📊 Основные задачи (обязательные)

1. **[Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)** (#104) - максимальная глубина

   - **Подходы:** Рекурсивный DFS, BFS по уровням
   - **Ключевой инсайт:** Базовый рекурсивный паттерн

2. **[Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)** (#98) - проверка BST

   - **Подходы:** In-order обход, рекурсия с границами
   - **Ключевой инсайт:** Проверка свойств BST для каждого узла

3. **[Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)** (#102) - обход по уровням

   - **Подходы:** BFS с очередью
   - **Ключевой инсайт:** Сбор значений по уровням

4. **[Lowest Common Ancestor](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)** (#236) - LCA

   - **Подходы:** Рекурсивный поиск в поддеревьях
   - **Ключевой инсайт:** Возврат узлов или null

5. **[Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/)** (#124) - максимальная сумма пути
   - **Подходы:** Рекурсия с возвратом максимальной ветви
   - **Ключевой инсайт:** Обновление глобального максимума

### 🚀 Сложные задачи для закрепления

1. **[Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)** (#297) - **Hard**

   - **Подходы:** Pre-order обход с null markers
   - **Ключевой инсайт:** Восстановление структуры из последовательности

2. **[Binary Tree Cameras](https://leetcode.com/problems/binary-tree-cameras/)** (#968) - **Hard**

   - **Подходы:** DFS с возвратом состояния
   - **Ключевой инсайт:** Greedy размещение камер снизу вверх

3. **[Count Complete Tree Nodes](https://leetcode.com/problems/count-complete-tree-nodes/)** (#222) - **Medium**

   - **Подходы:** Использование свойств complete tree
   - **Ключевой инсайт:** Бинарный поиск по высоте дерева

---

## 📚 Модуль 8: [Graphs](../src/_08_graphs/about.md) (Графы)

### 🎯 Когда использовать

- Связи между объектами
- Поиск путей/связности
- Топологическая сортировка
- Shortest path problems
- Минимальное покрытие

### 📊 Основные задачи (обязательные)

1. **[Number of Islands](https://leetcode.com/problems/number-of-islands/)** (#200) - подсчёт островов

   - **Подходы:** DFS/BFS по матрице как графу
   - **Ключевой инсайт:** Помечение посещённых клеток

2. **[Clone Graph](https://leetcode.com/problems/clone-graph/)** (#133) - клонирование графа

   - **Подходы:** BFS/DFS + HashMap старый→новый
   - **Ключевой инсайт:** Работа с adjacency list

3. **[Course Schedule](https://leetcode.com/problems/course-schedule/)** (#207) - проверка циклов

   - **Подходы:** Топологическая сортировка (Kahn's algorithm)
   - **Ключевой инсайт:** DFS с обнаружением циклов

4. **[Rotting Oranges](https://leetcode.com/problems/rotting-oranges/)** (#994) - BFS по слоям

   - **Подходы:** Multi-source BFS
   - **Ключевой инсайт:** Отслеживание времени распространения

5. **[Network Delay Time](https://leetcode.com/problems/network-delay-time/)** (#743) - shortest path
   - **Подходы:** Dijkstra's algorithm с priority queue
   - **Ключевой инсайт:** Выбор ближайшего непосещённого узла

### 🚀 Сложные задачи для закрепления

1. **[Alien Dictionary](https://leetcode.com/problems/alien-dictionary/)** (#269) - **Hard**

   - **Подходы:** Построение графа из порядка букв
   - **Ключевой инсайт:** Топологическая сортировка с обнаружением циклов

2. **[Word Ladder](https://leetcode.com/problems/word-ladder/)** (#127) - **Hard**

   - **Подходы:** BFS по графу слов
   - **Ключевой инсайт:** Преобразование за один символ

3. **[Cheapest Flights Within K Stops](https://leetcode.com/problems/cheapest-flights-within-k-stops/)** (#787) - **Medium**

   - **Подходы:** BFS with pruning, Dijkstra с ограничением
   - **Ключевой инсайт:** Учёт количества остановок

---

## 📚 Модуль 9: [Heaps](../src/_09_heaps/about.md) (Кучи/Приоритетные очереди)

### 🎯 Когда использовать

- Нужно постоянно получать min/max
- Merge k sorted lists
- Find median from data stream
- Task scheduling
- Top K элементов

### 📊 Основные задачи (обязательные)

1. **[Kth Largest Element in Array](https://leetcode.com/problems/kth-largest-element-in-an-array/)** (#215) - k-й наибольший элемент

   - **Подходы:** QuickSelect, min-heap размера k
   - **Ключевой инсайт:** Trade-off время/память

2. **[Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/)** (#347) - k наиболее частых

   - **Подходы:** HashMap + Bucket sort, max-heap
   - **Ключевой инсайт:** O(n) vs O(n log k)

3. **[Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream/)** (#295) - медиана из потока

   - **Подходы:** Две кучи (max-heap + min-heap)
   - **Ключевой инсайт:** Балансировка размеров куч

4. **[Meeting Rooms II](https://leetcode.com/problems/meeting-rooms-ii/)** (#253) - минимальное количество комнат

   - **Подходы:** Сортировка + min-heap окончаний
   - **Ключевой инсайт:** Greedy распределение

5. **[Task Scheduler](https://leetcode.com/problems/task-scheduler/)** (#621) - планировщик задач
   - **Подходы:** Max-heap частот + очередь ожидания
   - **Ключевой инсайт:** Циклическое выполнение с cooldown

### 🚀 Сложные задачи для закрепления

1. **[Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)** (#23) - **Hard**

   - **Подходы:** Min-heap из голов списков
   - **Ключевой инсайт:** O(n log k) слияние

2. **[Sliding Window Median](https://leetcode.com/problems/sliding-window-median/)** (#480) - **Hard**

   - **Подходы:** Две кучи + lazy deletion
   - **Ключевой инсайт:** Поддержка медианы в скользящем окне

3. **[Minimum Cost to Hire K Workers](https://leetcode.com/problems/minimum-cost-to-hire-k-workers/)** (#857) - **Hard**

   - **Подходы:** Сортировка по ratio + max-heap качества
   - **Ключевой инсайт:** Поддержка суммы k наибольших качеств

---

## 📚 Модуль 10: [Dynamic Programming](../src/_10_dynamic_programming/about.md) (Динамическое программирование)

### 🎯 Когда использовать

- Задачи на оптимизацию (min/max)
- Подсчёт количества способов
- Overlapping subproblems
- Можно построить рекуррентное соотношение
- Оптимальная подструктура

### 📊 Основные задачи (обязательные)

1. **[Climbing Stairs](https://leetcode.com/problems/climbing-stairs/)** (#70) - количество способов

   - **Подходы:** dp[i] = dp[i-1] + dp[i-2]
   - **Ключевой инсайт:** Оптимизация памяти до O(1)

2. **[Coin Change](https://leetcode.com/problems/coin-change/)** (#322) - минимальное количество монет

   - **Подходы:** Unbounded knapsack
   - **Ключевой инсайт:** dp[amount] = min(dp[amount - coin] + 1)

3. **[Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/)** (#300) - LIS

   - **Подходы:** dp[i] = max(dp[j] + 1) где nums[j] < nums[i]
   - **Ключевой инсайт:** O(n²) и O(n log n) с patience sorting

4. **[Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/)** (#1143) - LCS

   - **Подходы:** 2D DP: dp[i][j] на основе предыдущих
   - **Ключевой инсайт:** Классика строковых DP

5. **[House Robber](https://leetcode.com/problems/house-robber/)** (#198) - максимальная сумма без соседей
   - **Подходы:** dp[i] = max(dp[i-1], dp[i-2] + nums[i])
   - **Ключевой инсайт:** Оптимизация до двух переменных

### 🚀 Сложные задачи для закрепления

1. **[Edit Distance](https://leetcode.com/problems/edit-distance/)** (#72) - **Hard**

   - **Подходы:** 2D DP для преобразования строк
   - **Ключевой инсайт:** Минимум из insert/delete/replace

2. **[Word Break](https://leetcode.com/problems/word-break/)** (#139) - **Medium**

   - **Подходы:** dp[i] = can segment first i chars
   - **Ключевой инсайт:** Проверка подстрок в словаре

3. **[Best Time to Buy/Sell Stock with Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)** (#309) - **Medium**

   - **Подходы:** State machine DP
   - **Ключевой инсайт:** Три состояния: hold, sold, rest

---

## 📚 Модуль 11: [Backtracking](../src/_11_backtracking/about.md) (Поиск с возвратом)

### 🎯 Когда использовать

- Перебор всех комбинаций/перестановок
- Задачи на размещение (N-Queens)
- Поиск с ограничениями
- Когда brute force слишком медленный
- Генерация всех возможных решений

### 📊 Основные задачи (обязательные)

1. **[Subsets](https://leetcode.com/problems/subsets/)** (#78) - все подмножества

   - **Подходы:** Рекурсивный backtracking
   - **Ключевой инсайт:** Include/exclude элемент

2. **[Permutations](https://leetcode.com/problems/permutations/)** (#46) - все перестановки

   - **Подходы:** Backtracking с swapping
   - **Ключевой инсайт:** Рекурсия по позициям

3. **[Combination Sum](https://leetcode.com/problems/combination-sum/)** (#39) - комбинации с суммой

   - **Подходы:** Backtracking с повторением элементов
   - **Ключевой инсайт:** Сортировка + pruning

4. **[Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)** (#22) - генерация скобок

   - **Подходы:** Backtracking с подсчётом открытых/закрытых
   - **Ключевой инсайт:** Валидация на лету

5. **[Word Search](https://leetcode.com/problems/word-search/)** (#79) - поиск слова в матрице
   - **Подходы:** DFS с backtracking по матрице
   - **Ключевой инсайт:** Помечение посещённых клеток

### 🚀 Сложные задачи для закрепления

1. **[N-Queens](https://leetcode.com/problems/n-queens/)** (#51) - **Hard**

   - **Подходы:** Backtracking по строкам
   - **Ключевой инсайт:** Проверка атак за O(1) с sets

2. **[Sudoku Solver](https://leetcode.com/problems/sudoku-solver/)** (#37) - **Hard**

   - **Подходы:** Backtracking с pruning
   - **Ключевой инсайт:** Быстрая проверка допустимости

3. **[Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/)** (#131) - **Medium**

   - **Подходы:** Backtracking + проверка палиндромов
   - **Ключевой инсайт:** Разбиение строки на палиндромы

---

## 📚 Модуль 12: [Greedy Algorithms](../src/_12_greedy/about.md) (Жадные алгоритмы)

### 🎯 Когда использовать

- Оптимальная подструктура
- Greedy choice property
- Нужно быстрое приближённое решение
- Когда можно доказать что greedy работает
- Локально оптимальный выбор ведёт к глобальному оптимуму

### 📊 Основные задачи (обязательные)

1. **[Jump Game](https://leetcode.com/problems/jump-game/)** (#55) - можно ли достичь конца

   - **Подходы:** Greedy: отслеживание максимально достижимого
   - **Ключевой инсайт:** O(n) решение

2. **[Jump Game II](https://leetcode.com/problems/jump-game-ii/)** (#45) - минимальное количество прыжков

   - **Подходы:** Greedy BFS по слоям
   - **Ключевой инсайт:** O(n) решение

3. **[Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)** (#53) - максимальная сумма подмассива

   - **Подходы:** Kadane's algorithm (greedy/DP)
   - **Ключевой инсайт:** Локальный и глобальный максимум

4. **[Task Scheduler](https://leetcode.com/problems/task-scheduler/)** (#621) - планирование задач

   - **Подходы:** Greedy по наиболее частым задачам
   - **Ключевой инсайт:** Формула для idle slots

5. **[Queue Reconstruction by Height](https://leetcode.com/problems/queue-reconstruction-by-height/)** (#406) - восстановление очереди
   - **Подходы:** Сортировка по height descending, k ascending
   - **Ключевой инсайт:** Вставка по индексу k

### 🚀 Сложные задачи для закрепления

1. **[Candy](https://leetcode.com/problems/candy/)** (#135) - **Hard**

   - **Подходы:** Два прохода: слева направо и справа налево
   - **Ключевой инсайт:** Локальные максимумы получают больше

2. **[Gas Station](https://leetcode.com/problems/gas-station/)** (#134) - **Medium**

   - **Подходы:** Greedy: если суммарный газ >= суммарную стоимость
   - **Ключевой инсайт:** Reset start при отрицательном балансе

3. **[Minimum Number of Arrows](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/)** (#452) - **Medium**

   - **Подходы:** Сортировка по end points
   - **Ключевой инсайт:** Greedy выбор точки стрельбы

---

## 📚 Модуль 13: [Intervals](../src/_13_intervals/about.md) (Интервалы)

### 🎯 Когда использовать

- Работа с временными интервалами
- Слияние перекрывающихся интервалов
- Поиск пересечений
- Минимизация перекрытий
- Планирование ресурсов

### 📊 Основные задачи (обязательные)

1. **[Merge Intervals](https://leetcode.com/problems/merge-intervals/)** (#56) - слияние перекрывающихся

   - **Подходы:** Сортировка по start
   - **Ключевой инсайт:** Построение результата слиянием

2. **[Insert Interval](https://leetcode.com/problems/insert-interval/)** (#57) - вставка интервала

   - **Подходы:** Три фазы: до, пересечение, после
   - **Ключевой инсайт:** O(n) in-place вставка

3. **[Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals/)** (#435) - минимум удалений

   - **Подходы:** Сортировка по end (greedy)
   - **Ключевой инсайт:** Выбор интервалов с наименьшим end

4. **[Meeting Rooms](https://leetcode.com/problems/meeting-rooms/)** (#252) - проверка пересечений

   - **Подходы:** Сортировка + проверка соседей
   - **Ключевой инсайт:** O(n log n) решение

5. **[Meeting Rooms II](https://leetcode.com/problems/meeting-rooms-ii/)** (#253) - минимальное количество комнат
   - **Подходы:** Сортировка + min-heap окончаний
   - **Ключевой инсайт:** Greedy распределение

### 🚀 Сложные задачи для закрепления

1. **[Minimum Interval to Include Each Query](https://leetcode.com/problems/minimum-interval-to-include-each-query/)** (#1851) - **Hard**

   - **Подходы:** Сортировка интервалов и queries
   - **Ключевой инсайт:** Min-heap по размеру интервала

2. **[Employee Free Time](https://leetcode.com/problems/employee-free-time/)** (#759) - **Hard**

   - **Подходы:** Merge всех интервалов + найти gaps
   - **Ключевой инсайт:** Работа с списками списков

3. **[Data Stream as Disjoint Intervals](https://leetcode.com/problems/data-stream-as-disjoint-intervals/)** (#352) - **Hard**

   - **Подходы:** Поддержка отсортированного списка интервалов
   - **Ключевой инсайт:** Эффективное добавление и слияние

---

## 📚 Модуль 14: [Trie & Bit Manipulation](../src/_14_trie_bit_manipulation/about.md)

### 🎯 Когда использовать

- Работа с префиксами строк (Trie)
- Эффективные битовые операции
- Сжатие состояний в битовые маски
- Задачи на subsets с bitmask
- Быстрые проверки наличия элементов

### 📊 Основные задачи (обязательные)

1. **[Implement Trie](https://leetcode.com/problems/implement-trie-prefix-tree/)** (#208) - реализация префиксного дерева

   - **Подходы:** Node с детьми и is_end
   - **Ключевой инсайт:** Insert, Search, StartsWith операции

2. **[Add and Search Words](https://leetcode.com/problems/design-add-and-search-words-data-structure/)** (#211) - поиск с wildcard

   - **Подходы:** Trie + backtracking при '.'
   - **Ключевой инсайт:** Рекурсивный поиск по Trie

3. **[Word Search II](https://leetcode.com/problems/word-search-ii/)** (#212) - поиск слов в матрице

   - **Подходы:** Trie + DFS backtracking
   - **Ключевой инсайт:** Удаление из Trie при нахождении

4. **[Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/)** (#191) - подсчёт единиц

   - **Подходы:** n & (n-1) trick
   - **Ключевой инсайт:** Brian Kernighan's algorithm

5. **[Reverse Bits](https://leetcode.com/problems/reverse-bits/)** (#190) - обращение битов
   - **Подходы:** Постепенное построение результата
   - **Ключевой инсайт:** Bit by bit reversal

### 🚀 Сложные задачи для закрепления

1. **[Maximum XOR of Two Numbers](https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/)** (#421) - **Medium**

   - **Подходы:** Trie для битов
   - **Ключевой инсайт:** Поиск противоположного бита

2. **[Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/)** (#371) - **Medium**

   - **Подходы:** Bit manipulation без +/-
   - **Ключевой инсайт:** Использование XOR и AND с shift

3. **[Count Words With Prefix](https://leetcode.com/problems/counting-words-with-a-given-prefix/)** (#2185) - **Easy**

   - **Подходы:** Trie с подсчётом слов в поддереве
   - **Ключевой инсайт:** Эффективные префиксные запросы

---

## 📚 Модуль 15: [Advanced Patterns](../src/_15_advanced/about.md) (Продвинутые паттерны)

### 🎯 Когда использовать

- Смешанные задачи, объединяющие несколько тем
- Нужно применить комбинацию алгоритмов
- Оптимизация сложных решений
- Решение реальных проблем из интервью

### 📊 Основные задачи (обязательные)

1. **[Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)** (#238) - произведение кроме себя

   - **Подходы:** Prefix & Suffix products
   - **Ключевой инсайт:** O(n) без деления

2. **[Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/)** (#287) - поиск дубликата

   - **Подходы:** Cycle Detection (Floyd's)
   - **Ключевой инсайт:** O(1) память, O(n) время

3. **[First Missing Positive](https://leetcode.com/problems/first-missing-positive/)** (#41) - первое пропущенное положительное

   - **Подходы:** In-place marking с использованием индексов
   - **Ключевой инсайт:** O(n) время, O(1) память

4. **[Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)** (#42) - сбор дождевой воды

   - **Подходы:** Two Pointers/DP
   - **Ключевой инсайт:** Разные подходы к одной задаче

5. **[Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/)** (#84) - максимальный прямоугольник
   - **Подходы:** Монотонный стек
   - **Ключевой инсайт:** Вычисление площади для каждого барьера

### 🚀 Сложные задачи для закрепления

1. **[Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/)** (#10) - **Hard**

   - **Подходы:** 2D DP с обработкой '\*' и '.'
   - **Ключевой инсайт:** Обработка пустых совпадений

2. **[Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)** (#23) - **Hard**

   - **Подходы:** Heap/Divide & Conquer
   - **Ключевой инсайт:** Сравнение подходов

3. **[Word Ladder II](https://leetcode.com/problems/word-ladder-ii/)** (#126) - **Hard**

   - **Подходы:** BFS + Backtracking
   - **Ключевой инсайт:** Нахождение всех кратчайших путей

---

## 📊 Статистика пути

**Всего модулей:** 15  
**Всего задач (основные):** ~75  
**Всего задач (сложные):** ~45  
**Всего задач:** ~120  
**Оценочное время:** 4-6 месяцев (2-4 задачи в неделю)

---

## 🔗 Полезные ссылки

- [LeetCode Patterns](https://seanprashad.com/leetcode-patterns/) — шаблоны задач
- [NeetCode 150](https://neetcode.io/practice) — отобранные задачи
- [Big-O Cheat Sheet](https://www.bigocheatsheet.com/) — шпаргалка по сложностям
- [Rust Algorithm Club](https://rust-algo.club/) — алгоритмы на Rust
- [Visual Algo](https://visualgo.net/) — визуализация алгоритмов
- [Algorithm Visualizer](https://algorithm-visualizer.org/) — интерактивная визуализация

---

## 📈 Отслеживание прогресса

```markdown
### Неделя 1: Hash Tables

- [ ] Two Sum (1) - HashMap one-pass
- [ ] Contains Duplicate (217) - HashSet
- [ ] Valid Anagram (242) - Frequency array
- [ ] Group Anagrams (49) - Sorting key
- [ ] Intersection of Two Arrays (349) - Set operations
- [ ] Longest Consecutive Sequence (128) - Hard

### Неделя 2: Two Pointers

- [ ] Valid Palindrome (125)
- [ ] Two Sum II (167)
- [ ] 3Sum (15)
- [ ] Container With Most Water (11)
- [ ] Trapping Rain Water (42)
- [ ] 3Sum Closest (16)

### Неделя 3: Binary Search

- [ ] Binary Search (704)
- [ ] Search Insert Position (35)
- [ ] First Bad Version (278)
- [ ] Find Minimum in Rotated (153)
- [ ] Search in Rotated (33)
- [ ] Find First and Last Position (34)

### Неделя 4: Sliding Window

- [ ] Maximum Average Subarray I (643)
- [ ] Longest Substring Without Repeating (3)
- [ ] Minimum Size Subarray Sum (209)
- [ ] Permutation in String (567)
- [ ] Longest Repeating Character Replacement (424)
- [ ] Minimum Window Substring (76) - Hard

### Неделя 5: Stacks & Queues

- [ ] Valid Parentheses (20)
- [ ] Min Stack (155)
- [ ] Daily Temperatures (739)
- [ ] Implement Queue using Stacks (232)
- [ ] Evaluate RPN (150)
- [ ] Largest Rectangle in Histogram (84) - Hard

### Неделя 6: Linked Lists

- [ ] Reverse Linked List (206)
- [ ] Linked List Cycle (141)
- [ ] Merge Two Sorted Lists (21)
- [ ] Remove Nth Node From End (19)
- [ ] Copy List with Random Pointer (138)
- [ ] LRU Cache (146) - Hard

### Неделя 7: Trees

- [ ] Maximum Depth of Binary Tree (104)
- [ ] Validate BST (98)
- [ ] Binary Tree Level Order (102)
- [ ] Lowest Common Ancestor (236)
- [ ] Binary Tree Maximum Path Sum (124)
- [ ] Serialize and Deserialize (297) - Hard

### Неделя 8: Graphs

- [ ] Number of Islands (200)
- [ ] Clone Graph (133)
- [ ] Course Schedule (207)
- [ ] Rotting Oranges (994)
- [ ] Network Delay Time (743)
- [ ] Alien Dictionary (269) - Hard

### Неделя 9: Heaps

- [ ] Kth Largest Element (215)
- [ ] Top K Frequent Elements (347)
- [ ] Find Median from Data Stream (295)
- [ ] Meeting Rooms II (253)
- [ ] Task Scheduler (621)
- [ ] Merge k Sorted Lists (23) - Hard

### Неделя 10: Dynamic Programming

- [ ] Climbing Stairs (70)
- [ ] Coin Change (322)
- [ ] Longest Increasing Subsequence (300)
- [ ] Longest Common Subsequence (1143)
- [ ] House Robber (198)
- [ ] Edit Distance (72) - Hard

### Неделя 11: Backtracking

- [ ] Subsets (78)
- [ ] Permutations (46)
- [ ] Combination Sum (39)
- [ ] Generate Parentheses (22)
- [ ] Word Search (79)
- [ ] N-Queens (51) - Hard

### Неделя 12: Greedy Algorithms

- [ ] Jump Game (55)
- [ ] Jump Game II (45)
- [ ] Maximum Subarray (53)
- [ ] Task Scheduler (621)
- [ ] Queue Reconstruction by Height (406)
- [ ] Candy (135) - Hard

### Неделя 13: Intervals

- [ ] Merge Intervals (56)
- [ ] Insert Interval (57)
- [ ] Non-overlapping Intervals (435)
- [ ] Meeting Rooms (252)
- [ ] Meeting Rooms II (253)
- [ ] Minimum Interval to Include (1851) - Hard

### Неделя 14: Trie & Bit Manipulation

- [ ] Implement Trie (208)
- [ ] Add and Search Words (211)
- [ ] Word Search II (212)
- [ ] Number of 1 Bits (191)
- [ ] Reverse Bits (190)
- [ ] Maximum XOR (421) - Medium

### Неделя 15: Advanced Patterns

- [ ] Product of Array Except Self (238)
- [ ] Find the Duplicate Number (287)
- [ ] First Missing Positive (41)
- [ ] Trapping Rain Water (42)
- [ ] Largest Rectangle in Histogram (84)
- [ ] Regular Expression Matching (10) - Hard
```
