import CoreGraphics
import Foundation

/// The games Retold knows, in the engine's numbering.
enum Game: Int32, CaseIterable, Identifiable {
    case red = 1, blue, yellow, gold, silver, crystal, fireRed, leafGreen, ruby, sapphire, emerald

    var id: Int32 { rawValue }

    var name: String {
        switch self {
        case .red: "Red"
        case .blue: "Blue"
        case .yellow: "Yellow"
        case .gold: "Gold"
        case .silver: "Silver"
        case .crystal: "Crystal"
        case .fireRed: "FireRed"
        case .leafGreen: "LeafGreen"
        case .ruby: "Ruby"
        case .sapphire: "Sapphire"
        case .emerald: "Emerald"
        }
    }

    var era: String {
        switch self {
        case .red, .blue, .yellow: "Kanto · Game Boy"
        case .gold, .silver, .crystal: "Johto · Game Boy Color"
        case .fireRed, .leafGreen: "Kanto · Game Boy Advance"
        case .ruby, .sapphire, .emerald: "Hoenn · Game Boy Advance"
        }
    }

    var playable: Bool { retold_game_playable(rawValue) == 1 }
}

/// Looks inside a file (or a zip) for a supported cartridge.
func findCartridge(in data: Data) -> (Game, Data)? {
    var out: UnsafeMutablePointer<UInt8>?
    var outLen = 0
    let number = data.withUnsafeBytes { raw in
        retold_find_cartridge(raw.bindMemory(to: UInt8.self).baseAddress, raw.count, &out, &outLen)
    }
    guard number != 0, let game = Game(rawValue: number), let out else { return nil }
    let cart = Data(bytes: out, count: outLen)
    retold_free_buffer(out, outLen)
    return (game, cart)
}

/// One running game.
final class Session {
    private let handle: OpaquePointer
    let game: Game

    init?(game: Game, cartridge: Data) {
        guard let h = cartridge.withUnsafeBytes({ raw in
            retold_open(raw.bindMemory(to: UInt8.self).baseAddress, raw.count)
        }) else { return nil }
        handle = h
        self.game = game
    }

    deinit { retold_close(handle) }

    func setView(tilesWide: Int, tilesHigh: Int) {
        retold_set_view(handle, UInt32(tilesWide), UInt32(tilesHigh))
    }

    func set(_ key: Int32, _ value: Int) {
        retold_set_option(handle, UInt32(key), Int32(value))
    }

    @discardableResult
    func step(buttons: UInt32) -> Bool { retold_step(handle, buttons) == 1 }

    /// The current picture.
    func image() -> CGImage? {
        var w: UInt32 = 0
        var h: UInt32 = 0
        guard let px = retold_pixels(handle, &w, &h), w > 0, h > 0 else { return nil }
        let data = Data(bytes: px, count: Int(w * h) * 4)
        guard let provider = CGDataProvider(data: data as CFData) else { return nil }
        return CGImage(
            width: Int(w), height: Int(h), bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: Int(w) * 4,
            space: CGColorSpaceCreateDeviceRGB(),
            bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.noneSkipLast.rawValue),
            provider: provider, decode: nil, shouldInterpolate: false, intent: .defaultIntent)
    }

    func save() -> Data {
        let size = retold_save(handle, nil, 0)
        var out = Data(count: size)
        _ = out.withUnsafeMutableBytes { raw in
            retold_save(handle, raw.bindMemory(to: UInt8.self).baseAddress, size)
        }
        return out
    }

    @discardableResult
    func load(_ data: Data) -> Bool {
        data.withUnsafeBytes { raw in
            retold_load(handle, raw.bindMemory(to: UInt8.self).baseAddress, raw.count) == 1
        }
    }
}

/// Button bits, as the engine wants them.
struct Pad: OptionSet {
    let rawValue: UInt32
    static let a = Pad(rawValue: 1)
    static let b = Pad(rawValue: 2)
    static let select = Pad(rawValue: 4)
    static let start = Pad(rawValue: 8)
    static let right = Pad(rawValue: 16)
    static let left = Pad(rawValue: 32)
    static let up = Pad(rawValue: 64)
    static let down = Pad(rawValue: 128)
}
