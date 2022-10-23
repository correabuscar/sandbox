#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <limits>
#include <string>
#include <utility>

#include "unicode/uformattedvalue.h"
#include "unicode/unumberformatter.h"
#include "unicode/ustdio.h"
#include "unicode/utypes.h"

#define CHECK_OR_RETURN(name, status) \
    if (U_FAILURE(status)) { printf("%s failed: %s\n", name, u_errorName(status)); return 1; }

class UNumberFormatterDeleter {
 public:
  void operator()(UNumberFormatter* aPtr) {
    unumf_close(aPtr);
  }
};
using UniqueUNumberFormatter = std::unique_ptr<UNumberFormatter, UNumberFormatterDeleter>;

class UFormattedNumberDeleter {
 public:
  void operator()(UFormattedNumber* aPtr) {
    unumf_closeResult(aPtr);
  }
};
using UniqueUFormattedNumber = std::unique_ptr<UFormattedNumber, UFormattedNumberDeleter>;

int main() {
    const char* locale = "en";
    const char16_t* skeleton = u"integer-width/+0 sign-except-zero rounding-mode-half-up";

    UErrorCode status = U_ZERO_ERROR;
    UniqueUNumberFormatter nf(unumf_openForSkeletonAndLocale(
        skeleton, std::char_traits<char16_t>::length(skeleton), locale, &status));
    CHECK_OR_RETURN("unumf_openForSkeletonAndLocale", status);

    UniqueUFormattedNumber formatted(unumf_openResult(&status));
    CHECK_OR_RETURN("unumf_openResult", status);

    double num = std::numeric_limits<double>::infinity();

    unumf_formatDouble(nf.get(), num, formatted.get(), &status);
    CHECK_OR_RETURN("unumf_formatDouble", status);

    const UFormattedValue* formattedValue = unumf_resultAsValue(formatted.get(), &status);
    CHECK_OR_RETURN("unumf_resultAsValue", status);

    //1
    int32_t strLength;
    const char16_t* str = ufmtval_getString(formattedValue, &strLength, &status);
    CHECK_OR_RETURN("ufmtval_getString", status);
    printf("%p\n", str);

    //2
    int32_t strLength2;
    const char16_t* str2 = ufmtval_getString(formattedValue, &strLength2, &status);
    CHECK_OR_RETURN("ufmtval_getString", status);
    printf("%p\n", str2);

    //3
    UniqueUFormattedNumber formatted2(unumf_openResult(&status));
    CHECK_OR_RETURN("unumf_openResult", status);
    double num2 = 12;
    unumf_formatDouble(nf.get(), num2, formatted2.get(), &status);
    CHECK_OR_RETURN("unumf_formatDouble", status);
    const UFormattedValue* formattedValue2 = unumf_resultAsValue(formatted2.get(), &status);
    CHECK_OR_RETURN("unumf_resultAsValue", status);
    int32_t strLength3;
    const char16_t* str3 = ufmtval_getString(formattedValue2, &strLength3, &status);
    CHECK_OR_RETURN("ufmtval_getString", status);
    printf("%p\n", str3);

    //4 overwritt-al
    //these 2 lines will overwrite str and str2 which are apparently just pointers to the contents of 'formatted' var!
    double num3 = 89;
    unumf_formatDouble(nf.get(), num3, formatted.get(), &status);
    // ---
    // so I guess this is irrelevant then, when compiling icu on gentoo:
    // In function 'const UChar* icu::ufmtval_getString(const UFormattedValue*, int32_t*, UErrorCode*)':
    // cc1plus: warning: function may return address of local variable [-Wreturn-local-addr]
    // /var/tmp/portage/dev-libs/icu-68.2/work/icu/source/i18n/formattedvalue.cpp:205:19: note: declared here
    //  205 |     UnicodeString readOnlyAlias = impl->fFormattedValue->toTempString(*ec);
    //      |                   ^~~~~~~~~~~~~

    std::u16string result{str, size_t(strLength)};
    result.push_back('\0');
    std::u16string result2{str2, size_t(strLength2)};
    result2.push_back('\0');
    std::u16string result3{str3, size_t(strLength3)};
    result3.push_back('\0');

    u_printf("result: %S %S %S\n", result.data(), result2.data(), result3.data());

    return 0;
}
