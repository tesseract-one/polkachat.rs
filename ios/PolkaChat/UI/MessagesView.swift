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
        GeometryReader { geometry in
            ScrollViewReader { scrollView in
                ScrollView {
                    LazyVStack(alignment: .leading) {
                        ForEach(messages) { message in
                            HStack {
                                if case .submitted(id: _, text: _) = message {
                                    ProgressView()
                                        .progressViewStyle(CircularProgressViewStyle(tint: Color.white))
                                        .padding(.trailing, 2)
                                }
                                Text(message.text)
                                    .foregroundColor(Color.white)
                            }
                            .id(message.id)
                            .padding(6)
                            .padding(.horizontal, 4)
                            .background(Color(red: 0x17/0xFF,
                                              green: 0x61/0xFF,
                                              blue: 0xB0/0xFF))
                            .cornerRadius(32)
                        }
                    }
                    .padding(.top, 8)
                    .onChange(of: messages) { messages in
                        withAnimation(.easeInOut(duration: 60)) {
                            scrollView.scrollTo(messages.last?.id)
                        }
                    }
                    .onChange(of: Int(geometry.size.height)) { _ in
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
}
