//
//  Messages.swift
//  PolkaChat
//
//  Created by Daniel Leping on 02/03/2023.
//

import SwiftUI

struct MessagesView: View {
    let messages: Array<Message>
    
    var body: some View {
        ScrollViewReader { scrollView in
            ScrollView {
                LazyVStack(alignment: .leading) {
                    ForEach(messages) { message in
                        HStack {
                            if case .submitted(id: _, text: _) = message {
                                ProgressView().padding(.trailing, 2)
                            }
                            Text(message.text)
                        }
                        .id(message.id)
                        .padding(6)
                        .padding(.horizontal, 4)
                        .background(Color(red: 0x8F/0xFF,
                                          green: 0xB8/0xFF,
                                          blue: 0xE3/0xFF))
                        .cornerRadius(32)
                        .padding(.vertical, 2)
                    }
                }
                .onChange(of: messages) { messages in
                    withAnimation(.easeInOut(duration: 60)) {
                        scrollView.scrollTo(messages.last?.id)
                    }
                }
                .onAppear {
                    scrollView.scrollTo(messages.last?.id)
                }
            }
        }
    }
}
