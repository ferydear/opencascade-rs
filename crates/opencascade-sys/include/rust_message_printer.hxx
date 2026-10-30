// AstraKernel diagnostic bridge (M5 WP8): a Message_Printer subclass that
// captures OCCT messages (warning and above) into a mutex-protected drain
// buffer. Rust installs it once per process and drains after each engine op
// to populate ValidityReport.warnings. ASCII-only comments (GBK console rule).

#pragma once
#include "rust/cxx.h"
#include <Message_Printer.hxx>
#include <Message_Messenger.hxx>
#include <Message.hxx>
#include <Message_Gravity.hxx>
#include <TCollection_AsciiString.hxx>
#include <TCollection_ExtendedString.hxx>
#include <mutex>
#include <string>
#include <vector>

// Captured messages buffer (process-wide singleton).
struct RustPrinterBuffer {
  std::mutex mutex;
  std::vector<std::string> lines;

  static RustPrinterBuffer& instance() {
    static RustPrinterBuffer buffer;
    return buffer;
  }

  void lines_push(const char* label, const char* text) {
    std::lock_guard<std::mutex> lock(mutex);
    lines.emplace_back(std::string("[") + label + "] " + text);
  }

  void take_all(rust::Vec<rust::String>& out) {
    std::lock_guard<std::mutex> lock(mutex);
    for (const std::string& line : lines) {
      out.push_back(line);
    }
    lines.clear();
  }
};

class RustMessagePrinter : public Message_Printer {
public:
  RustMessagePrinter() { SetTraceLevel(Message_Warning); }

protected:
  // The only pure virtual in OCCT 7.8.1 (the public Send overloads funnel here).
  void send(const TCollection_AsciiString& theString,
            const Message_Gravity theGravity) const override {
    if (theGravity < Message_Warning) {
      return;
    }
    const char* label = "Warning";
    if (theGravity == Message_Alarm) {
      label = "Alarm";
    } else if (theGravity == Message_Fail) {
      label = "Fail";
    }
    RustPrinterBuffer::instance().lines_push(label, theString.ToCString());
  }
};

// Install (idempotent by flag) onto the default messenger.
inline void install_rust_printer() {
  static bool installed = false;
  if (!installed) {
    installed = true;
    Handle(Message_Printer) printer = new RustMessagePrinter();
    Message::DefaultMessenger()->AddPrinter(printer);
  }
}

// Drain all captured messages (Rust takes ownership; buffer empties).
inline rust::Vec<rust::String> take_rust_printer_messages() {
  rust::Vec<rust::String> out;
  RustPrinterBuffer::instance().take_all(out);
  return out;
}

// Test hook: emit via the real default messenger (validates the full path).
inline void emit_test_message(rust::Str theText) {
  const std::string text(theText.data(), theText.size());
  Message::DefaultMessenger()->Send(TCollection_AsciiString(text.c_str()), Message_Warning);
}
