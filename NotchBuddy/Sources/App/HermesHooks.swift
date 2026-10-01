import Foundation

// MARK: - HermesHooks
// The Hermes counterpart of HookServer's Claude Code installer: read
// ~/.hermes/config.yaml, take a dated backup, merge only our own keys, show the
// diff, write after an explicit click. Uninstall removes Coucou's entries and
// nothing else. The plugin itself ships in the bundle under hermes-plugin/.
//
// Why the YAML is edited by line and never round-tripped through a parser:
// config.yaml carries comments, anchors and ordering that belong to the user
// (and to Hermes), and a parse/serialise cycle would rewrite all of it. So the
// file is read line by line, one known key is set or removed, and every other
// line goes back untouched. Anything unrecognised is an error, not a guess.

struct HermesHooksError: LocalizedError {
    let message: String
    var errorDescription: String? { message }
}

enum HermesHooks {
    static let pluginName = "coucou"

    static var home: URL { FileManager.default.homeDirectoryForCurrentUser }
    static var configURL: URL {
        home.appendingPathComponent(".hermes").appendingPathComponent("config.yaml")
    }
    static var pluginDir: URL {
        home.appendingPathComponent(".hermes")
            .appendingPathComponent("plugins").appendingPathComponent(pluginName)
    }

    struct Status: Sendable {
        let installed: Bool
        /// The relay the plugin forwards to. Without it events go nowhere —
        /// harmless, but Settings says so instead of looking broken.
        let hookReady: Bool
        let configPath: String
        let pluginPath: String
    }

    struct Preview: Sendable {
        let diff: String
        let backup: String
        let configPath: String
        let pluginPath: String
        let fingerprint: String
    }

    // MARK: - Public API

    static func status() -> Status {
        let installed = FileManager.default.fileExists(atPath: pluginDir.appendingPathComponent("__init__.py").path)
            && hasTransport((try? readConfig()) ?? "")
        return Status(
            installed: installed,
            hookReady: relayPath() != nil,
            configPath: configURL.path,
            pluginPath: pluginDir.path
        )
    }

    /// Which relay the plugin will find: the App Store copy under ~/.claude,
    /// or the GitHub build's copy in Application Support.
    private static func relayPath() -> String? {
        let candidates = [
            home.appendingPathComponent(".claude/coucou/nb-hook").path,
            HookServer.hookScriptPath,
        ]
        return candidates.first { FileManager.default.fileExists(atPath: $0) }
    }

    /// Returns the diff to show. Nothing is written here.
    static func preview(install: Bool) throws -> Preview {
        let current = try readConfig()
        let next = install ? try withKeys(current) : try withoutKeys(current)
        return Preview(
            diff: unifiedDiff(current, next),
            backup: backupURL().path,
            configPath: configURL.path,
            pluginPath: pluginDir.path,
            fingerprint: fingerprint(Data(current.utf8))
        )
    }

    /// Writes the merged (or cleaned) config and the plugin files, after a
    /// backup. `expected` is the fingerprint the user looked at.
    static func write(install: Bool, expected: String) throws -> String {
        #if APPSTORE
        throw HermesHooksError(message: "The App Store build is sandboxed and cannot reach ~/.hermes.")
        #else
        let current = try readConfig()
        guard fingerprint(Data(current.utf8)) == expected else {
            throw HermesHooksError(
                message: "\(configURL.path) changed since the preview. Nothing was written — review the new diff."
            )
        }
        let next = install ? try withKeys(current) : try withoutKeys(current)

        let hermesDir = home.appendingPathComponent(".hermes")
        try FileManager.default.createDirectory(at: hermesDir, withIntermediateDirectories: true)

        var backup = ""
        if !current.isEmpty {
            let path = backupURL()
            do {
                try FileManager.default.copyItem(at: configURL, to: path)
            } catch {
                throw HermesHooksError(message: "backup failed: \(error.localizedDescription)")
            }
            backup = path.path
        }

        // Config first: if the plugin files then fail to land, the transport is
        // already gone and Hermes falls back to its own prompt instead of
        // naming a plugin that is not there.
        if next != current {
            try FileManager.default.createDirectory(
                at: configURL.deletingLastPathComponent(), withIntermediateDirectories: true
            )
            do {
                try next.write(to: configURL, atomically: true, encoding: .utf8)
            } catch {
                throw HermesHooksError(message: "write failed: \(error.localizedDescription)")
            }
        }

        if install {
            try FileManager.default.createDirectory(at: pluginDir, withIntermediateDirectories: true)
            try bundleText("plugin.yaml").write(
                to: pluginDir.appendingPathComponent("plugin.yaml"), atomically: true, encoding: .utf8
            )
            try bundleText("__init__.py").write(
                to: pluginDir.appendingPathComponent("__init__.py"), atomically: true, encoding: .utf8
            )
        } else {
            try? FileManager.default.removeItem(at: pluginDir)
        }
        return backup
        #endif
    }

    private static func backupURL() -> URL {
        let fmt = DateFormatter()
        fmt.dateFormat = "yyyyMMdd-HHmmss"
        return configURL.deletingLastPathComponent()
            .appendingPathComponent("config.yaml.bak-\(fmt.string(from: Date()))")
    }

    private static func bundleText(_ name: String) throws -> String {
        guard let url = Bundle.main.url(forResource: name, withExtension: nil, subdirectory: "hermes-plugin") else {
            throw HermesHooksError(message: "\(name) is missing from the app bundle — nothing was written.")
        }
        guard let text = try? String(contentsOf: url, encoding: .utf8) else {
            throw HermesHooksError(message: "\(name) could not be read from the app bundle.")
        }
        return text
    }

    /// Only "the file is not there" means "start from nothing". An unreadable
    /// config must stop us, never be replaced by an empty one.
    private static func readConfig() throws -> String {
        let url = configURL
        guard FileManager.default.fileExists(atPath: url.path) else { return "" }
        guard let data = try? Data(contentsOf: url) else {
            throw HermesHooksError(message: "\(url.path): could not be read — nothing was written.")
        }
        guard let text = String(data: data, encoding: .utf8) else {
            throw HermesHooksError(message: "config.yaml is not UTF-8 — nothing was written.")
        }
        return text
    }

    /// FNV-1a: the question is only "is this still the file I showed the
    /// user?", not cryptographic strength.
    static func fingerprint(_ data: Data) -> String {
        var hash: UInt64 = 0xcbf2_9ce4_8422_2325
        for byte in data {
            hash ^= UInt64(byte)
            hash = hash &* 0x1000_0000_01b3
        }
        return String(format: "%016llx", hash)
    }

    // MARK: - The line-oriented YAML editor

    private static func indentOf(_ line: String) -> Int {
        var n = 0
        for ch in line {
            if ch == " " || ch == "\t" { n += 1 } else { break }
        }
        return n
    }

    private static func skippable(_ line: String) -> Bool {
        let t = line.trimmingCharacters(in: .whitespaces)
        return t.isEmpty || t.hasPrefix("#")
    }

    private static func keyOf(_ line: String) -> (String, String)? {
        var i = line.startIndex
        while i < line.endIndex, line[i] == " " || line[i] == "\t" { i = line.index(after: i) }
        let t = line[i...]
        if t.isEmpty || t.hasPrefix("#") || t.hasPrefix("-") { return nil }
        guard let colon = t.firstIndex(of: ":") else { return nil }
        let key = String(t[t.startIndex ..< colon]).trimmingCharacters(in: .whitespaces)
        if key.isEmpty { return nil }
        let value = String(t[line.index(after: colon)...]).trimmingCharacters(in: .whitespaces)
        return (key, value)
    }

    private static func findToplevel(_ lines: [String], _ key: String) -> Int? {
        for (i, line) in lines.enumerated() where indentOf(line) == 0 {
            if keyOf(line)?.0 == key { return i }
        }
        return nil
    }

    /// The line after the last content line of the block starting at `start`.
    /// Trailing comments belong to whatever comes next, so they are not
    /// swallowed.
    private static func blockEnd(_ lines: [String], _ start: Int) -> Int {
        let base = indentOf(lines[start])
        var last = start
        var i = start + 1
        while i < lines.count {
            let line = lines[i]
            if skippable(line) { i += 1; continue }
            if indentOf(line) <= base { break }
            last = i
            i += 1
        }
        return last + 1
    }

    /// Index of `key` anywhere inside the block at `start`, at any deeper indent.
    private static func findKey(_ lines: [String], _ start: Int, _ key: String) -> Int? {
        let base = indentOf(lines[start])
        var i = start + 1
        while i < lines.count {
            let line = lines[i]
            if skippable(line) { i += 1; continue }
            if indentOf(line) <= base { return nil }
            if keyOf(line)?.0 == key { return i }
            i += 1
        }
        return nil
    }

    /// Indexes of the content lines nested inside the block at `start`.
    private static func childLines(_ lines: [String], _ start: Int) -> [Int] {
        let base = indentOf(lines[start])
        return ((start + 1) ..< blockEnd(lines, start)).filter { i in
            !skippable(lines[i]) && indentOf(lines[i]) > base
        }
    }

    /// Index of the key at `path`, creating nothing. `nil` means "not there",
    /// which is exactly what an uninstall wants to hear.
    private static func lookup(_ lines: [String], _ path: [String]) -> Int? {
        guard let last = path.last else { return nil }
        var cur: Int?
        for key in path.dropLast() {
            let next = cur.map { findKey(lines, $0, key) } ?? findToplevel(lines, key)
            guard let n = next else { return nil }
            cur = n
        }
        if let parent = cur { return findKey(lines, parent, last) }
        return findToplevel(lines, last)
    }

    /// Walks `path` to its parent mapping, creating whatever is missing.
    /// Returns the index of the last key on the path, or nil for "top level".
    private static func ensureParents(_ lines: inout [String], _ path: [String]) throws -> Int? {
        var cur: Int?
        // Top-level keys sit at column 0, so the imaginary parent starts at -2.
        var curIndent = -2
        for key in path {
            let found = cur.map { findKey(lines, $0, key) } ?? findToplevel(lines, key)
            if let idx = found {
                if let (_, value) = keyOf(lines[idx]), !value.isEmpty {
                    throw HermesHooksError(
                        message: "`\(key)` in \(configURL.path) is not a mapping — nothing was written."
                    )
                }
                curIndent = indentOf(lines[idx])
                cur = idx
            } else {
                let indent = max(curIndent + 2, 0)
                let pos: Int
                if let parent = cur {
                    pos = blockEnd(lines, parent)
                } else {
                    // Trailing empty lines are the file's own terminator, not a
                    // place for our block: leaving them last is what lets a
                    // removal round-trip byte for byte.
                    pos = lines.lastIndex(where: { !$0.isEmpty }).map { $0 + 1 } ?? 0
                }
                lines.insert(String(repeating: " ", count: indent) + key + ":", at: pos)
                curIndent = indent
                cur = pos
            }
        }
        return cur
    }

    /// Sets one scalar key inside its parent mapping (or at the top level).
    private static func setKey(
        _ lines: inout [String], _ parent: Int?, _ key: String, _ value: String
    ) throws {
        let found = parent.map { findKey(lines, $0, key) } ?? findToplevel(lines, key)
        if let idx = found {
            if !childLines(lines, idx).isEmpty {
                throw HermesHooksError(
                    message: "`\(key)` holds a mapping of its own — nothing was written."
                )
            }
            lines[idx] = String(repeating: " ", count: indentOf(lines[idx])) + key + ": " + value
            return
        }
        let pos: Int
        let indent: Int
        if let p = parent {
            pos = blockEnd(lines, p)
            indent = indentOf(lines[p]) + 2
        } else {
            pos = lines.count
            indent = 0
        }
        lines.insert(String(repeating: " ", count: indent) + key + ": " + value, at: pos)
    }

    /// Drops a scalar key, but only when it still holds the value we wrote:
    /// somebody who changed it by hand keeps their setting.
    @discardableResult
    private static func removeKey(
        _ lines: inout [String], _ parent: Int?, _ key: String, _ value: String
    ) -> Bool {
        let found = parent.map { findKey(lines, $0, key) } ?? findToplevel(lines, key)
        if let idx = found, keyOf(lines[idx])?.1 == value {
            lines.remove(at: idx)
            return true
        }
        return false
    }

    private static func stripQuotes(_ value: String) -> String {
        let v = value.trimmingCharacters(in: .whitespaces)
        for quote in ["\"", "'"] {
            if v.count >= 2, v.hasPrefix(quote), v.hasSuffix(quote) {
                return String(v.dropFirst().dropLast())
            }
        }
        return v
    }

    private static func inlineHas(_ value: String, _ item: String) -> Bool {
        let inner = value.trimmingCharacters(in: CharacterSet(charactersIn: "[]"))
        return inner.split(separator: ",").contains { stripQuotes(String($0)) == item }
    }

    /// `- item` inside a (possibly inline) list at `path`.
    private static func setListItem(_ lines: inout [String], _ path: [String], _ item: String) throws {
        guard let last = path.last else { throw HermesHooksError(message: "empty path") }
        let parent = try ensureParents(&lines, Array(path.dropLast()))
        guard let idx = parent.map({ findKey(lines, $0, last) }) ?? findToplevel(lines, last) else {
            // No `enabled:` yet: write the whole block under the parent we have.
            let pos: Int
            let indent: Int
            if let p = parent {
                pos = blockEnd(lines, p)
                indent = indentOf(lines[p]) + 2
            } else {
                pos = lines.count
                indent = 0
            }
            // Key first: it shifts the terminator down, so the item then lands
            // between the two instead of after the end of the file.
            lines.insert(String(repeating: " ", count: indent) + last + ":", at: pos)
            lines.insert(
                String(repeating: " ", count: indent) + "  - " + item, at: pos + 1
            )
            return
        }

        guard let (key, value) = keyOf(lines[idx]) else { return }

        // Inline form: `enabled: [a, b]` — extended where it stands.
        if value.hasPrefix("[") {
            if inlineHas(value, item) { return }
            guard let close = value.lastIndex(of: "]") else {
                throw HermesHooksError(message: "`enabled` has an unterminated inline list.")
            }
            let inner = String(value[value.startIndex ..< close])
                .trimmingCharacters(in: .whitespaces)
            let joined = inner.hasSuffix("[") ? inner + item + "]" : inner + ", " + item + "]"
            lines[idx] = String(repeating: " ", count: indentOf(lines[idx])) + key + ": " + joined
            return
        }
        if !value.isEmpty {
            throw HermesHooksError(
                message: "`\(path.joined(separator: "."))` is not a list — nothing was written."
            )
        }

        // Block form: nested lines starting with `-`.
        let keyIndent = indentOf(lines[idx])
        var lastItem: Int?
        var i = idx + 1
        while i < blockEnd(lines, idx) {
            let line = lines[i]
            if skippable(line) || indentOf(line) <= keyIndent { i += 1; continue }
            if line.trimmingCharacters(in: .whitespaces).hasPrefix("-") {
                let text = line
                    .trimmingCharacters(in: .whitespaces)
                    .drop { $0 == "-" || $0 == " " }
                    .trimmingCharacters(in: .whitespaces)
                if stripQuotes(String(text)) == item { return }
                lastItem = i
            }
            i += 1
        }
        let pos = (lastItem ?? idx) + 1
        lines.insert(
            String(repeating: " ", count: keyIndent + 2) + "- " + item, at: pos
        )
    }

    /// Removes `- item` from the list at `path`, creating nothing.
    private static func removeListItem(_ lines: inout [String], _ path: [String], _ item: String) {
        guard let idx = lookup(lines, path) else { return }
        guard let (key, value) = keyOf(lines[idx]) else { return }

        if value.hasPrefix("[") {
            if !inlineHas(value, item) { return }
            let kept = value
                .trimmingCharacters(in: CharacterSet(charactersIn: "[]"))
                .split(separator: ",")
                .map { stripQuotes(String($0)) }
                .filter { $0 != item }
            lines[idx] = String(repeating: " ", count: indentOf(lines[idx]))
                + key + ": [" + kept.joined(separator: ", ") + "]"
            return
        }

        let keyIndent = indentOf(lines[idx])
        var i = idx + 1
        while i < blockEnd(lines, idx) {
            let line = lines[i]
            let isItem = !skippable(line) && indentOf(line) > keyIndent
                && line.trimmingCharacters(in: .whitespaces).hasPrefix("-")
            if isItem {
                let text = line
                    .trimmingCharacters(in: .whitespaces)
                    .drop { $0 == "-" || $0 == " " }
                    .trimmingCharacters(in: .whitespaces)
                if stripQuotes(String(text)) == item {
                    lines.remove(at: i)
                    continue
                }
            }
            i += 1
        }

        // An emptied `enabled:` (or `plugins:`) goes away with it: a bare key
        // is YAML null, and leaving our nulls behind would be our edit, not
        // theirs.
        if value.isEmpty && childLines(lines, idx).isEmpty {
            lines.remove(at: idx)
            dropEmptyMapping(&lines, Array(path.dropLast()))
        }
    }

    /// Removes `keys` from the outermost inwards while they hold no children.
    private static func dropEmptyMapping(_ lines: inout [String], _ keys: [String]) {
        var parent: Int?
        for key in keys {
            guard let idx = parent.map({ findKey(lines, $0, key) }) ?? findToplevel(lines, key) else {
                return
            }
            if childLines(lines, idx).isEmpty {
                lines.remove(at: idx)
                parent = nil
            } else {
                parent = idx
            }
        }
    }

    // MARK: - Install / uninstall

    private static func splitLines(_ text: String) -> [String] {
        if text.isEmpty { return [] }
        return text.components(separatedBy: "\n")
    }

    /// Puts the file back exactly as found: same trailing newline, or a fresh
    /// newline when we are the ones creating it.
    private static func join(_ lines: [String], _ original: String) -> String {
        var text = lines.joined(separator: "\n")
        if text.isEmpty { return "" }
        if (original.isEmpty || original.hasSuffix("\n")), !text.hasSuffix("\n") {
            text += "\n"
        }
        return text
    }

    private static func withKeys(_ text: String) throws -> String {
        var lines = splitLines(text)
        let parent = try ensureParents(&lines, ["security", "approval"])
        try setKey(&lines, parent, "transport", pluginName)
        try setKey(&lines, parent, "transport_fallback", "builtin")
        try setListItem(&lines, ["plugins", "enabled"], pluginName)
        return join(lines, text)
    }

    private static func withoutKeys(_ text: String) throws -> String {
        if text.isEmpty { return "" }
        var lines = splitLines(text)

        if let approval = lookup(lines, ["security", "approval"]) {
            // `transport_fallback: builtin` only ever mattered next to our
            // transport, so it goes the moment transport does.
            if removeKey(&lines, approval, "transport", pluginName) {
                removeKey(&lines, approval, "transport_fallback", "builtin")
                if childLines(lines, approval).isEmpty {
                    lines.remove(at: approval)
                    dropEmptyMapping(&lines, ["security"])
                }
            }
        }

        removeListItem(&lines, ["plugins", "enabled"], pluginName)
        return join(lines, text)
    }

    private static func hasTransport(_ config: String) -> Bool {
        for line in config.split(separator: "\n", omittingEmptySubsequences: false) {
            if let (k, v) = keyOf(String(line)), k == "transport", stripQuotes(v) == pluginName {
                return true
            }
        }
        return false
    }

    // MARK: - Minimal unified diff (LCS)

    static func unifiedDiff(_ before: String, _ after: String) -> String {
        let a = lineArray(before), b = lineArray(after)
        let n = a.count, m = b.count
        var lcs = [[Int]](repeating: [Int](repeating: 0, count: m + 1), count: n + 1)
        var i = n - 1
        while i >= 0 {
            var j = m - 1
            while j >= 0 {
                lcs[i][j] = a[i] == b[j]
                    ? lcs[i + 1][j + 1] + 1
                    : max(lcs[i + 1][j], lcs[i][j + 1])
                j -= 1
            }
            i -= 1
        }
        var out: [String] = []
        var x = 0, y = 0
        while x < n && y < m {
            if a[x] == b[y] {
                out.append("  " + a[x]); x += 1; y += 1
            } else if lcs[x + 1][y] >= lcs[x][y + 1] {
                out.append("- " + a[x]); x += 1
            } else {
                out.append("+ " + b[y]); y += 1
            }
        }
        while x < n { out.append("- " + a[x]); x += 1 }
        while y < m { out.append("+ " + b[y]); y += 1 }
        return out.joined(separator: "\n")
    }

    /// `lines()`, matching the Rust side: a trailing newline is a terminator,
    /// not an empty line.
    private static func lineArray(_ text: String) -> [String] {
        if text.isEmpty { return [] }
        var parts = text.components(separatedBy: "\n")
        if parts.last == "" { parts.removeLast() }
        return parts
    }
}
