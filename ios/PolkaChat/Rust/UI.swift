//
//  UI.swift
//  PolkaChat
//
//  Created by Daniel Leping on 07/03/2023.
//

import Foundation

import CPolkaChat

public class UI {
    private let errorModel: ErrorModel
    
    init(errorModel: ErrorModel) {
        self.errorModel = errorModel
    }
    
    func presentError(message: String) {
        Task {
            await self.errorModel.presentError(message: message)
        }
    }
    
    func asRust() -> SUI {
        SUI(ui: self)
    }
}
