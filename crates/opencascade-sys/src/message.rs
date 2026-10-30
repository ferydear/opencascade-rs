pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/message.hxx");

        type Message_ProgressRange;
        #[cxx_name = "construct_unique"]
        pub fn Message_ProgressRange_new() -> UniquePtr<Message_ProgressRange>;
    }

    unsafe extern "C++" {
        // 诊断桥（M5 WP8）：捕获 OCCT Message（warning 级以上）→ 排空缓冲。
        include!("opencascade-sys/include/rust_message_printer.hxx");

        /// 幂等安装捕获 Printer 到默认 Messenger（进程级一次）。
        pub fn install_rust_printer();

        /// 取走全部已捕获消息（缓冲清空）。
        pub fn take_rust_printer_messages() -> Vec<String>;

        /// 测试钩子：经真实默认 Messenger 发一条 warning（验证全链路）。
        pub fn emit_test_message(text: &str);
    }
}
