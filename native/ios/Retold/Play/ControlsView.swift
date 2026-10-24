import UIKit

/// The on-screen buttons: a D-pad, A, B, Start and Select. Handles several fingers at once,
/// lets a finger slide between A and B, and keeps the D-pad following a finger that drifts off
/// it. Touches anywhere else go through to whatever is underneath.
final class ControlsView: UIView {
    /// Called whenever the set of pressed buttons changes.
    var onChange: ((Pad) -> Void)?
    private(set) var pressed: Pad = []

    var sizeScale: CGFloat = 1 { didSet { setNeedsLayout() } }
    var haptics = true
    var landscape = false { didSet { setNeedsLayout() } }
    /// The part of the view the buttons live in (the whole view in landscape).
    var controlArea: CGRect = .zero { didSet { setNeedsLayout() } }

    private enum Control: Hashable { case dpad, a, b, start, select }

    private let dpad = DPadLayer()
    private let aButton = RoundButton(label: "A")
    private let bButton = RoundButton(label: "B")
    private let startButton = PillButton(label: "START")
    private let selectButton = PillButton(label: "SELECT")
    private var touches: [UITouch: Control] = [:]
    private var dpadDir: Pad = []
    private let tap = UIImpactFeedbackGenerator(style: .light)

    override init(frame: CGRect) {
        super.init(frame: frame)
        isMultipleTouchEnabled = true
        backgroundColor = .clear
        layer.addSublayer(dpad)
        for v in [aButton, bButton, startButton, selectButton] as [UIView] {
            v.isUserInteractionEnabled = false
            addSubview(v)
        }
    }

    required init?(coder: NSCoder) { fatalError() }

    override func layoutSubviews() {
        super.layoutSubviews()
        let area = controlArea.isEmpty ? bounds : controlArea
        let s = sizeScale
        let padR = 72 * s
        let btnR = 34 * s
        let pill = CGSize(width: 66 * s, height: 26 * s)
        let inset = safeAreaInsets
        let padCenter: CGPoint
        let aCenter: CGPoint
        let bCenter: CGPoint
        let startCenter: CGPoint
        let selectCenter: CGPoint
        if landscape {
            let cy = area.maxY - max(inset.bottom, 12) - padR - 40 * s
            padCenter = CGPoint(x: area.minX + max(inset.left, 16) + padR + 8, y: cy)
            let right = area.maxX - max(inset.right, 16) - btnR - 8
            aCenter = CGPoint(x: right, y: cy - 26 * s)
            bCenter = CGPoint(x: right - btnR * 2.3, y: cy + 22 * s)
            let lowY = area.maxY - max(inset.bottom, 10) - pill.height / 2 - 4
            selectCenter = CGPoint(x: padCenter.x, y: lowY)
            startCenter = CGPoint(x: (aCenter.x + bCenter.x) / 2, y: lowY)
        } else {
            let bottom = area.maxY - max(inset.bottom, 16)
            let cy = area.minY + (bottom - area.minY - pill.height - 24) / 2
            padCenter = CGPoint(x: area.minX + max(area.width * 0.25, padR + 20), y: cy)
            aCenter = CGPoint(x: area.maxX - max(area.width * 0.14, btnR + 18), y: cy - 24 * s)
            bCenter = CGPoint(x: aCenter.x - btnR * 2.4, y: cy + 24 * s)
            let lowY = bottom - pill.height / 2 - 6
            selectCenter = CGPoint(x: area.midX - pill.width * 0.65, y: lowY)
            startCenter = CGPoint(x: area.midX + pill.width * 0.65, y: lowY)
        }
        CATransaction.begin()
        CATransaction.setDisableActions(true)
        dpad.frame = CGRect(x: padCenter.x - padR, y: padCenter.y - padR, width: padR * 2, height: padR * 2)
        dpad.setNeedsDisplay()
        CATransaction.commit()
        aButton.frame = CGRect(x: aCenter.x - btnR, y: aCenter.y - btnR, width: btnR * 2, height: btnR * 2)
        bButton.frame = CGRect(x: bCenter.x - btnR, y: bCenter.y - btnR, width: btnR * 2, height: btnR * 2)
        startButton.frame = CGRect(origin: CGPoint(x: startCenter.x - pill.width / 2, y: startCenter.y - pill.height / 2), size: pill)
        selectButton.frame = CGRect(origin: CGPoint(x: selectCenter.x - pill.width / 2, y: selectCenter.y - pill.height / 2), size: pill)
    }

    /// Only the buttons (with a generous margin) take touches.
    override func point(inside point: CGPoint, with event: UIEvent?) -> Bool {
        control(at: point) != nil
    }

    private func control(at p: CGPoint) -> Control? {
        func near(_ f: CGRect, _ grow: CGFloat) -> Bool { f.insetBy(dx: -grow, dy: -grow).contains(p) }
        if near(dpad.frame, 24) { return .dpad }
        // A and B share the space between them, so a thumb resting in the middle hits one.
        let da = hypot(p.x - aButton.center.x, p.y - aButton.center.y)
        let db = hypot(p.x - bButton.center.x, p.y - bButton.center.y)
        let reach = aButton.bounds.width * 0.9
        if min(da, db) < reach { return da <= db ? .a : .b }
        if near(startButton.frame, 12) { return .start }
        if near(selectButton.frame, 12) { return .select }
        return nil
    }

    private func direction(for touch: UITouch) -> Pad {
        let p = touch.location(in: self)
        let c = CGPoint(x: dpad.frame.midX, y: dpad.frame.midY)
        let dx = p.x - c.x
        let dy = p.y - c.y
        if hypot(dx, dy) < dpad.frame.width * 0.12 { return [] }
        if abs(dx) > abs(dy) { return dx > 0 ? .right : .left }
        return dy > 0 ? .down : .up
    }

    override func touchesBegan(_ set: Set<UITouch>, with event: UIEvent?) {
        for t in set {
            if let c = control(at: t.location(in: self)) { touches[t] = c }
        }
        update()
    }

    override func touchesMoved(_ set: Set<UITouch>, with event: UIEvent?) {
        for t in set {
            guard let c = touches[t] else { continue }
            if c == .a || c == .b, let now = control(at: t.location(in: self)), now == .a || now == .b {
                touches[t] = now
            }
        }
        update()
    }

    override func touchesEnded(_ set: Set<UITouch>, with event: UIEvent?) {
        for t in set { touches[t] = nil }
        update()
    }

    override func touchesCancelled(_ set: Set<UITouch>, with event: UIEvent?) {
        touchesEnded(set, with: event)
    }

    /// Lets go of everything (when the game pauses).
    func releaseAll() {
        touches.removeAll()
        update()
    }

    private func update() {
        var now: Pad = []
        var dir: Pad = []
        for (t, c) in touches {
            switch c {
            case .dpad: dir.formUnion(direction(for: t))
            case .a: now.insert(.a)
            case .b: now.insert(.b)
            case .start: now.insert(.start)
            case .select: now.insert(.select)
            }
        }
        now.formUnion(dir)
        guard now != pressed else { return }
        if haptics && !now.subtracting(pressed).isEmpty { tap.impactOccurred(intensity: 0.6) }
        pressed = now
        dpadDir = dir
        dpad.pressed = dir
        aButton.isDown = now.contains(.a)
        bButton.isDown = now.contains(.b)
        startButton.isDown = now.contains(.start)
        selectButton.isDown = now.contains(.select)
        onChange?(now)
    }
}

/// A plus-shaped D-pad drawn in the same see-through style as the buttons.
private final class DPadLayer: CALayer {
    var pressed: Pad = [] { didSet { if pressed != oldValue { setNeedsDisplay() } } }

    override init() {
        super.init()
        contentsScale = UIScreen.main.scale
        needsDisplayOnBoundsChange = true
    }

    override init(layer: Any) { super.init(layer: layer) }
    required init?(coder: NSCoder) { fatalError() }

    override func draw(in ctx: CGContext) {
        let b = bounds
        let arm = b.width * 0.34
        let r = arm * 0.22
        let midX = b.midX - arm / 2
        let midY = b.midY - arm / 2
        let vertical = CGRect(x: midX, y: b.minY + 2, width: arm, height: b.height - 4)
        let horizontal = CGRect(x: b.minX + 2, y: midY, width: b.width - 4, height: arm)
        let shape = CGMutablePath()
        shape.addPath(CGPath(roundedRect: vertical, cornerWidth: r, cornerHeight: r, transform: nil))
        shape.addPath(CGPath(roundedRect: horizontal, cornerWidth: r, cornerHeight: r, transform: nil))
        ctx.addPath(shape)
        ctx.setFillColor(UIColor.white.withAlphaComponent(0.16).cgColor)
        ctx.fillPath(using: .winding)
        let lit: [(Pad, CGRect)] = [
            (.up, CGRect(x: midX, y: b.minY + 2, width: arm, height: midY - b.minY - 2)),
            (.down, CGRect(x: midX, y: midY + arm, width: arm, height: b.maxY - midY - arm - 2)),
            (.left, CGRect(x: b.minX + 2, y: midY, width: midX - b.minX - 2, height: arm)),
            (.right, CGRect(x: midX + arm, y: midY, width: b.maxX - midX - arm - 2, height: arm)),
        ]
        for (dir, rect) in lit where pressed.contains(dir) {
            ctx.setFillColor(UIColor.white.withAlphaComponent(0.32).cgColor)
            ctx.fill(rect)
        }
        ctx.addPath(shape)
        ctx.setStrokeColor(UIColor.white.withAlphaComponent(0.45).cgColor)
        ctx.setLineWidth(1.5)
        ctx.strokePath()
        // Small arrows.
        ctx.setFillColor(UIColor.white.withAlphaComponent(0.7).cgColor)
        let t = arm * 0.16
        let arrows: [(CGPoint, CGFloat)] = [
            (CGPoint(x: b.midX, y: b.minY + arm * 0.55), 0),
            (CGPoint(x: b.midX, y: b.maxY - arm * 0.55), .pi),
            (CGPoint(x: b.minX + arm * 0.55, y: b.midY), -.pi / 2),
            (CGPoint(x: b.maxX - arm * 0.55, y: b.midY), .pi / 2),
        ]
        for (c, angle) in arrows {
            ctx.saveGState()
            ctx.translateBy(x: c.x, y: c.y)
            ctx.rotate(by: angle)
            ctx.move(to: CGPoint(x: 0, y: -t))
            ctx.addLine(to: CGPoint(x: t, y: t * 0.6))
            ctx.addLine(to: CGPoint(x: -t, y: t * 0.6))
            ctx.closePath()
            ctx.fillPath()
            ctx.restoreGState()
        }
    }
}

private class GlassButton: UIView {
    let label = UILabel()
    var isDown = false { didSet { refresh() } }

    init(label text: String, font: UIFont) {
        super.init(frame: .zero)
        label.text = text
        label.font = font
        label.textColor = UIColor.white.withAlphaComponent(0.85)
        label.textAlignment = .center
        addSubview(label)
        layer.borderColor = UIColor.white.withAlphaComponent(0.45).cgColor
        layer.borderWidth = 1.5
        refresh()
    }

    required init?(coder: NSCoder) { fatalError() }

    override func layoutSubviews() {
        super.layoutSubviews()
        label.frame = bounds
        layer.cornerRadius = min(bounds.width, bounds.height) / 2
    }

    private func refresh() {
        backgroundColor = UIColor.white.withAlphaComponent(isDown ? 0.36 : 0.16)
        transform = isDown ? CGAffineTransform(scaleX: 0.94, y: 0.94) : .identity
    }
}

private final class RoundButton: GlassButton {
    init(label: String) { super.init(label: label, font: .systemFont(ofSize: 22, weight: .bold)) }
    required init?(coder: NSCoder) { fatalError() }
}

private final class PillButton: GlassButton {
    init(label: String) { super.init(label: label, font: .systemFont(ofSize: 11, weight: .semibold)) }
    required init?(coder: NSCoder) { fatalError() }
}
