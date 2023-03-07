//
//  CFuture.swift
//  PolkaChat
//
//  Created by Daniel Leping on 07/03/2023.
//

import Foundation
import TesseractUtils

import CPolkaChat

extension CFutureValue_CArray_CString: CFutureValueValue {
    public typealias Val = CArray_CString
    
    public static var valueTag: CFutureValue_CArray_CString_Tag {
        CFutureValue_CArray_CString_Value_CArray_CString
    }
    
    public static var errorTag: CFutureValue_CArray_CString_Tag {
        CFutureValue_CArray_CString_Error_CArray_CString
    }
    
    public static var noneTag: CFutureValue_CArray_CString_Tag {
        CFutureValue_CArray_CString_None_CArray_CString
    }
}

extension CFuture_CArray_CString: CFuturePtr {
    public static func convert(value: inout [String?]) -> TesseractUtils.CResult<CArray_CString> {
        fatalError("Not implemented: we don't do it anywhere")
    }
    
    public typealias CVal = CFutureValue_CArray_CString
    public typealias Val = [String?]
    
    public mutating func _onComplete(cb: @escaping (CResult<CVal.Val>) -> Void) -> CVal {
        _withOnCompleteContext(cb) { ctx in
            self.set_on_complete(&self, ctx) { ctx, val, err in
                Self._onCompleteCallback(ctx, val, err)
            }
        }
    }
    
    public mutating func _setupSetOnCompleteFunc() {
        self.set_on_complete = { this, ctx, cb in
            Self._setOnCompleteFunc(this, ctx) { this, val, err in
                cb?(this, val, err)
            }
        }
    }
}
