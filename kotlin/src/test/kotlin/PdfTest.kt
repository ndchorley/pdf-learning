import com.lowagie.text.pdf.PdfName
import com.lowagie.text.pdf.PdfReader
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class OpenPdfTest {
    @Test
    fun `can find the page tree`() {
        val reader = PdfReader("sample.pdf")

        val pageTree = reader.catalog.getAsDict(PdfName.PAGES)

        val type = pageTree.get(PdfName.TYPE).toString()

        assertEquals("/Pages", type)
    }
}
