//
//  Core.swift
//  PolkaChat
//
//  Created by Daniel Leping on 03/03/2023.
//

import Foundation
import TesseractClient

import CPolkaChat

final class Core {
    fileprivate var `internal`: CCore
    
    public init(ui: UI) throws {
        self.internal = try CResult<CCore>.wrap { value, error in
            polkachat_ccore_create(ui.asRust(), IPCTransportIOS().asNative(), value, error)
        }.get()
    }
    
    public func account() async throws -> String {
        try await polkachat_ccore_account(self.internal).value
    }
    
    public func messages(from: UInt32) async throws -> [String] {
        try await polkachat_ccore_messages(self.internal, from).value.compactMap {$0}
    }
    
    public func send(message: String) async throws {
        try await polkachat_ccore_send(self.internal, message.copiedPtr()).value
    }
    
    deinit {
        polkachat_ccore_drop(&self.internal)
    }
}


//protocol CoreProtocol {
//    func account()
//}
