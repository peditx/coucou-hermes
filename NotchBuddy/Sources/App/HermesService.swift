import Foundation

// MARK: - Hermes Agent (remote)

/// One turn of the conversation shown in the Hermes window.
struct HermesTurn: Identifiable {
    enum Role { case user, assistant }
    let id = UUID()
    let role: Role
    var text: String
}

enum HermesError: LocalizedError {
    case notConfigured
    case badURL
    case unreachable(String)
    case server(String)
    case silent

    var errorDescription: String? {
        switch self {
        case .notConfigured: return "No Hermes URL or key yet — open Settings first."
        case .badURL: return "Hermes URL must start with http:// or https://."
        case .unreachable(let why): return "Hermes unreachable: \(why)"
        case .server(let why): return why
        case .silent: return "Hermes sent nothing back."
        }
    }
}

/// The HTTP side of talking to a remote Hermes: endpoint, auth and stream
/// parsing. Both the full-screen window (drawing deltas as they land) and the
/// island chat (waiting for the whole reply) go through here, so the URL rules
/// and the error wording exist exactly once.
@MainActor
enum HermesAPI {
    static var configured: Bool {
        KeychainStore.shared.get("hermes-url") != nil
            && KeychainStore.shared.get("hermes-token") != nil
    }

    /// Base URL → chat-completions URL. Accepts the path already being pasted in.
    private static func endpoint() throws -> (URL, String) {
        guard let raw = KeychainStore.shared.get("hermes-url"),
              let token = KeychainStore.shared.get("hermes-token") else {
            throw HermesError.notConfigured
        }
        let base = raw.trimmingCharacters(in: .whitespaces)
            .trimmingCharacters(in: CharacterSet(charactersIn: "/"))
        // Set by the user, but still untrusted input: no file://, no garbage.
        guard base.hasPrefix("http://") || base.hasPrefix("https://") else {
            throw HermesError.badURL
        }
        let path = base.hasSuffix("/v1/chat/completions") ? base : base + "/v1/chat/completions"
        guard let url = URL(string: path) else { throw HermesError.badURL }
        return (url, token)
    }

    /// One turn. `messages` is complete — system prompt, history and prompt —
    /// so the caller decides what the agent sees. Always streamed: the window
    /// draws the deltas, the island just takes the finished text, and there is
    /// then only one parser to keep honest.
    static func send(
        messages: [[String: Any]],
        onDelta: (String) -> Void
    ) async throws -> String {
        let (url, token) = try endpoint()
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.timeoutInterval = 120          // idle between chunks, not the whole turn
        request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        let body: [String: Any] = [
            "model": "hermes-agent",
            "stream": true,
            "messages": messages,
        ]
        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (bytes, response) = try await URLSession.shared.bytes(for: request)
        if let http = response as? HTTPURLResponse, !(200..<300).contains(http.statusCode) {
            var text = ""
            for try await line in bytes.lines { text += line + "\n" }
            throw HermesError.server(serverMessage(text, status: http.statusCode))
        }

        var full = ""
        var received = 0
        for try await line in bytes.lines {
            guard line.hasPrefix("data:") else { continue }
            let payload = line.dropFirst(5).trimmingCharacters(in: .whitespaces)
            if payload == "[DONE]" { break }
            if let error = streamError(payload) { throw HermesError.server(error) }
            if let piece = delta(payload) {
                received += 1
                full += piece
                onDelta(piece)
            }
        }
        if received == 0 { throw HermesError.silent }
        return full
    }

    /// `choices[0].delta.content` from one already-unwrapped payload.
    private static func delta(_ payload: String) -> String? {
        guard let object = try? JSONSerialization.jsonObject(with: Data(payload.utf8)) as? [String: Any],
              let choices = object["choices"] as? [[String: Any]],
              let delta = choices.first?["delta"] as? [String: Any],
              let content = delta["content"] as? String,
              !content.isEmpty
        else { return nil }
        return content
    }

    /// Some servers report failures mid-stream instead of with a status code.
    private static func streamError(_ payload: String) -> String? {
        guard let object = try? JSONSerialization.jsonObject(with: Data(payload.utf8)) as? [String: Any],
              let error = object["error"] as? [String: Any],
              let message = error["message"] as? String
        else { return nil }
        return message
    }

    /// The server's own message is what makes a bad key or URL obvious.
    private static func serverMessage(_ body: String, status: Int) -> String {
        if let object = try? JSONSerialization.jsonObject(with: Data(body.utf8)) as? [String: Any] {
            let nested = object["error"] as? [String: Any] ?? object
            if let message = nested["message"] as? String, !message.isEmpty {
                return "Hermes API \(status): \(message)"
            }
        }
        let trimmed = body.trimmingCharacters(in: .whitespacesAndNewlines)
        if trimmed.isEmpty { return "Hermes API \(status)" }
        return "Hermes API \(status): \(String(trimmed.prefix(200)))"
    }
}

/// Chat against a remote Hermes agent over its OpenAI-compatible API server.
/// The key is read from the Keychain here, so it never reaches the view.
@MainActor
final class HermesChatModel: ObservableObject {
    @Published private(set) var turns: [HermesTurn] = []
    @Published private(set) var busy = false
    @Published var notice: String?

    static var configured: Bool { HermesAPI.configured }

    func reset() {
        guard !busy else { return }
        turns = []
        notice = nil
    }

    func send(_ prompt: String) async {
        let text = prompt.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !text.isEmpty, !busy else { return }
        guard Self.configured else {
            notice = HermesError.notConfigured.errorDescription
            return
        }

        notice = nil
        busy = true
        defer { busy = false }

        // Messages stop short of the new turn: the prompt goes on the end here.
        var messages: [[String: Any]] = turns.map { turn in
            ["role": turn.role == .user ? "user" : "assistant", "content": turn.text]
        }
        messages.append(["role": "user", "content": text])

        let userIndex = turns.count
        turns.append(HermesTurn(role: .user, text: text))
        turns.append(HermesTurn(role: .assistant, text: ""))

        do {
            // Non-escaping, so the transcript is safe to write into directly:
            // the reply has to land as it arrives, not when the turn ends.
            _ = try await HermesAPI.send(messages: messages) { delta in
                guard let i = self.turns.indices.last else { return }
                self.turns[i].text += delta
            }
        } catch {
            // Nothing arrived: undo both turns, so the transcript stays exactly
            // what the server saw. A partial reply is kept.
            if turns.indices.contains(userIndex + 1), turns[userIndex + 1].text.isEmpty {
                turns.removeSubrange(userIndex...)
            }
            notice = (error as? HermesError)?.errorDescription ?? error.localizedDescription
        }
    }
}
