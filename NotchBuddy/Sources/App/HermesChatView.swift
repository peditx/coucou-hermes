import SwiftUI

/// The full-screen Hermes chat — the counterpart of the Windows/Linux window,
/// laid out the way Claude Desktop lays its own out: header, transcript, composer.
struct HermesChatView: View {
    @StateObject private var model = HermesChatModel()
    @State private var draft = ""
    @FocusState private var composerFocused: Bool

    private var configured: Bool { HermesChatModel.configured }

    var body: some View {
        VStack(spacing: 0) {
            header
            Divider()
            transcript
            Divider()
            composer
        }
        .frame(minWidth: 720, minHeight: 520)
        .background(Color(nsColor: .windowBackgroundColor))
        .task { composerFocused = true }
    }

    // MARK: - Header

    private var header: some View {
        HStack(spacing: 12) {
            Text("Hermes")
                .font(.system(size: 15, weight: .semibold))

            HStack(spacing: 6) {
                Circle()
                    .fill(configured ? Color.green : Color.red)
                    .frame(width: 7, height: 7)
                Text(configured ? "ready" : "not configured")
                    .font(.system(size: 12))
                    .foregroundColor(.secondary)
            }

            Spacer()

            Button("New chat") { model.reset() }
                .disabled(model.busy)
            Button("Settings") {
                NotificationCenter.default.post(name: .openFullSettings, object: nil)
            }
        }
        .padding(.horizontal, 18)
        .padding(.vertical, 12)
        .background(Color(nsColor: .controlBackgroundColor))
    }

    // MARK: - Transcript

    private var transcript: some View {
        ScrollViewReader { proxy in
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    if model.turns.isEmpty {
                        emptyState
                    } else {
                        ForEach(model.turns) { message($0) }
                    }
                    if let notice = model.notice {
                        Text(notice)
                            .font(.system(size: 13))
                            .foregroundColor(.red)
                            .padding(10)
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .background(Color.red.opacity(0.10))
                            .cornerRadius(10)
                    }
                    Color.clear.frame(height: 1).id("bottom")
                }
                .padding(24)
                .frame(maxWidth: 736, alignment: .leading)
                .frame(maxWidth: .infinity)
            }
            // Every new delta moves the anchor: the reply reads as it arrives.
            .onChange(of: model.turns.last?.text) { _, _ in
                proxy.scrollTo("bottom", anchor: .bottom)
            }
            .onChange(of: model.notice) { _, _ in
                proxy.scrollTo("bottom", anchor: .bottom)
            }
        }
    }

    @ViewBuilder
    private func message(_ turn: HermesTurn) -> some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(turn.role == .user ? "You" : "Hermes")
                .font(.system(size: 11, weight: .bold))
                .tracking(0.9)
                .foregroundColor(turn.role == .user ? .indigo : .secondary)

            Text(turn.text.isEmpty ? "…" : turn.text)
                .font(.system(size: 15))
                .foregroundColor(turn.text.isEmpty ? .secondary : .primary)
                .textSelection(.enabled)
                .padding(.horizontal, turn.role == .user ? 14 : 0)
                .padding(.vertical, turn.role == .user ? 11 : 0)
                .frame(maxWidth: .infinity, alignment: .leading)
                .background {
                    if turn.role == .user {
                        RoundedRectangle(cornerRadius: 12)
                            .fill(Color(nsColor: .controlBackgroundColor))
                            .overlay(
                                RoundedRectangle(cornerRadius: 12)
                                    .stroke(Color.primary.opacity(0.07), lineWidth: 1)
                            )
                    }
                }
        }
    }

    private var emptyState: some View {
        VStack(spacing: 14) {
            Text(configured ? "Start a conversation" : "Connect Hermes")
                .font(.system(size: 20, weight: .semibold))
            Text(configured
                 ? "Messages go to your Hermes agent, wherever you installed it. It brings its own tools and memory — this window only shows the conversation."
                 : "Coucou talks to your Hermes agent over its API server. Add the URL and the key in Settings, then come back here.")
                .font(.system(size: 14))
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
            if !configured {
                Button("Open Settings") {
                    NotificationCenter.default.post(name: .openFullSettings, object: nil)
                }
                .buttonStyle(.borderedProminent)
            }
        }
        .padding(.top, 90)
        .frame(maxWidth: .infinity)
    }

    // MARK: - Composer

    private var composer: some View {
        HStack(alignment: .bottom, spacing: 10) {
            TextField("Message Hermes…", text: $draft, axis: .vertical)
                .lineLimit(1...6)
                .textFieldStyle(.plain)
                .focused($composerFocused)
                .onSubmit(send)
                .padding(.horizontal, 14)
                .padding(.vertical, 10)
                .background(Color(nsColor: .controlBackgroundColor))
                .clipShape(RoundedRectangle(cornerRadius: 12))
                .overlay(
                    RoundedRectangle(cornerRadius: 12)
                        .stroke(Color.primary.opacity(0.07), lineWidth: 1)
                )

            Button(action: send) {
                Image(systemName: "arrow.up.circle.fill")
                    .font(.system(size: 26))
            }
            .buttonStyle(.plain)
            .foregroundStyle(canSend ? Color.indigo : Color.secondary)
            .disabled(!canSend)
        }
        .padding(.horizontal, 24)
        .padding(.top, 16)
        .padding(.bottom, 20)
        .background(Color(nsColor: .windowBackgroundColor))
    }

    private var canSend: Bool {
        !model.busy && !draft.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
    }

    private func send() {
        guard canSend else { return }
        let text = draft
        draft = ""
        Task { await model.send(text) }
    }
}
