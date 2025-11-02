# NPM Version Specifier Specification

Comprehensive specification of all valid npm version specifiers accepted in `package.json` dependency fields, based on deep analysis of the node-semver implementation and test suite.

## 1. Exact Versions

**Format:** `[v|=]major.minor.patch[-prerelease][+build]`

**Behavior:** Must match version exactly.

**Examples:**

- `1.2.3` - exact version
- `v1.2.3` - leading `v` stripped, equivalent to `1.2.3`
- `=1.2.3` - leading `=` stripped, equivalent to `1.2.3`
- `1.2.3-alpha.1` - exact prerelease version
- `1.2.3+build.456` - exact version with build metadata

**Notes:**

- Leading `v` or `=` optional, stripped during parsing
- Version must conform to semver 2.0.0 spec
- Max 256 characters

---

## 2. Comparison Operators

**Format:** `<operator>[whitespace]<version>`

**Operators:** `>`, `>=`, `<`, `<=`, `=`

**Behavior:** Version constraint based on operator.

**Examples:**

- `>1.2.3` - greater than 1.2.3
- `>=1.2.3` - greater than or equal to 1.2.3
- `<2.0.0` - less than 2.0.0
- `<=1.2.3` - less than or equal to 1.2.3
- `=1.2.3` - equal (same as `1.2.3`)

**Edge cases:**

- `>1` → `>=2.0.0` (NOT `>1.0.0`)
- `>1.2` → `>=1.3.0` (NOT `>1.2.0`)
- `<1` → `<1.0.0-0`
- `<1.2` → `<1.2.0-0`
- `>X` → `<0.0.0-0` (impossible range)
- `<X` → `<0.0.0-0` (impossible range)

**Whitespace handling:**

- `>  1.0.0` → `>1.0.0` (arbitrary spaces)
- `>=   1.0.0` → `>=1.0.0`
- `<\t2.0.0` → `<2.0.0` (tab normalized)

**Notes:**

- Operators can be combined with spaces (AND logic)
- Whitespace between operator and version is normalized

---

## 3. Tilde Ranges

**Format:** `~<version>` or `~><version>`

**Behavior:** Allow patch-level changes if minor specified, minor-level changes if not.

**Operators:**

- `~` - standard tilde operator
- `~>` - alternative form (identical behavior)

**Examples:**

- `~1.2.3` → `>=1.2.3 <1.3.0-0` (patch updates)
- `~>1.2.3` → `>=1.2.3 <1.3.0-0` (same as `~`)
- `~1.2` → `>=1.2.0 <1.3.0-0` (same as `1.2.x`)
- `~1` → `>=1.0.0 <2.0.0-0` (same as `1.x`)
- `~0.2.3` → `>=0.2.3 <0.3.0-0`
- `~0.2` → `>=0.2.0 <0.3.0-0`
- `~0` → `>=0.0.0 <1.0.0-0`
- `~1.2.3-beta.2` → `>=1.2.3-beta.2 <1.3.0-0`
- `~>3.2.1` → `>=3.2.1 <3.3.0-0`
- `~> 1` → `>=1.0.0 <2.0.0-0` (space allowed)

**Desugaring:**

- `~X.Y.Z` → `>=X.Y.Z <X.(Y+1).0-0`
- `~X.Y` → `>=X.Y.0 <X.(Y+1).0-0`
- `~X` → `>=X.0.0 <(X+1).0.0-0`

**Notes:**

- `~>` is legacy operator, identical to `~`
- Upper bound uses `-0` suffix to exclude prereleases

---

## 4. Caret Ranges

**Format:** `^<version>`

**Behavior:** Allow changes that don't modify left-most non-zero element.

**Examples:**

- `^1.2.3` → `>=1.2.3 <2.0.0-0` (minor & patch updates)
- `^0.2.3` → `>=0.2.3 <0.3.0-0` (patch updates only)
- `^0.0.3` → `>=0.0.3 <0.0.4-0` (no updates)
- `^0` → `<1.0.0-0` (less than 1.0.0)
- `^1.2.3-beta.2` → `>=1.2.3-beta.2 <2.0.0-0`
- `^0.0.3-beta` → `>=0.0.3-beta <0.0.4-0`
- `^1.2.x` → `>=1.2.0 <2.0.0-0`
- `^0.0.x` → `>=0.0.0 <0.1.0-0`
- `^0.0` → `>=0.0.0 <0.1.0-0`
- `^1.x` → `>=1.0.0 <2.0.0-0`
- `^0.x` → `>=0.0.0 <1.0.0-0`
- `^ 1` → `>=1.0.0 <2.0.0-0` (space allowed)

**Desugaring:**

- `^X.Y.Z` (X>0) → `>=X.Y.Z <(X+1).0.0-0`
- `^0.Y.Z` (Y>0) → `>=0.Y.Z <0.(Y+1).0-0`
- `^0.0.Z` → `>=0.0.Z <0.0.(Z+1)-0`
- `^0` → `<1.0.0-0`

**Notes:**

- Common for 0.x versions where X is breaking change indicator
- Missing patch treated as `0` with flexibility
- `^0` has special handling (no lower bound, just upper)

---

## 5. X-Ranges (Wildcards)

**Format:** `X|x|*` in place of major, minor, or patch

**Behavior:** Wildcard matching for version components.

**Examples:**

- `*` → `>=0.0.0` (any version)
- `x` → `>=0.0.0` (same as `*`)
- `1.x` → `>=1.0.0 <2.0.0-0` (any 1.x.x)
- `1.X` → `>=1.0.0 <2.0.0-0` (case insensitive)
- `1.2.x` → `>=1.2.0 <1.3.0-0` (any 1.2.x)
- `1.2.*` → `>=1.2.0 <1.3.0-0` (same as `1.2.x`)
- `2.x.x` → `>=2.0.0 <3.0.0-0`
- `2.*.*` → `>=2.0.0 <3.0.0-0`
- `""` → `*` → `>=0.0.0` (empty string = any)

**Partial versions as X-ranges:**

- `1` → `1.x.x` → `>=1.0.0 <2.0.0-0`
- `1.2` → `1.2.x` → `>=1.2.0 <1.3.0-0`
- `2` → `>=2.0.0 <3.0.0-0`
- `2.3` → `>=2.3.0 <2.4.0-0`

**Notes:**

- `X`, `x`, `*` are interchangeable
- Partial versions treated as X-ranges
- Empty string equivalent to `*`
- Case insensitive for wildcard characters

---

## 6. Hyphen Ranges

**Format:** `<version> - <version>`

**Behavior:** Inclusive set between two versions, with asymmetric partial handling.

**Examples:**

- `1.2.3 - 2.3.4` → `>=1.2.3 <=2.3.4`
- `1.2 - 2.3.4` → `>=1.2.0 <=2.3.4`
- `1.2.3 - 2.3` → `>=1.2.3 <2.4.0-0`
- `1.2.3 - 2` → `>=1.2.3 <3.0.0-0`
- `1 - 2` → `>=1.0.0 <3.0.0-0`
- `1.0 - 2.0` → `>=1.0.0 <2.1.0-0`

**With `includePrerelease` option:**

- `1.0.0 - 2.0.0` → `>=1.0.0-0 <2.0.1-0`
- `1 - 2` → `>=1.0.0-0 <3.0.0-0`
- `1.0 - 2.0` → `>=1.0.0-0 <2.1.0-0`

**Desugaring rules (asymmetric!):**

- **First version partial:** missing pieces filled with `0`
  - `1.2 - 2.3.4` → first becomes `1.2.0`
- **Second version partial:** accepts all in that tuple, excludes next
  - `1.2.3 - 2.3` → second becomes `<2.4.0-0`
  - `1.2.3 - 2` → second becomes `<3.0.0-0`

**Notes:**

- Space around hyphen required (`1.2.3 - 2.3.4` not `1.2.3-2.3.4`)
- First bound always inclusive (uses `>=`)
- Second bound inclusive for full versions (`<=`), exclusive for partial (`<`)
- Asymmetric behavior intentional for natural range expression

---

## 7. Compound Ranges

**Format:** Space-separated (AND) or `||` (OR)

**Behavior:** Logical combinations of ranges.

**AND examples (space-separated):**

- `>=1.0.2 <2.1.2` - must satisfy both
- `>1.0.2 <=2.3.4` - intersection of ranges
- `>=1.2.7 <1.3.0` - range between versions
- `~1.2.1 >=1.2.3` - both conditions required
- `^ 1.2 ^ 1` → `>=1.2.0 <2.0.0-0 >=1.0.0`

**OR examples (`||`):**

- `1.2.7 || >=1.2.9 <2.0.0` - satisfies either
- `<1.0.0 || >=2.3.1 <2.4.5 || >=2.5.2 <3.0.0` - multiple alternatives
- `0.1.20 || 1.2.4` - exact versions with OR
- `1.2.x || 2.x` - wildcards with OR

**Special cases:**

- `||` alone → `*` (any version)
- `>=*` → `*`
- `>x 2.x || * || <x` → `*`
- `<x <* || >* 2.x` → `<0.0.0-0` (impossible)

**Error cases:**

- `sadf||asdf` → TypeError (both sides invalid, even in loose mode)
- `invalid||valid` → TypeError (one side invalid)

**Notes:**

- Whitespace separates AND conditions
- `||` separates OR conditions
- Can mix: `>=1.0.0 <2.0.0 || >=3.0.0`
- Empty comparator sets throw TypeError

---

## 8. Dist Tags

**Format:** `<tag-name>`

**Behavior:** Resolves to version currently published under tag.

**Examples:**

- `latest` - latest published version (default)
- `next` - next/beta release channel
- `canary` - canary builds
- `beta`, `alpha`, `rc` - prerelease channels
- Custom tags - any string without special characters

**Notes:**

- Tags resolved at install time
- `latest` is default tag for `npm publish`
- Tags are mutable pointers to versions
- Not validated by semver parser (npm/registry concern)

---

## 9. Git URLs

**Format:** `<protocol>://[<user>[:<password>]@]<hostname>[:<port>][:][/]<path>[#<commit-ish> | #semver:<semver>]`

**Protocols:** `git`, `git+ssh`, `git+http`, `git+https`, `git+file`

**Examples:**

- `git+ssh://git@github.com:npm/cli.git#v1.0.27`
- `git+ssh://git@github.com:npm/cli#semver:^5.0`
- `git+https://isaacs@github.com/npm/cli.git`
- `git://github.com/npm/cli.git#v1.0.27`
- `git+file:///path/to/repo.git`

**Commit-ish formats:**

- `#<commit-sha>` - specific commit
- `#<branch>` - branch name
- `#<tag>` - tag name
- `#semver:<semver-range>` - npm searches tags/refs matching range

**Build behavior:**

- Cloned if `workspaces` used or build scripts present
- Build scripts: `build`, `prepare`, `prepack`, `preinstall`, `install`, `postinstall`

**Notes:**

- Default branch used if no `#` specified
- `#semver:` allows semver ranges in git repos
- Not validated by semver parser (npm concern)

---

## 10. GitHub Shorthand

**Format:** `<user>/<repo>[#<commit-ish>]`

**Behavior:** Shorthand for GitHub repos.

**Examples:**

- `expressjs/express` → latest default branch
- `mochajs/mocha#4727d357ea` → specific commit
- `npm/example-github-repo#feature\/branch` → branch (escape `/`)
- `npm/cli#semver:^5.0` → semver range in tags

**Notes:**

- Equivalent to `git+https://github.com/<user>/<repo>.git`
- Same commit-ish rules as full git URLs
- Available since npm 1.1.65
- Not validated by semver parser (npm concern)

---

## 11. HTTP/HTTPS URLs

**Format:** `http[s]://<url-to-tarball>`

**Behavior:** Downloads and installs tarball from URL.

**Examples:**

- `http://npmjs.com/example.tar.gz`
- `https://registry.example.com/package-1.2.3.tgz`
- `https://example.com/path/to/tarball.tar.gz`

**Notes:**

- HTTP auto-upgraded to HTTPS
- Downloaded and installed at install time
- Tarball must be valid npm package
- Not validated by semver parser (npm concern)

---

## 12. File Paths

**Format:** `file:<path>`

**Paths:**

- `file:../foo/bar` - relative parent
- `file:~/foo/bar` - home directory
- `file:./foo/bar` - relative current
- `file:/foo/bar` - absolute

**Examples:**

```json
{
  "dependencies": {
    "bar": "file:../foo/bar",
    "baz": "file:~/projects/baz",
    "qux": "file:./local/qux",
    "pkg": "file:/absolute/path/pkg"
  }
}
```

**Behavior:**

- Path normalized to relative path in `package.json`
- Used for local offline development
- Dependencies not installed recursively

**Notes:**

- Should NOT be used for public registry packages
- Requires manual `npm install` inside linked directory
- Available since npm 2.0.0
- Not validated by semver parser (npm concern)

---

## 13. Aliases

**Format:** `npm:[<@scope>/]<package>[@<version>]`

**Behavior:** Install package under different name.

**Examples:**

- `npm:pkg@1.0.0` - specific version alias
- `npm:@scope/pkg@1.0.0` - scoped package alias
- `npm:package@^2.0.0` - alias with range
- `npm:package@latest` - alias with tag

**Full example:**

```json
{
  "dependencies": {
    "react-old": "npm:react@16.0.0",
    "react": "npm:react@18.0.0"
  }
}
```

**Notes:**

- Allows multiple versions of same package
- Version/range/tag required after `@`
- Works with scoped packages
- Not validated by semver parser (npm concern)

---

## 14. Workspace Protocol

**Format:** `workspace:<version-range>`

**Behavior:** Link to workspace package in monorepo.

**Examples:**

- `workspace:*` - any workspace version
- `workspace:^` - workspace caret range
- `workspace:~` - workspace tilde range
- `workspace:^1.2.3` - specific semver range in workspace
- `workspace:1.2.3` - exact workspace version

**Behavior:**

- `workspace:*` - link to workspace, any version
- `workspace:^` / `workspace:~` - use workspace with that range type
- `workspace:<version>` - specific version constraint within workspace

**Notes:**

- Specific to yarn/pnpm workspaces
- May not be fully supported by npm
- Resolved to actual version when publishing
- Not validated by semver parser (package manager concern)

---

## 15. Prerelease Versions

**Format:** `<major>.<minor>.<patch>-<prerelease>[+<build>]`

**Prerelease format:** `<identifier>[.<identifier>]...`

**Identifier:** Alphanumeric + hyphen `[0-9A-Za-z-]+`

**Examples:**

- `1.2.3-alpha` - alpha prerelease
- `1.2.3-alpha.1` - numbered alpha
- `1.2.3-0.3.7` - numeric identifiers
- `1.2.3-x.7.z.92` - mixed identifiers
- `1.2.3-beta.2+build.123` - with build metadata
- `1.0.0-rc.1+20130313144700` - release candidate with timestamp
- `1.2.3-alpha-.-beta` - hyphens within identifiers

**Build metadata format:** Same as prerelease, after `+`

**Range behavior (critical!):**

Default behavior (excludes prereleases):

- `^1.2.3` does NOT match `1.2.3-beta` ✗
- `^1.2.3` matches `1.2.4` ✓
- `~1.2.3` does NOT match `1.2.3-pre` ✗
- `1.x` does NOT match `1.0.0-alpha` ✗

Prerelease versions only match ranges with same `[major, minor, patch]` tuple:

- `^1.2.3-beta.4` matches `1.2.3-beta.5` ✓ (same 1.2.3)
- `^1.2.3-beta.4` does NOT match `1.2.4-beta.1` ✗ (different patch)
- `^0.0.1-beta` matches `0.0.1-beta.4` ✓
- `>1.2.3-alpha.3` matches `1.2.3-alpha.7` ✓
- `>1.2.3-alpha.3` does NOT match `3.4.5-alpha.9` ✗ (different tuple)

With `includePrerelease: true` option:

- `^1.0.0` matches `1.0.1-rc1` ✓
- `1.x` matches `1.0.0-alpha` ✓
- `*` matches `1.0.0-rc1` ✓

**Comparison algorithm (critical!):**

1. **Release vs prerelease:**
   - `1.0.0-alpha < 1.0.0` (prerelease always less than release)
   - `0.0.0 > 0.0.0-foo`

2. **Numeric identifiers** (compared as integers):
   - `1.2.3-5 > 1.2.3-4` ✓
   - `1.2.3-1 < 1.2.3-2 < 1.2.3-10` ✓
   - `1.2.3-a.10 > 1.2.3-a.5` ✓ (numeric at position 2)

3. **String identifiers** (compared lexically, case-sensitive):
   - `1.2.3-r2 > 1.2.3-r100` ✓ (lexical: "r2" > "r100")
   - `1.2.3-r100 > 1.2.3-R2` ✓ (lowercase > uppercase)
   - `1.2.3-5-Foo < 1.2.3-5-foo` ✓ (case-sensitive)
   - `1.2.3-a.b > 1.2.3-a` ✓ (longer > shorter)

4. **Mixed numeric/alphanumeric:**
   - ALL purely numeric identifiers < ALL alphanumeric identifiers
   - `1.2.3-1 < 1.2.3-1a` ✓
   - `1.2.3-4 < 1.2.3-4-foo` ✓
   - `1.2.3-a.10 > 1.2.3-a.b` ✗ (both positions compared)

5. **Multi-part comparison:**
   - Compare position by position
   - `1.2.3-a.b.c.10.d.5 > 1.2.3-a.b.c.5.d.100` ✓ (10 > 5 at position 4)
   - `1.2.3-alpha.0.beta < 1.2.3-alpha.1.beta` ✓

6. **Build metadata ignored:**
   - `1.2.3+build1 == 1.2.3+build2` ✓ (equal for comparison)
   - `1.2.3+a < 1.2.3+z` ✗ (false, they're equal)
   - Build preserved in raw value but not used for precedence

**Prerelease boundary syntax (`-0` suffix):**

- Upper bounds use `-0` to exclude all prereleases of that version
- `<2.0.0-0` excludes `2.0.0-alpha`, `2.0.0-beta`, etc.
- `>=1.0.0 <2.0.0-0` allows `1.x.x` but not `2.0.0-anything`
- This is how ranges exclude prereleases by default

**Notes:**

- Build metadata (`+`) doesn't affect version precedence
- Numeric identifiers compared as integers, not strings
- String comparison is case-sensitive
- Longer prerelease arrays > shorter (after matching prefix)

---

## 16. Special Values

### Empty String

**Format:** `""`

**Behavior:** Same as `*` - matches any version (excluding prereleases by default)

### Asterisk

**Format:** `*`

**Behavior:** Matches any non-prerelease version (unless `includePrerelease` set)

**Note:** `*` and `>=0.0.0` differ in prerelease handling:

- `*` → excludes prereleases by default
- `>=0.0.0` → may include prereleases in certain contexts
- With `includePrerelease: true`, they're equivalent

---

## 17. Constants and Limits

**MAX_LENGTH:** 256 characters

- Maximum total length of a version string
- Versions exceeding this are invalid

**MAX_SAFE_INTEGER:** 9007199254740991 (2^53 - 1)

- Maximum value for major, minor, patch components
- Values exceeding this are invalid
- Example: `${MAX_SAFE_INTEGER}0.0.0` → invalid

**MAX_SAFE_COMPONENT_LENGTH:** 16 characters

- Maximum length for individual numeric components during coercion
- Components > 16 chars ignored in coercion
- Example: `${'1'.repeat(17)}.2.3` coerces to `2.3.0`

**MAX_SAFE_BUILD_LENGTH:** 250 characters

- Maximum length for build metadata
- Calculated as MAX_LENGTH - 6 (for "0.0.0+")

**SEMVER_SPEC_VERSION:** 2.0.0

- Semver specification version implemented

**RELEASE_TYPES:**

- `major`, `premajor`, `minor`, `preminor`, `patch`, `prepatch`, `prerelease`

---

## 18. Loose Mode Detailed Behavior

**Enable with:** `{ loose: true }` option

**Allowed in loose mode:**

1. **Leading `v` or `=`:**
   - `v1.2.3` → `1.2.3`
   - `=1.2.3` → `1.2.3`
   - `[v=\s]*` pattern allowed

2. **Leading zeros:**
   - `>=01.02.03` → `>=1.2.3`
   - `~01.02.03` → `~1.2.3`

3. **Prerelease without hyphen:**
   - `~1.2.3beta` → `>=1.2.3-beta <1.3.0-0`
   - `1.2.3tag` (major increment) → `2.0.0`
   - `1.2.3alpha` → `1.2.3-alpha`

4. **Extra whitespace:**
   - More lenient whitespace handling

**Still throws in loose mode:**

- Completely invalid syntax: `sadf||asdf` → TypeError
- Wrong types: `{version: '1.2.3'}` → TypeError
- Too long: `${'1'.repeat(257)}` → invalid
- Too big: `${MAX_SAFE_INTEGER}0.0.0` → invalid

**Examples:**

```javascript
// Strict mode - throws
new Range("~1.2.3beta"); // TypeError

// Loose mode - parses
new Range("~1.2.3beta", { loose: true }); // >=1.2.3-beta <1.3.0-0
```

**Notes:**

- Output always strict semver, even from loose input
- Loose mode is parser-level, not output-level
- Still validates against MAX_LENGTH and MAX_SAFE_INTEGER

---

## 19. Coercion Detailed Behavior

**Function:** `coerce(input, options)`

**Behavior:** Attempts to extract valid semver from any string

**Component length limit:**

- Each numeric component max 16 characters
- Longer components ignored/skipped

**Extraction algorithm:**

1. Find first digit sequence (1-16 chars)
2. Optionally match `.` + digit sequence (1-16 chars)
3. Optionally match `.` + digit sequence (1-16 chars)
4. Stop at first non-digit (or max length)

**With `includePrerelease` option:**

- Also preserves prerelease and build parts
- `1.2.3.4-rc.1+build.2` → `2.3.4-rc.1+build.2` (LTR)

**LTR mode (default):**

```javascript
coerce("1.2.3.4"); // 1.2.3
coerce("v1.2.3"); // 1.2.3
coerce("version 1.2.3"); // 1.2.3
coerce("1 1 1"); // 1.0.0 (first digit)
coerce("42.6.7.9.3-alpha"); // 42.6.7
```

**RTL mode (`{ rtl: true }`):**

```javascript
coerce("1.2.3.4", { rtl: true }); // 2.3.4
coerce("4.6.3.9.2-alpha2", { rtl: true }); // 4.6.3 (still finds valid triple)
```

**Component length truncation:**

```javascript
coerce('${'1'.repeat(17)}.2.3')  // 2.3.0 (first component too long)
coerce('1.${'2'.repeat(17)}.3')  // 1.0.0 (second component too long)
coerce('1.2.${'3'.repeat(17)}')  // 1.2.0 (third component too long)
coerce('${'1'.repeat(16)}.2.3')  // ${16 ones}.2.3 (exactly 16 ok)
```

**Returns null for:**

```javascript
coerce(""); // null (too short)
coerce("version one"); // null (no digits)
coerce("9".repeat(16)); // null (too big, exceeds MAX_SAFE_INTEGER)
coerce("1".repeat(17)); // null (component too long)
coerce(null); // null
coerce({ version: "1.2.3" }); // null (not a string)
```

**Valid coercions:**

```javascript
coerce(".1"); // 1.0.0
coerce("1."); // 1.0.0
coerce("1.0"); // 1.0.0
coerce("a1"); // 1.0.0
coerce("1a"); // 1.0.0
coerce("version1.2"); // 1.2.0
```

---

## 20. Build Metadata in Ranges

**Critical:** Build metadata is ALWAYS stripped from range specifiers during parsing

**Examples:**

```javascript
// X-ranges with build metadata
'1.x.x+build'                    → '>=1.0.0 <2.0.0-0'
'1.x+build.123'                  → '>=1.0.0 <2.0.0-0'
'1.x.x+meta-data'                → '>=1.0.0 <2.0.0-0'

// Operators with build metadata
'>=1.x+build <2.x.x+build'       → '>=1.0.0 <2.0.0-0'
'>1.x+build <=2.x.x+meta'        → '>=2.0.0 <3.0.0-0'

// Tilde with build metadata
'~1.x+build'                     → '>=1.0.0 <2.0.0-0'
'~1.x.x+build'                   → '>=1.0.0 <2.0.0-0'
'~1.2.x+build'                   → '>=1.2.0 <1.3.0-0'

// Caret with build metadata
'^1.x+build'                     → '>=1.0.0 <2.0.0-0'
'^1.x.x+build'                   → '>=1.0.0 <2.0.0-0'
'^1.2.3+build'                   → '>=1.2.3 <2.0.0-0'

// OR ranges with build metadata
'1.x.x+build || 2.x.x+build'     → '>=1.0.0 <2.0.0-0||>=2.0.0 <3.0.0-0'

// AND ranges with build metadata
'1.x.x+build >2.x+meta'          → '>=1.0.0 <2.0.0-0 >=3.0.0'

// With prerelease and build
'1.x.x-alpha+build'              → '>=1.0.0 <2.0.0-0'
'>1.x.x-alpha+build'             → '>=2.0.0'
```

**Note:** Build metadata preserved in version objects but ignored in range parsing

---

## 21. Invalid Version Handling

**Critical:** Invalid versions in `satisfies()` return `false`, never throw

**Examples:**

```javascript
satisfies("not-a-version", "*"); // false
satisfies("glorp", ">=2"); // false
satisfies(false, "2.x"); // false
satisfies(null, "*"); // false
satisfies(undefined, ">=1.0.0"); // false
satisfies({ version: "1.2.3" }, "*"); // false (object, not string)

// Version parsing attempts type coercion first
satisfies("1.2.3", ">=1.0.0"); // true (valid)
satisfies("v1.2.3-foo", "*", { loose: true }); // false (invalid even in loose)
```

**When constructing Range/SemVer directly:**

```javascript
new SemVer("not-a-version"); // TypeError: Invalid Version
new Range("invalid range"); // TypeError: Invalid SemVer Range
```

**Notes:**

- `satisfies()` is more forgiving than constructors
- Type coercion attempted but failure returns `false`
- Constructors throw TypeError on invalid input

---

## 22. Edge Cases and Surprising Behaviors

### 1. Prerelease Boundary `-0` Suffix

The `-0` suffix in upper bounds is critical for prerelease exclusion:

```javascript
"<2.0.0-0"; // Excludes 2.0.0-alpha, 2.0.0-beta, etc.
">=1.0.0 <2.0.0-0"; // All 1.x.x but NOT 2.0.0-anything

// Why needed:
"1.2.3" < "1.2.4-0" < "1.2.4-alpha" < "1.2.4"; // Ordering
```

This is how ranges exclude prereleases by default.

### 2. Whitespace and ReDoS Protection

```javascript
// Arbitrary whitespace normalized
'>  1.0.0'       → '>1.0.0'
'>=   1.0.0'     → '>=1.0.0'
'<\t2.0.0'       → '<2.0.0'

// ReDoS protection tested
' '.repeat(500000) + '1.2.3'  // Still parses without hanging
```

### 3. Range Intersection: `*` vs `>=0.0.0`

```javascript
// Without includePrerelease:
"*"; // Excludes prereleases
">=0.0.0"; // May include prereleases in some contexts

// With includePrerelease:
"*"; // Includes prereleases
">=0.0.0"; // Includes prereleases
// Now equivalent

// Test:
satisfies("1.0.0-rc", "*"); // false
satisfies("1.0.0-rc", "*", { includePrerelease: true }); // true
```

### 4. Hyphen Range Asymmetry

```javascript
// First partial: fills with 0
'1.2 - 2.3.4'   → '>=1.2.0 <=2.3.4'

// Second partial: excludes next
'1.2.3 - 2.3'   → '>=1.2.3 <2.4.0-0'
'1.2.3 - 2'     → '>=1.2.3 <3.0.0-0'

// Why: Natural language "from 1.2 to 2.3" means "up to but not including 2.4"
```

### 5. Empty Comparator Behavior

```javascript
'||'                   → '*'       // Empty OR resolves to any
'>=*'                  → '*'
'>x 2.x || * || <x'    → '*'

// But malformed throws:
'sadf||asdf'           // TypeError (both sides invalid)
```

### 6. Comparison Operator Increments

```javascript
'>1'       → '>=2.0.0'    // NOT '>1.0.0'
'>1.2'     → '>=1.3.0'    // NOT '>1.2.0'
'>1.2.3'   → '>1.2.3'     // Full version, no increment

// Impossible ranges:
'>X'       → '<0.0.0-0'
'<X'       → '<0.0.0-0'
```

### 7. Cache Behavior

Ranges and versions are cached globally:

```javascript
const r1 = new Range("1.0.0");
const r2 = new Range("1.0.0");
// r1 and r2 may share cached comparator objects
```

### 8. Leading Zeros

```javascript
// Strict mode - throws
'>=09090'              // TypeError

// Loose mode - strips
'>=09090', { loose: true }  → '>=9090.0.0'

// But prerelease leading zeros still fail
'>=09090-0', { loose: true }  → TypeError
```

---

## Grammar (BNF)

```bnf
range-set  ::= range ( logical-or range ) *
logical-or ::= ( ' ' ) * '||' ( ' ' ) *
range      ::= hyphen | simple ( ' ' simple ) * | ''
hyphen     ::= partial ' - ' partial
simple     ::= primitive | partial | tilde | caret
primitive  ::= ( '<' | '>' | '>=' | '<=' | '=' ) ws* partial
partial    ::= xr ( '.' xr ( '.' xr qualifier ? )? )?
xr         ::= 'x' | 'X' | '*' | nr
nr         ::= '0' | ['1'-'9'] ( ['0'-'9'] ) *
tilde      ::= ( '~' | '~>' ) ws* partial
caret      ::= '^' ws* partial
qualifier  ::= ( '-' pre )? ( '+' build )?
pre        ::= parts
build      ::= parts
parts      ::= part ( '.' part ) *
part       ::= nr | [-0-9A-Za-z]+
ws         ::= ' ' | '\t'
```

**Notes:**

- `ws*` indicates optional whitespace (spaces or tabs)
- Build metadata ignored in range parsing
- `~>` alternative form for tilde operator

---

## Validation Rules

### Version String Validation

1. **Max length:** 256 characters total
2. **Component value:** Each numeric component ≤ MAX_SAFE_INTEGER (9007199254740991)
3. **Component format:** Major.minor.patch required, each non-negative integer
4. **Prerelease format:** Alphanumeric + hyphen identifiers separated by `.`
5. **Build format:** Same as prerelease, after `+`
6. **Leading zeros:** Not allowed in numeric identifiers (except `0` itself)

### Range String Validation

1. **Max length:** 256 characters total
2. **Comparator sets:** Must have at least one valid comparator per OR branch
3. **Operators:** Must be one of `<`, `<=`, `>`, `>=`, `=`, or implicit
4. **Wildcards:** `X`, `x`, `*` case-insensitive
5. **Hyphen ranges:** Require space around `-`

### Character Restrictions

- **Version numbers:** Digits, `.`, `-` (prerelease separator), `+` (build separator)
- **Prerelease/build identifiers:** `[0-9A-Za-z-]+`
- **Operators:** `<`, `>`, `=`
- **Wildcards:** `X`, `x`, `*`
- **Whitespace:** Space, tab (normalized)

### Numeric Limits

- **MAX_LENGTH:** 256
- **MAX_SAFE_INTEGER:** 9007199254740991
- **MAX_SAFE_COMPONENT_LENGTH:** 16 (for coercion)
- **MAX_SAFE_BUILD_LENGTH:** 250

---

## Resolution Priority

When multiple specifiers could apply:

1. Exact versions
2. Prerelease versions (if opted in via `includePrerelease`)
3. Range constraints (narrowest to widest)
4. Dist tags (resolved at install time)
5. Git refs (resolved at install time)
6. URLs (resolved at install time)
7. File paths (resolved at install time)

---

## Implementation Notes

- **Semver spec version:** 2.0.0
- **Loose mode:** More forgiving parsing, but output always strict
- **Include prerelease:** Option to include prerelease in range matching (default: false)
- **Coercion:** Attempts to extract semver from any string
- **Leading zeros:** Not allowed in numeric identifiers (except `0` itself)
- **Case sensitivity:** Versions case-sensitive; wildcards (`X`/`x`) case-insensitive
- **Build metadata:** Preserved in objects but ignored in comparisons
- **Caching:** Ranges and versions cached for performance
- **ReDoS protection:** Regexes designed to prevent catastrophic backtracking
