#include <Standard_Type.hxx>
#include <Standard_Version.hxx>
#include <bindings_common.hxx>

inline rust::String type_name(const Handle_Standard_Type &handle) { return std::string(handle->Name()); }

// astrakernel M19（QUAL 10.1）：运行时 OCCT 版本（跨版本黄金样本的样本选择依据）
inline rust::String occt_version_string() { return rust::String(OCC_VERSION_COMPLETE); }
