//
//  Message.swift
//  PolkaChat
//
//  Created by Daniel Leping on 03/03/2023.
//

import Foundation

enum Message: Identifiable, Hashable, Equatable {
    case commited(id: UUID, text: String)
    case submitted(id: UUID, text: String)
    
    static func newCommited(text: String) -> Self {
        .commited(id: UUID(), text: text)
    }
    
    static func newSubmitted(text: String) -> Self {
        .submitted(id: UUID(), text: text)
    }
    
    func intoCommited() -> Self {
        guard case let .submitted(id, text) = self else {
            fatalError("can't convert into commited already commited message")
        }
        
        return .commited(id: id, text: text)
    }
    
    var id: UUID {
        switch self {
            case .commited(id: let id, text: _): return id
            case .submitted(id: let id, text: _): return id
        }
    }
    
    var text: String {
        switch self {
            case .commited(id: _, text: let text): return text
            case .submitted(id: _, text: let text): return text
        }
    }
}


