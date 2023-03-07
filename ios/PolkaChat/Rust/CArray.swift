//
//  CArray.swift
//  PolkaChat
//
//  Created by Daniel Leping on 07/03/2023.
//

import Foundation
import TesseractUtils

import CPolkaChat

extension CArray_CString: CCopyConvertArrayPtr {
    public static func convert(element: CString?) -> String? {
        element?.copied()
    }

    public mutating func _free() {
        polkachat_carray_cstring_drop(&self)
    }

    public typealias CElement = CString?
    public typealias SElement = String?
}
