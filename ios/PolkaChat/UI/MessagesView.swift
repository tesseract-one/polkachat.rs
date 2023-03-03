//
//  Messages.swift
//  PolkaChat
//
//  Created by Daniel Leping on 02/03/2023.
//

import SwiftUI

struct MessagesView: View {
    let messages: Array<String>
    
    var body: some View {
        ScrollViewReader { scrollView in
            ScrollView {
                LazyVStack(alignment: .leading) {
                    ForEach(messages, id: \.self) { message in
                        HStack {
                            if message.contains("0") {
                                ProgressView().padding(.trailing, 2)
                            }
                            Text(message)
                        }
                        .id(message)
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
                        scrollView.scrollTo(messages.last)
                    }
                }
                .onAppear {
                    scrollView.scrollTo(messages.last)
                }
            }
        }
    }
}
