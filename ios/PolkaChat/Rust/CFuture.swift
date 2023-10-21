//
//  CFuture.swift
//  PolkaChat
//
//  Created by Daniel Leping on 07/03/2023.
//

import Foundation
import TesseractUtils

import CPolkaChat

extension CFuture_CArray_CString: CFuturePtr {
    public static func convert(value: inout [String]) -> CResult<CArray_CString> {
        fatalError("Not implemented: we don't do it anywhere")
    }
    
    public typealias CVal = CArray_CString
    public typealias Val = [String]
    
    public mutating func _onComplete(cb: @escaping (CResult<CVal>) -> Void) -> CResult<CVal>? {
        _withOnCompleteContext(cb) { ctx, value, error in
            self.set_on_complete(&self, ctx, value, error) { ctx, val, err in
                Self._onCompleteCallback(ctx, val, err)
            }
        }
    }
    
    public mutating func _setupSetOnCompleteFunc() {
        self.set_on_complete = { this, ctx, value, error, cb in
            Self._setOnCompleteFunc(this, ctx, value, error) { this, val, err in
                cb?(this, val, err)
            }
        }
    }
}
