//! The threads.

pub use drone_cortexm::thr::{init, init_extended};
pub use drone_stm32_map::thr::*;

use drone_cortexm::thr;

thr! {
    /// The thread data.
    thread => pub Thr {};

    /// The thread-local storage.
    local => pub ThrLocal {};

    /// The vector table type.
    vtable => pub Vtable;

    /// A set of thread tokens.
    index => pub Thrs;

    /// Threads initialization token.
    init => pub ThrsInit;

    threads => {
        exceptions => {
            /// All classes of faults.
            pub hard_fault;
            /// System tick timer.
            pub sys_tick;
        };
        interrupts => {
            /// RCC global interrupt.
            5: pub rcc;
            /// EXTI Line 5(to9) interrupt.
            23: pub exti9_5;
        };
    };
}
