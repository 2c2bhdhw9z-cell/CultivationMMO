import SwiftUI
import UniformTypeIdentifiers

/// The home screen: every game, whether its cartridge is added, and a way to add one.
struct LibraryView: View {
    @ObservedObject private var carts = Cartridges.shared
    @State private var importing = false
    @State private var message: String?
    @State private var playing: Game?
    @State private var showSettings = false
    @State private var removing: Game?

    private let columns = [GridItem(.adaptive(minimum: 160), spacing: 14)]

    var body: some View {
        NavigationStack {
            ScrollView {
                LazyVGrid(columns: columns, spacing: 14) {
                    ForEach(Game.allCases) { game in
                        GameCard(game: game, added: carts.installed.contains(game))
                            .onTapGesture { open(game) }
                            .contextMenu {
                                if carts.installed.contains(game) {
                                    Button("Start over from the beginning", systemImage: "arrow.counterclockwise") {
                                        carts.forgetResume(game)
                                    }
                                    Button("Remove cartridge", systemImage: "trash", role: .destructive) {
                                        removing = game
                                    }
                                }
                            }
                    }
                }
                .padding(16)
                Text("Retold reads everything from your own cartridge file. No game is included.")
                    .font(.footnote)
                    .foregroundStyle(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 32)
                    .padding(.bottom, 24)
            }
            .background(Backdrop())
            .navigationTitle("Retold")
            .toolbar {
                ToolbarItem(placement: .topBarLeading) {
                    Button { showSettings = true } label: { Image(systemName: "gearshape") }
                }
                ToolbarItem(placement: .topBarTrailing) {
                    Button { importing = true } label: { Image(systemName: "plus") }
                }
            }
        }
        .fileImporter(isPresented: $importing, allowedContentTypes: [.item]) { result in
            guard case .success(let url) = result else { return }
            switch carts.add(from: url) {
            case .added(let game):
                if game.playable {
                    playing = game
                } else {
                    message = "Got your \(game.name) cartridge. \(game.name) isn't playable yet, it'll open here once it is."
                }
            case .notACartridge:
                message = "That file isn't a cartridge Retold knows. It needs the US version of one of these games (a .zip is fine)."
            case .failed:
                message = "Couldn't read that file. Try again?"
            }
        }
        .alert(message ?? "", isPresented: Binding(get: { message != nil }, set: { if !$0 { message = nil } })) {
            Button("OK", role: .cancel) {}
        }
        .confirmationDialog(
            "Remove \(removing?.name ?? "") and where you were?",
            isPresented: Binding(get: { removing != nil }, set: { if !$0 { removing = nil } }),
            titleVisibility: .visible
        ) {
            Button("Remove", role: .destructive) {
                if let g = removing { carts.remove(g) }
                removing = nil
            }
        }
        .fullScreenCover(item: $playing) { game in
            PlayView(game: game)
        }
        .sheet(isPresented: $showSettings) { SettingsView() }
    }

    private func open(_ game: Game) {
        if !carts.installed.contains(game) {
            importing = true
        } else if game.playable {
            playing = game
        } else {
            message = "\(game.name) isn't playable yet. Your cartridge is saved, so it'll be ready when it is."
        }
    }
}

private struct GameCard: View {
    let game: Game
    let added: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Spacer(minLength: 0)
            Text(game.name)
                .font(.system(size: 26, weight: .heavy, design: .rounded))
                .foregroundStyle(.white)
                .shadow(color: .black.opacity(0.35), radius: 3, y: 2)
            Text(game.era)
                .font(.caption.weight(.medium))
                .foregroundStyle(.white.opacity(0.85))
            Text(status)
                .font(.caption2.weight(.semibold))
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(.ultraThinMaterial, in: Capsule())
                .foregroundStyle(.white)
        }
        .frame(maxWidth: .infinity, minHeight: 150, alignment: .leading)
        .padding(14)
        .background(
            LinearGradient(colors: game.colors, startPoint: .topLeading, endPoint: .bottomTrailing)
                .opacity(game.playable ? 1 : 0.55)
        )
        .clipShape(RoundedRectangle(cornerRadius: 22, style: .continuous))
        .overlay(
            RoundedRectangle(cornerRadius: 22, style: .continuous)
                .strokeBorder(.white.opacity(0.18), lineWidth: 1)
        )
        .contentShape(RoundedRectangle(cornerRadius: 22))
    }

    private var status: String {
        switch (added, game.playable) {
        case (true, true): "Ready to play"
        case (false, true): "Add your cartridge"
        case (true, false): "Added · coming later"
        case (false, false): "Coming later"
        }
    }
}

private struct Backdrop: View {
    var body: some View {
        LinearGradient(
            colors: [Color(red: 0.08, green: 0.07, blue: 0.12), Color(red: 0.13, green: 0.06, blue: 0.10)],
            startPoint: .top, endPoint: .bottom
        )
        .ignoresSafeArea()
    }
}

extension Game {
    var colors: [Color] {
        func c(_ r: Double, _ g: Double, _ b: Double) -> Color { Color(red: r, green: g, blue: b) }
        switch self {
        case .red: return [c(0.92, 0.22, 0.20), c(0.55, 0.06, 0.10)]
        case .blue: return [c(0.22, 0.45, 0.95), c(0.08, 0.15, 0.50)]
        case .yellow: return [c(0.98, 0.80, 0.18), c(0.85, 0.50, 0.05)]
        case .gold: return [c(0.85, 0.66, 0.25), c(0.50, 0.33, 0.08)]
        case .silver: return [c(0.70, 0.74, 0.80), c(0.32, 0.36, 0.44)]
        case .crystal: return [c(0.35, 0.80, 0.92), c(0.12, 0.36, 0.60)]
        case .fireRed: return [c(0.98, 0.42, 0.16), c(0.66, 0.10, 0.06)]
        case .leafGreen: return [c(0.42, 0.82, 0.30), c(0.10, 0.42, 0.16)]
        case .ruby: return [c(0.86, 0.10, 0.32), c(0.42, 0.02, 0.14)]
        case .sapphire: return [c(0.16, 0.30, 0.86), c(0.04, 0.08, 0.38)]
        case .emerald: return [c(0.12, 0.76, 0.48), c(0.02, 0.36, 0.24)]
        }
    }
}
