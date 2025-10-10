package dev.gobley.test.jninioperfcomparison

import com.sun.jna.*;
import java.io.File
import java.nio.Buffer
import java.nio.ByteBuffer

object RustLibrary {
    // This value can be configured in the Gradle script.
    private const val RESOURCE_PREFIX = "jvm"
    private const val LIBRARY_NAME = "jni_nio_perf_comparison"

    init {
        val mappedLibraryName = System.mapLibraryName(LIBRARY_NAME)

        // Extract the library file to a temporary location as in JNA so this works even when packaged as a .jar file.
        val isWindows = System.getProperty("os.name").startsWith("Windows")
        val librarySuffix = ".dll".takeIf { isWindows }
        val libraryFile = File.createTempFile(LIBRARY_NAME, librarySuffix)

        RustLibrary::class.java.classLoader!!
            .getResourceAsStream("$RESOURCE_PREFIX/$mappedLibraryName")!!
            .use { inputStream ->
                libraryFile.outputStream().use { outputStream ->
                    inputStream.copyTo(outputStream)
                }
            }

        @Suppress("UnsafeDynamicallyLoadedCode")
        Runtime.getRuntime().load(libraryFile.absolutePath)
    }

    @JvmStatic
    external fun initJni()

    fun testUsingNio(
        buffer: ByteBuffer,
        a: Double,
        b: Int,
        c: Double,
        d: Int,
        pos: Int
    ): Double {
        buffer.clear()
        buffer.putDouble(pos, a)
        buffer.putInt(pos + 8, b)
        buffer.putDouble(pos + 16, c)
        buffer.putInt(pos + 24, d)
        testUsingNio()
        return buffer.getDouble(pos)
    }

    @JvmStatic
    external fun testUsingJni(a: Double, b: Int, c: Double, d: Int): Double

    @JvmStatic
    external fun testUsingNio()

    @JvmStatic
    external fun getStackBuffer(): ByteBuffer
}

object RustLibraryJna {
    private const val RESOURCE_PREFIX = "jvm"
    private const val LIBRARY_NAME = "jni_nio_perf_comparison"

    @JvmStatic
    external fun testUsingJnaNio()

    init {
        val mappedLibraryName = System.mapLibraryName(LIBRARY_NAME)

        // Extract the library file to a temporary location as in JNA so this works even when packaged as a .jar file.
        val isWindows = System.getProperty("os.name").startsWith("Windows")
        val librarySuffix = ".dll".takeIf { isWindows }
        val libraryFile = File.createTempFile(LIBRARY_NAME, librarySuffix)

        RustLibrary::class.java.classLoader!!
            .getResourceAsStream("$RESOURCE_PREFIX/$mappedLibraryName")!!
            .use { inputStream ->
                libraryFile.outputStream().use { outputStream ->
                    inputStream.copyTo(outputStream)
                }
            }

        Native.register(libraryFile.absolutePath)
    }

    fun testUsingNio(
        buffer: ByteBuffer,
        a: Double,
        b: Int,
        c: Double,
        d: Int,
        pos: Int
    ): Double {
        buffer.clear()
        buffer.putDouble(pos, a)
        buffer.putInt(pos + 8, b)
        buffer.putDouble(pos + 16, c)
        buffer.putInt(pos + 24, d)
        testUsingJnaNio()
        return buffer.getDouble(pos)
    }

    @JvmStatic
    external fun jnaNioGetStackPointer(): Pointer
}
