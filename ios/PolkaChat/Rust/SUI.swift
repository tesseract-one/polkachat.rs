//
//  SUI.swift
//  PolkaChat
//
//  Created by Daniel Leping on 07/03/2023.
//

import Foundation

import TesseractUtils

import CPolkaChat

extension SUI: CSwiftDropPtr {
    public typealias SObject = UI
}

extension SUI {
    public init(ui: UI) {
        self = SUI(value: ui)
        self.present_error = fn_present_error
    }
}

private func fn_present_error(this: UnsafePointer<SUI>?, message: CStringRef?) -> CResult_Nothing {
    guard let this = try? this?.unowned() else {
        var result = CResult_Nothing()
        result.tag = CResult_Nothing_Err_Nothing
        result.err = CError.nullPtr.copiedPtr()
        return result
    }
    
    guard let message = message?.copied() else {
        var result = CResult_Nothing()
        result.tag = CResult_Nothing_Err_Nothing
        result.err = CError.nullPtr.copiedPtr()
        return result
    }
    
    defer {
        this.presentError(message: message)
    }
    
    var result = CResult_Nothing()
    result.tag = CResult_Nothing_Ok_Nothing
    return result
}
