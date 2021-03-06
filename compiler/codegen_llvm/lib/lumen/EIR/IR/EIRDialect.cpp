#include "lumen/EIR/IR/EIRDialect.h"

#include "llvm/Support/Format.h"
#include "lumen/EIR/IR/EIRAttributes.h"
#include "lumen/EIR/IR/EIROps.h"
#include "lumen/EIR/IR/EIRTypes.h"
#include "mlir/IR/Builders.h"
#include "mlir/IR/DialectImplementation.h"
#include "mlir/IR/Module.h"
#include "mlir/IR/StandardTypes.h"
#include "mlir/Transforms/InliningUtils.h"

using namespace lumen::eir;

using ::llvm::SmallString;
using ::mlir::Attribute;
using ::mlir::DialectAsmParser;
using ::mlir::DialectAsmPrinter;

/// Create an instance of the EIR dialect, owned by the context.
///
/// This is where EIR types, operations, and attributes are registered.
void eirDialect::initialize() {
  // addInterfaces<EirInlinerInterface>();
  addOperations<
#define GET_OP_LIST
#include "lumen/EIR/IR/EIROps.cpp.inc"
      >();
  addTypes<::lumen::eir::NoneType, TermType, ListType, NumberType,
           ::lumen::eir::IntegerType, AtomType, ::lumen::eir::BooleanType,
           FixnumType, BigIntType, ::lumen::eir::FloatType, NilType, ConsType,
           TupleType, MapType, ClosureType, BinaryType, HeapBinType,
           ProcBinType, BoxType, RefType, PtrType, TraceRefType,
           ReceiveRefType>();

  addAttributes<AtomAttr, APIntAttr, APFloatAttr, BinaryAttr, SeqAttr>();
}

Attribute parseAtomAttr(DialectAsmParser &parser) {
  assert(false && "EIR dialect parsing is not fully implemented");
}

Attribute parseAPIntAttr(DialectAsmParser &parser, Type type) {
  assert(false && "EIR dialect parsing is not fully implemented");
}

Attribute parseAPFloatAttr(DialectAsmParser &parser) {
  assert(false && "EIR dialect parsing is not fully implemented");
}

Attribute parseBinaryAttr(DialectAsmParser &parser, Type type) {
  assert(false && "EIR dialect parsing is not fully implemented");
}

Attribute parseSeqAttr(DialectAsmParser &parser, Type type) {
  assert(false && "EIR dialect parsing is not fully implemented");
}

Attribute eirDialect::parseAttribute(DialectAsmParser &parser,
                                     Type type) const {
  // Parse the kind keyword first.
  StringRef attrKind;
  if (parser.parseKeyword(&attrKind)) return {};

  if (attrKind == AtomAttr::getAttrName()) return parseAtomAttr(parser);
  if (attrKind == APIntAttr::getAttrName()) return parseAPIntAttr(parser, type);
  if (attrKind == APFloatAttr::getAttrName()) return parseAPFloatAttr(parser);
  if (attrKind == BinaryAttr::getAttrName())
    return parseBinaryAttr(parser, type);
  if (attrKind == SeqAttr::getAttrName()) return parseSeqAttr(parser, type);

  parser.emitError(parser.getNameLoc(), "unknown EIR attribute kind: ")
      << attrKind;
  return {};
}

void eirDialect::printAttribute(Attribute attr, DialectAsmPrinter &p) const {
  auto &os = p.getStream();
  if (auto atomAttr = attr.dyn_cast_or_null<AtomAttr>()) {
    os << AtomAttr::getAttrName() << '<';
    os << "{ id = " << atomAttr.getValue();
    auto name = atomAttr.getStringValue();
    if (name.size() > 0) {
      os << ", value = '" << name << "'";
    }
    os << " }>";
    return;
  } else if (auto iAttr = attr.dyn_cast_or_null<APIntAttr>()) {
    os << APIntAttr::getAttrName() << '<';
    os << "{ value = " << iAttr.getValue() << " }>";
    return;
  } else if (auto floatAttr = attr.dyn_cast_or_null<APFloatAttr>()) {
    os << APFloatAttr::getAttrName() << '<';
    os << "{ value = " << floatAttr.getValue().convertToDouble() << " }>";
    return;
  } else if (auto binAttr = attr.dyn_cast_or_null<BinaryAttr>()) {
    os << BinaryAttr::getAttrName();
    os << "<{ value = ";
    if (binAttr.isPrintable()) {
      auto s = binAttr.getValue();
      os << '"' << s << '"';
    } else {
      auto bin = binAttr.getValue();
      auto size = bin.size();
      os << "0x";
      for (char c : bin.bytes()) {
        os << llvm::format_hex_no_prefix(c, 2, true);
      }
    }
    os << " }>";
    return;
  } else if (auto seqAttr = attr.dyn_cast_or_null<SeqAttr>()) {
    os << SeqAttr::getAttrName() << '<';
    os << '[';
    auto count = seqAttr.size();
    if (count > 0) {
      auto elements = seqAttr.getValue();
      bool printSeparator = count > 1;
      for (unsigned i = 0; i < count; i++) {
        p.printAttribute(elements[i]);
        if (i != (count - 1)) os << ", ";
      }
    }
    os << "]>";
    return;
  }
  llvm_unreachable("unhandled EIR type");
}
