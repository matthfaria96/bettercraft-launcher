// BetterCraft: draws the "Craft" side badge (yellow, white text) on a macOS app icon.
// Usage: swift scripts/badge-icon.swift <in.icns|png> <out-1024.png>
import AppKit

let args = CommandLine.arguments
guard args.count == 3, let src = NSImage(contentsOfFile: args[1]) else {
    FileHandle.standardError.write("usage: badge-icon.swift <in> <out.png>\n".data(using: .utf8)!)
    exit(1)
}
let size = 1024
let rep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: size, pixelsHigh: size, bitsPerSample: 8,
                           samplesPerPixel: 4, hasAlpha: true, isPlanar: false, colorSpaceName: .deviceRGB,
                           bytesPerRow: 0, bitsPerPixel: 0)!
NSGraphicsContext.saveGraphicsState()
let ctx = NSGraphicsContext(bitmapImageRep: rep)!
NSGraphicsContext.current = ctx
ctx.imageInterpolation = .high
src.draw(in: NSRect(x: 0, y: 0, width: size, height: size))

// A pill on the right side, over the lower corner of the icon's rounded square.
let pill = NSRect(x: 560, y: 120, width: 400, height: 168)
let path = NSBezierPath(roundedRect: pill, xRadius: 84, yRadius: 84)
let shadow = NSShadow()
shadow.shadowColor = NSColor.black.withAlphaComponent(0.35)
shadow.shadowBlurRadius = 18
shadow.shadowOffset = NSSize(width: 0, height: -6)
NSGraphicsContext.saveGraphicsState()
shadow.set()
NSColor(srgbRed: 0.98, green: 0.73, blue: 0.0, alpha: 1).setFill()
path.fill()
NSGraphicsContext.restoreGraphicsState()
NSColor.white.withAlphaComponent(0.9).setStroke()
path.lineWidth = 10
path.stroke()

let font = NSFont.systemFont(ofSize: 112, weight: .heavy)
let text = NSAttributedString(string: "Craft", attributes: [
    .font: font,
    .foregroundColor: NSColor.white,
    .kern: -2,
])
let box = text.size()
text.draw(at: NSPoint(x: pill.midX - box.width / 2, y: pill.midY - box.height / 2 + 4))
NSGraphicsContext.restoreGraphicsState()
guard let png = rep.representation(using: .png, properties: [:]) else { exit(1) }
try png.write(to: URL(fileURLWithPath: args[2]))
