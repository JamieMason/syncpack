---
name: signal-over-noise
description: Maximize useful information per word by removing filler, obvious explanations, and hedging language. Use when writing documentation, error messages, code comments, or any communication where clarity and conciseness matter.
---

# Signal Over Noise

Maximize useful information per word. Remove everything that doesn't earn its space.

## What This Is

Signal = information that moves the reader toward action or understanding.
Noise = filler, explanations of obvious things, hedging language, unnecessary context.

High signal-to-noise ratio means readers get what they need faster. Every word counts.

## When to Use

- Writing documentation, guides, skill descriptions
- Creating error messages and feedback
- Structuring arguments or explanations
- Naming things (variables, functions, branches)
- Any communication where clarity matters

## How to Apply

### 1. Remove Filler Words

**High noise:**

- "basically"
- "essentially"
- "in order to"
- "comprehensive"
- "helpful"
- "I think that"
- "it could be argued that"

**Before:** "Basically, you should essentially use this pattern in order to get comprehensive error handling."

**After:** "Use this pattern for error handling."

### 2. Cut Explanations of Obvious Things

Don't explain what readers already know. Assume competence.

**Before:** "Rust is a programming language. In Rust, you write code. This code runs on computers. When you compile Rust code, it produces a binary executable file."

**After:** "Compile Rust code to a binary with `cargo build --release`."

### 3. Front-Load the Point

Put the actionable information first. Context follows.

**Before:** "There are several reasons why you might want to consider testing your code. First, testing helps catch bugs. Second, it makes refactoring safer. Most importantly, tests serve as documentation. You should write tests."

**After:** "Write tests to catch bugs, enable safe refactoring, and document behavior."

### 4. Use Active, Direct Language

**High noise:** "It is recommended that consideration be given to..."

**Signal:** "Consider..."

**High noise:** "The file was created by the user."

**Signal:** "Create the file."

### 5. One Idea Per Sentence

Long sentences hide the point. Break them.

**Before:** "When you're writing code that handles errors, which can happen in many places throughout your application, you should think about whether you want to handle them at the point where they occur or propagate them upward, and this decision affects how you structure your code."

**After:** "Handle errors where they occur or propagate them upward. This choice shapes your code structure. Decide early."

### 6. Name Things Precisely

Names are communication. Bad names add noise; good names clarify.

**High noise:** `process_data()`, `handle_thing()`, `do_stuff()`

**Signal:** `validate_user_input()`, `retry_failed_request()`, `parse_json_config()`

### 7. Omit Context Readers Already Have

Don't repeat what's obvious from surrounding text.

**Before:** "Here's an example of how to use the feature. In this example, we show you how to use it."

**After:** "Example:"

## Trade-offs

### When NOT to apply aggressively

- **Teaching beginners:** Some explanatory redundancy helps them build intuition
- **Safety-critical systems:** Explicit redundancy catches misunderstanding
- **Legal/compliance docs:** Specificity sometimes requires verbose phrasing
- **User research notes:** Capture raw data, organize later

### The balance

Signal-over-noise isn't "be terse." It's "be useful." Sometimes that means:

- Repeating a key phrase for emphasis
- Providing context that prevents misreading
- Showing your work so readers can verify reasoning
- Acknowledging limitations explicitly

Remove noise, keep signal. Don't remove useful context.

## Examples

### Code Documentation

**Before:**

```
This function is useful for validating email addresses.
It takes a string as input, which represents the email address
that you want to check. The function will return true if the
email is valid, or false if it is not valid. You might want to
use this function when you need to validate emails.
```

**After:**

```
Validate email addresses. Returns true if valid, false otherwise.
```

### Error Messages

**Before:**

```
An error has occurred in the system. The operation you attempted
could not be completed successfully. Please try again later or
contact support if the problem persists.
```

**After:**

```
Failed to fetch user data. Retry or contact support.
```

### Skill Description

**Before:**

```
This skill helps you think about how to communicate more effectively
by using techniques that are designed to help you reduce unnecessary
information and focus on the important parts of what you're trying
to say so that your readers understand your message better and faster.
```

**After:**

```
Maximize useful information per word. Remove filler, obvious explanations, and hedging language. Every word should earn its space.
```

## Testing

How to know if you've applied this principle:

1. **Read it aloud.** Does every sentence move you forward?
2. **Ask "why?"** for each phrase. If the answer is "readers already know this," cut it.
3. **Count words.** Did you reduce the word count by 20–40%? Good sign.
4. **Test comprehension.** Can a fresh reader understand it without re-reading?
5. **Check names.** Are function/variable names predictable and specific?

## Application in Syncpack

This principle strengthens all your other skills:

- **write-code:** Function names, comments, commit messages all benefit
- **document-code:** Docs communicate faster when noise is removed
- **add-feature:** Feature descriptions and changelogs stay concise
- **fix-bug:** Error messages become actionable, not explanatory
- **search-code:** Clear naming makes code more discoverable

Apply signal-over-noise across the board. Your CLAUDE.md already mandates it—this skill formalizes how.
