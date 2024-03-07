import Foundation

@_cdecl("swift_function_swift")
public func swiftFunction() -> Int32 {
    return 123
}

@_cdecl("swift_string_swift")
public func swiftString() -> [CChar] {
    return "Hello World".cString(using: String.Encoding.utf8)!
}

