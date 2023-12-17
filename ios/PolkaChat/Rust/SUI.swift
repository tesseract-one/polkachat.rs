//
//  SUI.swift
//  PolkaChat
//
//  Created by Daniel Leping on 07/03/2023.
//

import Foundation

import TesseractTransportsClient
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

private func fn_present_error(
    this: UnsafePointer<SUI>?, message: CStringRef?, error: UnsafeMutablePointer<CTesseractShared.CError>!
) -> Bool {
    guard let this = this else {
        error.pointee = InteropError.null(SUI.self).copiedPtr()
        return false
    }
    guard let message = message else {
        error.pointee = InteropError.null(CStringRef.self).copiedPtr()
        return false
    }
    switch this.unowned() {
    case .failure(let err):
        error.pointee = err.copiedPtr()
        return false
    case .success(let this):
        defer { this.presentError(message: message.copied()) }
        return true
    }
}
