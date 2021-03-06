#include "lumen/EIR/Conversion/ComparisonOpConversions.h"

namespace lumen {
namespace eir {

template <typename Op, typename OperandAdaptor>
class ComparisonOpConversion : public EIROpConversion<Op> {
 public:
  explicit ComparisonOpConversion(MLIRContext *context,
                                  EirTypeConverter &converter_,
                                  TargetInfo &targetInfo_,
                                  mlir::PatternBenefit benefit = 1)
      : EIROpConversion<Op>::EIROpConversion(context, converter_, targetInfo_,
                                             benefit) {}

  LogicalResult matchAndRewrite(
      Op op, ArrayRef<Value> operands,
      ConversionPatternRewriter &rewriter) const override {
    OperandAdaptor adaptor(operands);
    auto ctx = getRewriteContext(op, rewriter);

    StringRef builtinSymbol = Op::builtinSymbol();

    auto termTy = ctx.getUsizeType();
    auto int1ty = ctx.getI1Type();

    auto callee =
        ctx.getOrInsertFunction(builtinSymbol, int1ty, {termTy, termTy});

    auto lhs = adaptor.lhs();
    auto rhs = adaptor.rhs();
    ArrayRef<Value> args({lhs, rhs});
    auto calleeSymbol =
        FlatSymbolRefAttr::get(builtinSymbol, callee->getContext());
    Operation *callOp = std_call(calleeSymbol, ArrayRef<Type>{int1ty}, args);

    rewriter.replaceOp(op, callOp->getResult(0));
    return success();
  }

 private:
  using EIROpConversion<Op>::getRewriteContext;
};

struct CmpEqOpConversion
    : public ComparisonOpConversion<CmpEqOp, CmpEqOpAdaptor> {
  using ComparisonOpConversion::ComparisonOpConversion;
};
struct CmpNeqOpConversion
    : public ComparisonOpConversion<CmpNeqOp, CmpNeqOpAdaptor> {
  using ComparisonOpConversion::ComparisonOpConversion;
};
struct CmpLtOpConversion
    : public ComparisonOpConversion<CmpLtOp, CmpLtOpAdaptor> {
  using ComparisonOpConversion::ComparisonOpConversion;
};
struct CmpLteOpConversion
    : public ComparisonOpConversion<CmpLteOp, CmpLteOpAdaptor> {
  using ComparisonOpConversion::ComparisonOpConversion;
};
struct CmpGtOpConversion
    : public ComparisonOpConversion<CmpGtOp, CmpGtOpAdaptor> {
  using ComparisonOpConversion::ComparisonOpConversion;
};
struct CmpGteOpConversion
    : public ComparisonOpConversion<CmpGteOp, CmpGteOpAdaptor> {
  using ComparisonOpConversion::ComparisonOpConversion;
};

void populateComparisonOpConversionPatterns(OwningRewritePatternList &patterns,
                                            MLIRContext *context,
                                            EirTypeConverter &converter,
                                            TargetInfo &targetInfo) {
  patterns.insert<CmpEqOpConversion, CmpNeqOpConversion, CmpLtOpConversion,
                  CmpLteOpConversion, CmpGtOpConversion, CmpGteOpConversion>(
      context, converter, targetInfo);
}

}  // namespace eir
}  // namespace lumen
