// BetterCraft: draws the "CRAFT" corner ribbon (a yellow diagonal band across the top-left corner,
// clipped to the icon's rounded square, bold white capitals) on a macOS app icon.
// Usage: swift scripts/badge-icon.swift <in.icns|png> <out-1024.png>
import AppKit

let args = CommandLine.arguments
guard args.count == 3, let src = NSImage(contentsOfFile: args[1]) else {
    FileHandle.standardError.write("usage: badge-icon.swift <in> <out.png>\n".data(using: .utf8)!)
    exit(1)
}
let size: CGFloat = 1024
let rep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: Int(size), pixelsHigh: Int(size), bitsPerSample: 8,
                           samplesPerPixel: 4, hasAlpha: true, isPlanar: false, colorSpaceName: .deviceRGB,
                           bytesPerRow: 0, bitsPerPixel: 0)!
NSGraphicsContext.saveGraphicsState()
let ctx = NSGraphicsContext(bitmapImageRep: rep)!
NSGraphicsContext.current = ctx
ctx.imageInterpolation = .high
src.draw(in: NSRect(x: 0, y: 0, width: size, height: size))

// macOS icons: an 824 px rounded square centred on the 1024 px canvas.
let tile = NSRect(x: 100, y: 100, width: 824, height: 824)
NSBezierPath(roundedRect: tile, xRadius: 185, yRadius: 185).addClip()

// The band runs at 45° across the top-left corner (AppKit's origin is bottom-left).
let reach: CGFloat = 300 // how far from the corner, along each edge, the band's centre line meets it
let centre = NSPoint(x: tile.minX + reach / 2, y: tile.maxY - reach / 2)
let transform = NSAffineTransform()
transform.translateX(by: centre.x, yBy: centre.y)
transform.rotate(byDegrees: 45)
transform.concat()

let band = NSRect(x: -420, y: -62, width: 840, height: 124)
let shadow = NSShadow()
shadow.shadowColor = NSColor.black.withAlphaComponent(0.4)
shadow.shadowBlurRadius = 16
shadow.shadowOffset = NSSize(width: 0, height: -6)
NSGraphicsContext.saveGraphicsState()
shadow.set()
NSColor(srgbRed: 1.0, green: 0.78, blue: 0.0, alpha: 1).setFill()
NSBezierPath(rect: band).fill()
NSGraphicsContext.restoreGraphicsState()
// Thin lighter edges, like a printed ribbon.
NSColor.white.withAlphaComponent(0.35).setFill()
NSBezierPath(rect: NSRect(x: band.minX, y: band.maxY - 8, width: band.width, height: 8)).fill()
NSBezierPath(rect: NSRect(x: band.minX, y: band.minY, width: band.width, height: 8)).fill()

let textShadow = NSShadow()
textShadow.shadowColor = NSColor(srgbRed: 0.55, green: 0.35, blue: 0.0, alpha: 0.55)
textShadow.shadowBlurRadius = 3
textShadow.shadowOffset = NSSize(width: 0, height: -3)
let text = NSAttributedString(string: "CRAFT", attributes: [
    .font: NSFont.systemFont(ofSize: 86, weight: .black),
    .foregroundColor: NSColor.white,
    .kern: 6,
    .shadow: textShadow,
])
let box = text.size()
text.draw(at: NSPoint(x: -box.width / 2 + 3, y: -box.height / 2 + 2))
NSGraphicsContext.restoreGraphicsState()
guard let png = rep.representation(using: .png, properties: [:]) else { exit(1) }
try png.write(to: URL(fileURLWithPath: args[2]))
