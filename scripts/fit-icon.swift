// BetterCraft: places full-bleed icon art (SVG or PNG, its rounded square filling most of the
// canvas) on the macOS icon grid: the art's square becomes 824 px, centred on 1024 px.
// Usage: swift scripts/fit-icon.swift <in.svg|png> <inset-fraction> <out-1024.png>
//   inset-fraction: the margin around the art's square in the source, as a fraction of its width
//   (Crafts_Icons: 24 / 512).
import AppKit

let args = CommandLine.arguments
guard args.count == 4, let src = NSImage(contentsOfFile: args[1]), let inset = Double(args[2]) else {
    FileHandle.standardError.write("usage: fit-icon.swift <in> <inset-fraction> <out.png>\n".data(using: .utf8)!)
    exit(1)
}
let size: CGFloat = 1024
let body: CGFloat = 824
let rep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: Int(size), pixelsHigh: Int(size), bitsPerSample: 8,
                           samplesPerPixel: 4, hasAlpha: true, isPlanar: false, colorSpaceName: .deviceRGB,
                           bytesPerRow: 0, bitsPerPixel: 0)!
NSGraphicsContext.saveGraphicsState()
NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: rep)
NSGraphicsContext.current?.imageInterpolation = .high
// The whole source is drawn so that its inner square lands on the 824 px body.
let drawn = body / CGFloat(1 - 2 * inset)
let origin = (size - drawn) / 2
src.draw(in: NSRect(x: origin, y: origin, width: drawn, height: drawn))
NSGraphicsContext.restoreGraphicsState()
guard let png = rep.representation(using: .png, properties: [:]) else { exit(1) }
try png.write(to: URL(fileURLWithPath: args[3]))
