---
name: front-loading
description: Put the answer first, then context and details follow. Use when writing documentation, tutorials, error messages, or any communication where readers need to decide quickly if something is relevant to them.
---

# Front-Loading

Put the answer first. Context follows.

## What This Is

Front-loading means leading with the most important information. Users decide in the first few words whether to keep reading.

Structure your communication as:

1. **The point** (first sentence)
2. **Why it matters** (next 1–2 sentences)
3. **Details and evidence** (rest of content)

Most writers do the opposite: context first, then conclusion. Readers get lost in the journey before reaching the destination.

## When to Use

- Writing documentation or guides
- Structuring arguments or explanations
- Creating tutorial sections
- Writing error messages
- Composing email or feedback
- Organizing any sequential information

Front-load at every level: sentences, paragraphs, sections, documents.

## How to Apply

### 1. Lead with the Conclusion

Put the answer first. Don't make readers wait.

**Bad (context first):**
"There are many different ways to approach testing. Some teams test before writing code. Others test after. Some tests run quickly, others take longer. The most important thing is consistency. So you should write tests."

**Good (conclusion first):**
"Write tests. Consistency matters more than approach."

Then explain the trade-offs if needed.

### 2. Answer the Reader's Question Immediately

Readers have a question in mind. Answer it in the first sentence.

**Bad (makes reader wait):**
"Syncpack is a CLI tool built in Rust. It uses a configuration file in JSON format. The configuration file specifies rules for how to synchronize dependencies. These rules can be customized per workspace..."

Reader's question: "How do I configure Syncpack?" Buried in paragraph 3.

**Good (answers immediately):**
"Configure Syncpack in `.syncpackrc.json` at your monorepo root. Specify version groups and rules for each."

Then explain each option.

### 3. Use Signposting

Tell readers what's coming before they read it.

**Bad (no preview):**
"First, install Node 18. Then, verify the installation. Next, clone the repository. After that, install dependencies..."

Reader: "Where is this going?"

**Good (preview structure):**
"Three steps: Install Node, clone the repo, and run `npm install`."

Then detail each step.

### 4. When Explaining a Process, Show the End State First

Let readers know what success looks like before they start.

**Bad (journey without destination):**
"Follow these 12 steps to set up CI/CD. Step 1: Create a workflow file. Step 2: Add a trigger. Step 3: Define a job..."

Reader: "Why am I doing this?"

**Good (end state first):**
"Goal: Your PR automatically runs tests and fails if any break. Here's how to set it up in 12 steps."

Then detail steps 1–12.

### 5. Structure Paragraphs with Topic Sentence First

Each paragraph's first sentence should be its main idea.

**Bad (idea buried):**
"There are many different caching strategies. Some store values in memory. Others use disk. Still others use remote services. The strategy you choose depends on your use case. For most applications, in-memory caching is sufficient."

Reader has to parse the whole paragraph to find the point.

**Good (topic first):**
"For most applications, in-memory caching is sufficient. Other strategies (disk, remote) exist for specific cases. Choose based on your performance and persistence needs."

### 6. When Providing Options, Lead with the Recommendation

Don't make readers compare three options to find the best one.

**Bad (no guidance):**
"You can use approach A, B, or C. Each has trade-offs. A is simple but inflexible. B is flexible but complex. C is somewhere in between. Most projects use B or C."

Reader: "So which should I use?"

**Good (recommendation first):**
"Use approach B (flexible but complex). It scales as your needs grow. Use approach A only if simplicity is critical. Approach C rarely adds value."

## Examples

### Documentation Section

**Bad (context first):**
"The registry client handles communication with npm. Over time, performance became an issue. We realized the client was making unnecessary requests. After analysis, we found that caching solved the problem. So the registry client now caches responses."

**Good (point first):**
"The registry client caches npm responses to improve performance. This reduced request overhead by 60%."

Then explain the details.

---

### Error Message

**Bad (generic context):**
"An operation has encountered an error condition. The system attempted to perform a task. This task requires certain prerequisites. One or more prerequisites were not met. Please verify your setup."

**Good (specific answer):**
"Missing required field in config: 'versionGroups'. See `.syncpackrc.json` example."

---

### Tutorial Section

**Bad (setup before goal):**
"Before you can run Syncpack, you need to set up your monorepo structure. This involves creating a root directory. Inside this directory, you need workspace packages. Each package should have a `package.json`. Once you have this structure, you can run Syncpack."

**Good (goal first):**
"Goal: Run Syncpack to sync versions across your monorepo."

Then list the prerequisites (monorepo structure, workspace packages).

---

### Workflow Documentation

**Bad (steps without outcome):**
"Step 1: Write your implementation. Step 2: Write tests. Step 3: Run tests locally. Step 4: Push to GitHub. Step 5: Wait for CI. Step 6: Address failures. Step 7: Merge when green."

**Good (outcome first):**
"Goal: Merge code only when tests pass and CI approves."

Then list the steps to reach that goal.

## Trade-offs

### When NOT to front-load

- **Narrative/storytelling:** Sometimes the journey is the point. Build suspense if appropriate.
- **Teaching from first principles:** Some explanation is necessary before jumping to conclusions.
- **Building trust:** Sometimes context _is_ the point (e.g., "here's why we made this decision").
- **Sensitive feedback:** Sometimes leading with criticism is harmful. Lead with understanding instead.

The principle: **Match the reader's mental model.** If they need context to understand the conclusion, provide context first.

### Common mistake

Over-front-loading: "Use this. End of story." without any reasoning alienates readers who need understanding.

Solve by asking: "Will the reader understand why this conclusion is right?" If no, provide enough context. But lead with the conclusion, not the context.

## Testing

How to know if you've front-loaded effectively:

1. **First sentence test:** Can someone read only the first sentence and understand the main point?
2. **Skim test:** Someone skims headings and bullets—do they get the gist?
3. **Reader expectation test:** "What happens next?" readers predict after each section. Were they right?
4. **Reduction test:** Remove everything after the first paragraph. Is the main point clear? (If yes, you front-loaded successfully.)
5. **Fresh eyes test:** Have someone read the first paragraph and summarize. Did they get it right?

## Application in Syncpack

Front-loading improves:

- **Skill descriptions:** Users know instantly if it's relevant
- **Error messages:** Users understand what failed and how to fix it
- **Documentation sections:** Readers find what they need faster
- **Code comments:** Maintainers understand the "why" before the "how"
- **Commit messages:** Reviewers know the intent before reading code
- **Proposals or plans:** Stakeholders see the recommendation before the analysis

Every document you write is a race against the reader's attention span. Front-load to win that race.
