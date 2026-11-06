// Binary Operation Algorithms
pub use inner::*;

#[cxx::bridge]
mod inner {
    #[derive(Debug)]
    #[repr(u32)]
    pub enum BOPAlgo_GlueEnum {
        BOPAlgo_GlueOff,
        BOPAlgo_GlueShift,
        BOPAlgo_GlueFull,
    }

    #[derive(Debug)]
    #[repr(u32)]
    pub enum BOPAlgo_Operation {
        BOPAlgo_COMMON,
        BOPAlgo_FUSE,
        BOPAlgo_CUT,
        BOPAlgo_CUT21,
        BOPAlgo_SECTION,
        BOPAlgo_UNKNOWN,
    }

    unsafe extern "C++" {
        include!("opencascade-sys/include/bop_algo.hxx");

        type BOPAlgo_GlueEnum;

        type BOPAlgo_Operation;
        type BOPAlgo_BOP = crate::bop_bop::BOPAlgo_BOP;
    }
}
