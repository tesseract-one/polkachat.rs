//
//  Messages.swift
//  PolkaChat
//
//  Created by Daniel Leping on 02/03/2023.
//

import SwiftUI

struct MessagesView: View {
    @State private var messages: Array<String>
    
    init(messages: Array<String>) {
        self.messages = messages
    }
    
    var body: some View {
        LazyVStack(alignment: .leading) {
            ForEach(messages, id: \.self) { message in
                Text(message)
                    .padding(.vertical, 2)
            }
        }
    }
}
