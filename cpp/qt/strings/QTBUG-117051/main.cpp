#include <QtCore>

QString tst_fromUtf8(const std::string& utf8String)
{
    return QString::fromUtf8(utf8String.c_str());
}

QString tst_decoder(const std::string& utf8String)
{
    QStringDecoder toUtf16(QStringDecoder::Utf8);
    return toUtf16(utf8String.c_str());
}

QString tst_decoder_append_full(const std::string &utf8string)
{
    QStringDecoder toUtf16(QStringDecoder::Utf8);
    QString buf(qsizetype(utf8string.size()), Qt::Uninitialized);

    auto *dst = buf.begin();
    auto data = QUtf8StringView(utf8string.data(), qsizetype(utf8string.size()));
    dst = toUtf16.appendToBuffer(dst, data);

    buf.resize(dst - buf.data());
    return buf;
}

template <typename It>
QString tst_decoder_append_piecemeal(It first, It last)
{
    QStringDecoder toUtf16(QStringDecoder::Utf8);
    QString buf(qsizetype(5), Qt::Uninitialized);

    auto *dst = buf.begin();
    while (first != last) {
        auto chunk = QUtf8StringView(first, 1);
        dst = toUtf16.appendToBuffer(dst, chunk);
        ++first;
    }

    buf.resize(dst - buf.data());
    return buf;
}

/* For reference:
 * legal utf-8 byte sequence
 * http://www.unicode.org/versions/Unicode6.0.0/ch03.pdf - page 94
 *
 *  Code Points        1st       2s       3s       4s
 * U+0000..U+007F     00..7F
 * U+0080..U+07FF     C2..DF   80..BF
 * U+0800..U+0FFF     E0       A0..BF   80..BF
 * U+1000..U+CFFF     E1..EC   80..BF   80..BF
 * U+D000..U+D7FF     ED       80..9F   80..BF
 * U+E000..U+FFFF     EE..EF   80..BF   80..BF
 * U+10000..U+3FFFF   F0       90..BF   80..BF   80..BF
 * U+40000..U+FFFFF   F1..F3   80..BF   80..BF   80..BF
 * U+100000..U+10FFFF F4       80..8F   80..BF   80..BF
 */

int main(int argc, char *argv[])
{
    // 1 code unit + illegal sequence + 1 code unit
//    const char illFormed[] = u8"a\xe0\x9f\x80""a";
    const char illFormed[] = u8"abc\xe2\xa0\xbf""cd";

    QString fromUtf8 = tst_fromUtf8(illFormed);
    QString decoder = tst_decoder(illFormed);
    QString decoderAppendFull = tst_decoder_append_full(illFormed);
    // Feeding the data piece meal will produce inconsistent results!
    QString decoderAppendPieceMeal = tst_decoder_append_piecemeal(std::begin(illFormed), std::end(illFormed) - 1);


    QString convertedWithConstructor(illFormed);

    auto check = [&](const QString &str, QString &&msg) {
        if (str != convertedWithConstructor) {
            qDebug() << msg << str << "!=" << convertedWithConstructor;
        } else {
            qDebug() << msg << "OK";
        }
    };

    check(fromUtf8, "fromUtf8");
    check(decoder, "decoder");
    check(decoderAppendFull, "decoderAppendFull");
    check(decoderAppendPieceMeal, "decoderAppendPieceMeal"); // FAIL!


    return 0;
}
