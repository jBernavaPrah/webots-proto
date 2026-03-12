#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateChunk {
    Text { content: String, span: Span },
    ExecutionBlock { content: String, span: Span },
    ExpressionBlock { content: String, span: Span },
}

#[derive(Debug)]
pub enum ParserError {
    UnclosedBlock(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Text,
    InsideBlock,
    InsideExpression,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParserMode {
    Code {
        depth: usize,
        is_interpolation: bool,
    },
    String(u8),
    LineComment,
    BlockComment,
}

pub(crate) fn parse(input: &str) -> Result<Vec<TemplateChunk>, ParserError> {
    let mut chunks = Vec::new();
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut cursor = 0;

    let mut chunk_start = 0;
    let mut state = State::Text;

    while cursor < len {
        match state {
            State::Text => {
                if cursor + 1 < len && bytes[cursor] == b'%' && bytes[cursor + 1] == b'<' {
                    if cursor > chunk_start {
                        let text_content = slice_bytes_to_string(bytes, chunk_start, cursor);
                        chunks.push(TemplateChunk::Text {
                            content: text_content,
                            span: Span::new(chunk_start, cursor),
                        });
                    }

                    cursor += 2;
                    if cursor < len && bytes[cursor] == b'=' {
                        state = State::InsideExpression;
                        cursor += 1;
                        chunk_start = cursor;
                    } else {
                        state = State::InsideBlock;
                        chunk_start = cursor;
                    }
                } else {
                    cursor += 1;
                }
            }
            State::InsideBlock | State::InsideExpression => {
                let mut mode_stack = vec![ParserMode::Code {
                    depth: 0,
                    is_interpolation: false,
                }];
                let block_content_start = chunk_start;
                let mut block_found_end = false;

                while cursor < len {
                    // Check for termination, ONLY if top state is Code(false)
                    // i.e., not partial code inside interpolation, not string, not comment
                    let is_terminatable = matches!(
                        mode_stack.last(),
                        Some(ParserMode::Code {
                            is_interpolation: false,
                            ..
                        })
                    );

                    if is_terminatable
                        && cursor + 1 < len
                        && bytes[cursor] == b'>'
                        && bytes[cursor + 1] == b'%'
                    {
                        let content = slice_bytes_to_string(bytes, block_content_start, cursor);
                        let delimiter_len = if state == State::InsideExpression {
                            3
                        } else {
                            2
                        };
                        let full_span_start = block_content_start - delimiter_len;
                        let full_span_end = cursor + 2;

                        let span = Span::new(full_span_start, full_span_end);

                        if state == State::InsideExpression {
                            chunks.push(TemplateChunk::ExpressionBlock { content, span });
                        } else {
                            chunks.push(TemplateChunk::ExecutionBlock { content, span });
                        }

                        cursor += 2;
                        chunk_start = cursor;
                        state = State::Text;
                        block_found_end = true;
                        break;
                    }

                    // State transition logic
                    let current_mode = *mode_stack.last().unwrap();
                    match current_mode {
                        ParserMode::LineComment => {
                            if bytes[cursor] == b'\n' {
                                mode_stack.pop();
                            }
                            cursor += 1;
                        }
                        ParserMode::BlockComment => {
                            if cursor + 1 < len
                                && bytes[cursor] == b'*'
                                && bytes[cursor + 1] == b'/'
                            {
                                mode_stack.pop();
                                cursor += 2;
                            } else {
                                cursor += 1;
                            }
                        }
                        ParserMode::String(quote) => {
                            if bytes[cursor] == b'\\' {
                                cursor += 2; // skip escaped char
                            } else if bytes[cursor] == quote {
                                mode_stack.pop();
                                cursor += 1;
                            } else if quote == b'`'
                                && cursor + 1 < len
                                && bytes[cursor] == b'$'
                                && bytes[cursor + 1] == b'{'
                            {
                                // Enter interpolation
                                mode_stack.push(ParserMode::Code {
                                    depth: 0,
                                    is_interpolation: true,
                                });
                                cursor += 2; // skip ${
                            } else {
                                cursor += 1;
                            }
                        }
                        ParserMode::Code {
                            depth,
                            is_interpolation,
                        } => {
                            if bytes[cursor] == b'/' {
                                // Check comments
                                if cursor + 1 < len {
                                    if bytes[cursor + 1] == b'/' {
                                        mode_stack.push(ParserMode::LineComment);
                                        cursor += 2;
                                        continue;
                                    } else if bytes[cursor + 1] == b'*' {
                                        mode_stack.push(ParserMode::BlockComment);
                                        cursor += 2;
                                        continue;
                                    }
                                }
                            }

                            if bytes[cursor] == b'"' {
                                mode_stack.push(ParserMode::String(b'"'));
                                cursor += 1;
                            } else if bytes[cursor] == b'\'' {
                                mode_stack.push(ParserMode::String(b'\''));
                                cursor += 1;
                            } else if bytes[cursor] == b'`' {
                                mode_stack.push(ParserMode::String(b'`'));
                                cursor += 1;
                            } else if bytes[cursor] == b'{' {
                                // Increment depth
                                // If we just entered interpolation, depth starts at 0. This { makes it 1.
                                // Wait, if we skipped ${ at invocation, this is the first {?
                                // No, `${` are the tokens that triggered the push.
                                // The brace IS part of the syntax but we consumed it?
                                // `cursor += 2` skips `${`. So the brace is consumed.
                                // So we start inside the Code block.
                                // So depth 0 is correct.
                                // If we see `{`, it's nested brace.
                                if let Some(ParserMode::Code { depth, .. }) = mode_stack.last_mut()
                                {
                                    *depth += 1;
                                }
                                cursor += 1;
                            } else if bytes[cursor] == b'}' {
                                if depth > 0 {
                                    if let Some(ParserMode::Code { depth, .. }) =
                                        mode_stack.last_mut()
                                    {
                                        *depth -= 1;
                                    }
                                    cursor += 1;
                                } else {
                                    // Depth 0.
                                    if is_interpolation {
                                        // Closing the interpolation!
                                        mode_stack.pop(); // Pop Code, return to String context
                                        cursor += 1;
                                    } else {
                                        // Top level } in execution block. Just ignore/consume.
                                        cursor += 1;
                                    }
                                }
                            } else {
                                cursor += 1;
                            }
                        }
                    }
                }

                if !block_found_end {
                    return Err(ParserError::UnclosedBlock(block_content_start));
                }
            }
        }
    }

    // Final text chunk
    if chunk_start < len {
        chunks.push(TemplateChunk::Text {
            content: slice_bytes_to_string(bytes, chunk_start, len),
            span: Span::new(chunk_start, len),
        });
    }

    Ok(chunks)
}

fn slice_bytes_to_string(bytes: &[u8], start: usize, end: usize) -> String {
    String::from_utf8_lossy(&bytes[start..end]).into_owned()
}
